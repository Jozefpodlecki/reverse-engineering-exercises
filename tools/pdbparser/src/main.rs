use std::{fs::File, io::{BufWriter, Seek, Write}};

use anyhow::Result;
use clap::Parser;
use pdb::FallibleIterator;

fn main() -> Result<()> {
    create_pdb()?;

    Ok(())
}

fn create_pdb() -> Result<()> {
    let file_path = r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\test.pdb";
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(b"");

    writer.seek(std::io::SeekFrom::Start(0))?;
    
    let file = File::open(file_path)?;
    let mut pdb = pdb::PDB::open(file)?;

    Ok(())
}

fn read_pdb() -> Result<()> {
    let file = File::open(r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\windivert.pdb")?;
    let mut pdb = pdb::PDB::open(file)?;

    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;

    let mut symbols = symbol_table.iter();
    while let Some(symbol) = symbols.next()? {
        let symbol_data = symbol.parse()?;

        match symbol_data {
            pdb::SymbolData::Public(data) if data.function => {
                        // we found the location of a function!
                        let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                        println!("{} is {}", rva, data.name);
                    }
            pdb::SymbolData::ScopeEnd => todo!(),
            pdb::SymbolData::ObjName(symbol) => todo!(),
            pdb::SymbolData::RegisterVariable(symbol) => todo!(),
            pdb::SymbolData::Constant(symbol) => {
                println!("{:?}", symbol);
            },
            pdb::SymbolData::UserDefinedType(symbol) => {
                println!("{:?}", symbol);
            },
            pdb::SymbolData::MultiRegisterVariable(symbol) => todo!(),
            pdb::SymbolData::Data(symbol) => {
                println!("{:?}", symbol);
            },
            pdb::SymbolData::Procedure(procedure_symbol) => todo!(),
            pdb::SymbolData::ThreadStorage(thread_storage_symbol) => todo!(),
            pdb::SymbolData::CompileFlags(compile_flags_symbol) => todo!(),
            pdb::SymbolData::UsingNamespace(using_namespace_symbol) => todo!(),
            pdb::SymbolData::ProcedureReference(symbol) => {
                println!("{:?}", symbol);
            },
            pdb::SymbolData::DataReference(data_reference_symbol) => todo!(),
            pdb::SymbolData::AnnotationReference(annotation_reference_symbol) => todo!(),
            pdb::SymbolData::Trampoline(trampoline_symbol) => todo!(),
            pdb::SymbolData::Export(export_symbol) => todo!(),
            pdb::SymbolData::Local(local_symbol) => todo!(),
            pdb::SymbolData::BuildInfo(build_info_symbol) => todo!(),
            pdb::SymbolData::InlineSite(inline_site_symbol) => todo!(),
            pdb::SymbolData::InlineSiteEnd => todo!(),
            pdb::SymbolData::ProcedureEnd => todo!(),
            pdb::SymbolData::Label(label_symbol) => todo!(),
            pdb::SymbolData::Block(block_symbol) => todo!(),
            pdb::SymbolData::RegisterRelative(register_relative_symbol) => todo!(),
            pdb::SymbolData::Thunk(thunk_symbol) => todo!(),
            pdb::SymbolData::SeparatedCode(separated_code_symbol) => todo!(),
            symbol => {
                println!("Default {:?}", symbol);
            },
        }
    }

    Ok(())
}
