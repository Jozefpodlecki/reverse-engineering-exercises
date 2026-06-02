use tantivy::{Index, TantivyDocument};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Value;
use anyhow::Result;
use crate::index::SyscallIndex;
use crate::schema::SchemaFields;

#[derive(Debug, Clone)]
pub struct SyscallResult {
    pub name: String,
    pub os: String,
    pub arch: String,
    pub category: String,
    pub version: String,
    pub number: Option<u64>,
}


pub struct QueryBuilder<'a> {
    index: &'a SyscallIndex,
    names: Option<Vec<String>>,
    os: Option<String>,
    arch: Option<String>,
    category: Option<String>,
    versions: Option<Vec<String>>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(index: &'a SyscallIndex) -> Self {
        Self {
            index,
            names: None,
            os: None,
            arch: None,
            category: None,
            versions: None,
        }
    }

    pub fn names(mut self, names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.names = Some(names.into_iter().map(|v| v.into()).collect());
        self
    }
    
    pub fn os(mut self, os: impl Into<String>) -> Self {
        self.os = Some(os.into());
        self
    }
    
    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.arch = Some(arch.into());
        self
    }
    
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
    
    pub fn versions(mut self, versions: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.versions = Some(versions.into_iter().map(|v| v.into()).collect());
        self
    }
    
    pub fn build_query_string(&self) -> String {
        let mut query_parts = Vec::new();
        
        if let Some(names) = &self.names {
            if names.len() == 1 {
                query_parts.push(format!("name:{}", names[0]));
            } else {
                let name_clause: Vec<String> = names.iter()
                    .map(|n| format!("{}", n))
                    .collect();
                query_parts.push(format!("name:({})", name_clause.join(" ")));
            }
        }
        
        if let Some(o) = &self.os {
            query_parts.push(format!("os:{}", o));
        }
        
        if let Some(a) = &self.arch {
            query_parts.push(format!("arch:{}", a));
        }
        
        if let Some(c) = &self.category {
            query_parts.push(format!("category:{}", c));
        }
        
        if let Some(vs) = &self.versions {
            if vs.len() == 1 {
                query_parts.push(format!("version:\"{}\"", vs[0]));
            } else {
                let version_clause: Vec<String> = vs.iter()
                    .map(|v| format!("\"{}\"", v))
                    .collect();
                query_parts.push(format!("version:({})", version_clause.join(" ")));
            }
        }
        
        if query_parts.is_empty() {
            "*".to_string()
        } else {
            query_parts.join(" AND ")
        }
    }
    
    pub fn execute(&self) -> Result<Vec<SyscallResult>> {
        let fields = &self.index.fields;
        let index = self.index.inner();
        let reader = index.reader()?;
        let searcher = reader.searcher();
        let query_str = self.build_query_string();
        
        let query_parser = QueryParser::for_index(
            index,
            vec![fields.name, fields.os, fields.arch, fields.category, fields.version],
        );
        
        let query = query_parser.parse_query(&query_str)?;
        let collector = TopDocs::with_limit(10000).order_by_score();
        let top_docs = searcher.search(&query, &collector)?;
        
        let mut results = Vec::new();
        
        for (_score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            
            let name = doc.get_first(fields.name).map(|f| f.as_str().unwrap()).unwrap_or("").to_string();
            let os = doc.get_first(fields.os).map(|f| f.as_str().unwrap()).unwrap_or("").to_string();
            let arch = doc.get_first(fields.arch).map(|f| f.as_str().unwrap()).unwrap_or("").to_string();
            let category = doc.get_first(fields.category).map(|f| f.as_str().unwrap()).unwrap_or("").to_string();
            let mut version = doc.get_first(fields.version).map(|f| f.as_str().unwrap()).unwrap_or("").to_string();
            let number = doc.get_first(fields.number).map(|f| f.as_u64()).flatten();
            
            if self.arch.is_none() {
                version = format!("{} {}", version, arch);
            }

            results.push(SyscallResult {
                name,
                os,
                arch,
                category,
                version,
                number,
            });
        }
        
        Ok(results)
    }
}