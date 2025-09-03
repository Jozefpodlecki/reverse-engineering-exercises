
use std::{fs::File, io::Read};

use anyhow::Result;
use clap::Parser;
use pdb::FallibleIterator;

use crate::utils::*;

mod parser;
mod builder;
mod utils;
mod dpi;
mod module_info;

fn main() -> Result<()> {

    // let path = r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\test.pdb.txt";
    // export_pdb(path)?;

    let path = r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\test.pdb.hex.txt";
    let data = {
        let mut file = File::open(r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\windivert.pdb")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        buffer
    };
    
    let path = r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\windivert.pdb.hex.txt";
    export_pdb_to_hex_file(data, path,16, None)?;

    // let data = create_pdb()?;
    // export_pdb_to_hex_stdout(data, 16, None)?;

    // let current_exe = std::env::current_exe()?;
    
    // export_to_hex()
    // let file = File::open(r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\windivert.pdb")?;

    // create_pdb()?;

    Ok(())
}

