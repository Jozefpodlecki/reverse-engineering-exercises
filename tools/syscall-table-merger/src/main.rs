use std::path::PathBuf;
use crate::{index::SyscallIndex};

mod models;
mod index;
mod query;
mod schema;
mod directory;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
 

    let index_path = PathBuf::from("./syscalls-index");
    let tar_path = PathBuf::from("./syscalls-index.tar");
    let base_dir = PathBuf::from(r"C:\repos\reverse_projects\rust_playground\syscall-table-merger\data");

    // SyscallIndex::clean(&index_path, &tar_path)?;
    let index = SyscallIndex::get_or_create(&base_dir, &index_path, &tar_path)?;

    let versions = vec![
        "Windows NT 3.x (3.1)",
        "Windows NT 3.x (3.5)",
        "Windows NT 3.x (3.51)",
        // "Windows 10 (1803)"
    ];

    let builder = index.query()
        .names(["NtAccessCheckByType"])
        .os("windows")
        .arch("x64")
        // .category("nt")
        .versions(versions);

    println!("{}", builder.build_query_string());
    let results = builder.execute()?;

    for result in results {
        println!("{:?}", result);
    }

    Ok(())
}