use std::{collections::{HashMap, HashSet}, fs::{self, File}, io::{BufRead, BufReader}, path::{Path, PathBuf}};
use anyhow::Result;
use tantivy::{Index, IndexWriter, TantivyDocument, schema::{Field, Schema}};

use crate::{directory::TarDirectory, query::QueryBuilder, schema::{SchemaFields, create_schema}, utils::{get_versions_from_csv, tar_directory}};

pub struct SyscallIndex {
    schema: Schema,
    pub fields: SchemaFields,
    index: Index
}

impl SyscallIndex {

    pub fn inner(&self) -> &Index {
        &self.index
    }

    pub fn get_or_create(base_dir: &Path, index_path: &Path, tar_path: &Path) -> Result<Self> {
        if tar_path.exists() {
            Self::from_tar(tar_path)
        }
        else {
            Self::create_tar(base_dir, index_path, tar_path)
        }
    }

    pub fn from_dir(index_path: &Path) -> Result<Self> {
        let index = Index::open_in_dir(index_path)?;
        let schema = create_schema();
        let fields = SchemaFields::from_schema(&schema);

        Ok(Self {
            index,
            schema,
            fields
        })
    }

    pub fn from_tar(tar_path: &Path) -> Result<Self> {
        let tar_directory = TarDirectory::new(&tar_path)?;
        let index = Index::open(tar_directory)?;
        let schema = create_schema();
        let fields = SchemaFields::from_schema(&schema);

        Ok(Self {
            index,
            schema,
            fields
        })
    }

    pub fn create_tar(base_dir: &Path, index_path: &Path, tar_path: &Path) -> Result<Self> {
     
        Self::create(base_dir, index_path)?;
        tar_directory(&index_path, &tar_path)?;
        fs::remove_dir_all(index_path)?;

        Self::from_tar(tar_path)
    }

    pub fn create(base_dir: &Path, index_path: &Path) -> Result<Self> {
        fs::create_dir_all(index_path)?;
        
        let schema = create_schema();
        let fields = SchemaFields::from_schema(&schema);
        let index = Index::create_in_dir(index_path, schema.clone())?;
        
        let csv_files = [
            (base_dir.join("nt.csv"), "windows", "nt", "x64"),
            (base_dir.join("win32k.csv"), "windows", "win32k", "x64"),
            (base_dir.join("x86-nt.csv"), "windows", "nt", "x86"),
            (base_dir.join("x86-win32k.csv"), "windows", "win32k", "x86"),
            (base_dir.join("linux-x86_64.csv"), "linux", "syscall", "x64"),
            (base_dir.join("linux-x86.csv"), "linux", "syscall", "x86"),
            (base_dir.join("macos-bsd-x86_64.csv"), "macos", "bsd", "x64"),
        ];

        let mut versions_by_os: HashMap<String, HashSet<String>> = HashMap::new();

        for (csv_file, os_name, _, _) in &csv_files {
            if csv_file.exists() {
                let versions = get_versions_from_csv(csv_file)?;
                let os_versions = versions_by_os.entry(os_name.to_string()).or_insert_with(HashSet::new);
                for v in versions {
                    os_versions.insert(v);
                }
            }
        }

        let mut writer = index.writer(50_000_000)?;

        for (csv_file, os_name, category, arch) in csv_files {
            let all_versions = versions_by_os.get(os_name).unwrap();
            Self::index_csv_file(&mut writer, &schema, &all_versions, os_name, &csv_file, arch, category)?;
        }

        writer.commit()?;

        Ok(Self {
            index,
            schema,
            fields
        })
    }

    pub fn clean(index_path: &Path, tar_path: &Path) -> Result<()> {
        
        if index_path.exists() {
            fs::remove_dir_all(index_path)?;
        }

        if tar_path.exists() {
            fs::remove_file(tar_path)?;
        }

        Ok(())
    }

    fn index_csv_file(
        writer: &mut IndexWriter,
        schema: &Schema,
        all_versions: &HashSet<String>,
        os_name: &str,
        path: &Path,
        arch: &str,
        category: &str,
    ) -> Result<()> {
        let csv_versions = get_versions_from_csv(path)?;
        let csv_versions_set: HashSet<String> = csv_versions.iter().cloned().collect();

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        lines.next();
        
        let name_field = schema.get_field("name").unwrap();
        let os_field = schema.get_field("os").unwrap();
        let arch_field = schema.get_field("arch").unwrap();
        let category_field = schema.get_field("category").unwrap();
        let version_field = schema.get_field("version").unwrap();
        let number_field = schema.get_field("number").unwrap();
        
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.is_empty() { continue; }
            
            let syscall_name = parts[0].trim().to_string();
            let versions = parts.iter().skip(1).enumerate();

            for (idx, &num_str) in versions {
                // if idx >= versions.len() { break; }
                let num_str = num_str.trim();
                
                let version = &csv_versions[idx];
                let number = (!num_str.is_empty()).then(|| u64::from_str_radix(num_str.trim_start_matches("0x"), 16).ok()).flatten();
                
                let mut document = TantivyDocument::new();
                document.add_text(name_field, &syscall_name);
                document.add_text(os_field, os_name);
                document.add_text(arch_field, arch);
                document.add_text(category_field, category);
                document.add_text(version_field, version);
                
                if let Some(number) = number {
                    document.add_u64(number_field, number);
                }
                
                writer.add_document(document)?;
            }

            for version in all_versions.difference(&csv_versions_set) {
                let mut document = TantivyDocument::new();
                document.add_text(name_field, &syscall_name);
                document.add_text(os_field, os_name);
                document.add_text(arch_field, arch);
                document.add_text(category_field, category);
                document.add_text(version_field, version);

                writer.add_document(document)?;
            }
        }
        
        Ok(())
    }

    pub fn num_docs(&self) -> Result<u64> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        Ok(searcher.num_docs())
    }

    pub fn num_segments(&self) -> Result<usize> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        Ok(searcher.segment_readers().len())
    }

    pub fn get_distinct_values(&self, field: Field) -> Result<Vec<String>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        let mut all_values = Vec::new();
        
        for segment_reader in searcher.segment_readers() {
            let inverted_index = segment_reader.inverted_index(field)?;
            let dict = inverted_index.terms();
            let mut stream = dict.stream()?;
            
            while let Some((term_bytes, _term_info)) = stream.next() {
                let text = std::str::from_utf8(term_bytes)?.to_string();
                if !all_values.contains(&text) {
                    all_values.push(text);
                }
            }
        }
        
        all_values.sort();
        Ok(all_values)
    }

    pub fn query(&self) -> QueryBuilder {
        QueryBuilder::new(self)
    }
}