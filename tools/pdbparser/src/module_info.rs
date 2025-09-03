use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct SectionContribEntry {
    pub section: u16,
    pub offset: i32,
    pub size: i32,
    pub characteristics: u32,
    pub module_index: u16,
    pub data_crc: u32,
    pub reloc_crc: u32,
}

#[derive(Debug)]
pub struct ModInfo {
    pub unused1: u32,
    pub section_contrib: SectionContribEntry,
    pub flags: u16,
    pub module_sym_stream: u16,
    pub sym_byte_size: u32,
    pub c11_byte_size: u32,
    pub c13_byte_size: u32,
    pub source_file_count: u16,
    pub unused2: u32,
    pub source_file_name_index: u32,
    pub pdb_file_path_name_index: u32,
    pub module_name: String,
    pub obj_file_name: String,
}

#[derive(Debug)]
pub struct ModuleInfoSubstream {
    pub modules: Vec<ModInfo>,
}

impl ModuleInfoSubstream {
    pub fn parse(data: &[u8], header_size: usize) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(&data[header_size..]);
        let mut modules = Vec::new();

        while (cursor.position() as usize) < data.len() {
            let unused1 = cursor.read_u32::<LittleEndian>()?;

            let section = cursor.read_u16::<LittleEndian>()?;
            let _padding1 = cursor.read_u16::<LittleEndian>()?;
            let offset = cursor.read_i32::<LittleEndian>()?;
            let size = cursor.read_i32::<LittleEndian>()?;
            let characteristics = cursor.read_u32::<LittleEndian>()?;
            let module_index = cursor.read_u16::<LittleEndian>()?;
            let _padding2 = cursor.read_u16::<LittleEndian>()?;
            let data_crc = cursor.read_u32::<LittleEndian>()?;
            let reloc_crc = cursor.read_u32::<LittleEndian>()?;

            let section_contrib = SectionContribEntry {
                section,
                offset,
                size,
                characteristics,
                module_index,
                data_crc,
                reloc_crc,
            };

            let flags = cursor.read_u16::<LittleEndian>()?;
            let module_sym_stream = cursor.read_u16::<LittleEndian>()?;
            let sym_byte_size = cursor.read_u32::<LittleEndian>()?;
            let c11_byte_size = cursor.read_u32::<LittleEndian>()?;
            let c13_byte_size = cursor.read_u32::<LittleEndian>()?;
            let source_file_count = cursor.read_u16::<LittleEndian>()?;
            let _padding = cursor.read_u16::<LittleEndian>()?;
            let unused2 = cursor.read_u32::<LittleEndian>()?;
            let source_file_name_index = cursor.read_u32::<LittleEndian>()?;
            let pdb_file_path_name_index = cursor.read_u32::<LittleEndian>()?;

            // read null-terminated module name
            let mut module_name = Vec::new();
            while let Ok(b) = cursor.read_u8() {
                if b == 0 { break; }
                module_name.push(b);
            }

            let module_name = String::from_utf8_lossy(&module_name).to_string();

            // read null-terminated obj file name
            let mut obj_file_name = Vec::new();
            while let Ok(b) = cursor.read_u8() {
                if b == 0 { break; }
                obj_file_name.push(b);
            }

            let obj_file_name = String::from_utf8_lossy(&obj_file_name).to_string();

            modules.push(ModInfo {
                unused1,
                section_contrib,
                flags,
                module_sym_stream,
                sym_byte_size,
                c11_byte_size,
                c13_byte_size,
                source_file_count,
                unused2,
                source_file_name_index,
                pdb_file_path_name_index,
                module_name,
                obj_file_name,
            });

            if cursor.position() as usize >= data.len() {
                break;
            }
        }

        Ok(Self { modules })
    }
}
