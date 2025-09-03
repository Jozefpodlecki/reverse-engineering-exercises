use std::{fs::File, io::{BufWriter, Cursor, Read, Seek, SeekFrom, Write}, path::Path};

use anyhow::Result;
use byteorder::{WriteBytesExt, LittleEndian};
use pdb::FallibleIterator;

use crate::builder::{PdbBuilder, PdbData};


pub fn export_pdb_to_hex<W: Write>(
    data: Vec<u8>,
    dest: &mut W,
    block_size: usize,
    row_size: usize,
    row_limit: Option<usize>) -> Result<()> {

    let mut rows_written = 0;
    for chunk in data.chunks(row_size) {
        if let Some(limit) = row_limit {
            if rows_written >= limit {
                break;
            }
        }

        let block_index = (rows_written * row_size) / block_size;
        write!(dest, "{:>4} ", block_index)?;
        
        for b in chunk {
            write!(dest, "{:02X} ", b)?;
        }
        
        writeln!(dest)?;
        rows_written += 1;
    }

    Ok(())
}

pub fn export_pdb<P: AsRef<Path>>(path: P) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let data = PdbData::new();
    let mut pdb = PdbBuilder::new(&mut writer, data);
    pdb.write_header()?;

    Ok(())
}

pub fn create_pdb() -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    let data = {
        let data = PdbData::new();
        let mut pdb = PdbBuilder::new(&mut buffer, data);
        pdb.write_header()?;
        buffer.into_inner()
    };
    
    Ok(data)
}

pub fn export_pdb_to_hex_file<P: AsRef<Path>>(
    data: Vec<u8>,
    path: P,
    row_size: usize,
    row_limit: Option<usize>,
) -> Result<()> {

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    export_pdb_to_hex(data, &mut writer, 0x1000, row_size, row_limit)
}


pub fn export_pdb_to_hex_stdout(data: Vec<u8>, row_size: usize, row_limit: Option<usize>) -> Result<()> {

    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    export_pdb_to_hex(data, &mut writer, 0x1000, row_size, row_limit)
}


fn read_pdb() -> Result<()> {
    let file = File::open(r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\windivert.pdb")?;
    let mut pdb = pdb::PDB::open(file)?;

    let test = pdb.string_table()?;
    
    // pdb.named_stream(name)
    
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

#[cfg(test)]
mod tests {
    use crate::utils::export_pdb_to_hex_stdout;

    use super::*;

    #[test]
    fn test_export_pdb() {
        // export_pdb_to_hex_stdout(16, Some(10)).unwrap();
    }
}