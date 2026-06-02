use anyhow::Result;
use tar::Builder;
use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn parse_hex(hex_str: &str) -> Option<u32> {
    if hex_str.is_empty() {
        return None;
    }
    u32::from_str_radix(hex_str.trim_start_matches("0x"), 16).ok()
}

pub fn get_versions_from_csv(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    let header = lines.next().unwrap()?;
    let versions: Vec<String> = header
        .split(',')
        .skip(1)
        .map(|s| s.trim().to_string())
        .collect();
    
    Ok(versions)
}

pub fn tar_directory(dir_path: &Path, output_path: &Path) -> Result<()> {
    let tar_file = File::create(output_path)?;
    let mut tar_builder = Builder::new(tar_file);
    tar_builder.append_dir_all("", dir_path)?;
    tar_builder.finish()?;
    Ok(())
}