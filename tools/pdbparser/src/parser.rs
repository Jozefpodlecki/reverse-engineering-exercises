use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::{dpi::DbiStreamHeader, module_info::{ModInfo, ModuleInfoSubstream, SectionContribEntry}};

const PDB_SIGNATURE: &[u8] = b"Microsoft C/C++ MSF 7.00\r\n\x1ADS\0\0\0";

#[derive(Debug)]
struct SuperBlock {
    block_size: u32,
    free_block_map_block: u32,
    num_blocks: u32,
    num_directory_bytes: u32,
    block_map_addr: u32,
}

#[derive(Debug)]
pub struct PdbParser {
    file: File,
    superblock: SuperBlock,
    stream_sizes: Vec<u32>,
    stream_blocks: Vec<Vec<u32>>,
}

impl PdbParser {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;

        let mut sig = vec![0u8; PDB_SIGNATURE.len()];
        file.read_exact(&mut sig)?;
        if sig != PDB_SIGNATURE {
            anyhow::bail!("Not a valid PDB (bad signature)");
        }

        let block_size = file.read_u32::<LittleEndian>()?;
        let free_block_map_block = file.read_u32::<LittleEndian>()?;
        let num_blocks = file.read_u32::<LittleEndian>()?;
        let num_directory_bytes = file.read_u32::<LittleEndian>()?;
        let _unknown = file.read_u32::<LittleEndian>()?;
        let block_map_addr = file.read_u32::<LittleEndian>()?;

        let superblock = SuperBlock {
            block_size,
            free_block_map_block,
            num_blocks,
            num_directory_bytes,
            block_map_addr,
        };

        file.seek(SeekFrom::Start(
            (block_map_addr as u64) * (block_size as u64),
        ))?;

        let num_dir_blocks =
            (num_directory_bytes + block_size - 1) / block_size;

        let mut dir_block_indices = Vec::new();
        for _ in 0..num_dir_blocks {
            dir_block_indices.push(file.read_u32::<LittleEndian>()?);
        }

        let mut dir_data = Vec::new();
        for &block_idx in &dir_block_indices {
            let pos = (block_idx as u64) * (block_size as u64);
            file.seek(SeekFrom::Start(pos))?;

            let mut buf = vec![0u8; block_size as usize];
            file.read_exact(&mut buf)?;
            dir_data.extend_from_slice(&buf);
        }

        let mut cursor = std::io::Cursor::new(dir_data);

        let num_streams = cursor.read_u32::<LittleEndian>()?;
        let mut stream_sizes = Vec::new();
        for _ in 0..num_streams {
            stream_sizes.push(cursor.read_u32::<LittleEndian>()?);
        }

        let mut stream_blocks = Vec::new();
        for &size in &stream_sizes {
            if size == u32::MAX {
                stream_blocks.push(vec![]);
                continue;
            }
            let blocks_needed = (size + block_size - 1) / block_size;
            let mut blocks = Vec::new();
            for _ in 0..blocks_needed {
                blocks.push(cursor.read_u32::<LittleEndian>()?);
            }
            stream_blocks.push(blocks);
        }

        Ok(Self {
            file,
            superblock,
            stream_sizes,
            stream_blocks,
        })
    }

    pub fn parse_dbi_stream_old(&mut self) -> std::io::Result<DbiStreamHeader> {
        let dbi_index = 3; // DBI stream index
        let blocks = &self.stream_blocks[dbi_index];
        let block_size = self.superblock.block_size as u64;

        let pos = (blocks[0] as u64) * block_size;
        self.file.seek(SeekFrom::Start(pos))?;

        let mut buf = vec![0u8; block_size as usize];
        self.file.read_exact(&mut buf)?;

        let cursor = std::io::Cursor::new(buf);
        DbiStreamHeader::parse(cursor)
    }

    fn read_stream(&mut self, index: usize) -> Result<Vec<u8>> {
        let size = self.stream_sizes[index] as usize;
        let blocks = &self.stream_blocks[index];
        let mut buf = Vec::with_capacity(size);

        for &block_idx in blocks {
            let pos = block_idx as u64 * self.superblock.block_size as u64;
            self.file.seek(SeekFrom::Start(pos))?;
            let mut tmp = vec![0u8; self.superblock.block_size as usize];
            self.file.read_exact(&mut tmp)?;
            buf.extend_from_slice(&tmp);
        }

        buf.truncate(size);
        Ok(buf)
    }

    
    pub fn parse_dbi_stream(&mut self) -> Result<(DbiStreamHeader, Vec<ModInfo>)> {
        let dbi_index = 3; // DBI stream
        let stream_data = self.read_stream(dbi_index)?;

        let mut cursor = std::io::Cursor::new(&stream_data);

        // Parse header
        let header = DbiStreamHeader {
            version_signature: cursor.read_u32::<LittleEndian>()?,
            version_header: cursor.read_u32::<LittleEndian>()?,
            age: cursor.read_u32::<LittleEndian>()?,
            global_stream_index: cursor.read_u16::<LittleEndian>()?,
            build_number: cursor.read_u16::<LittleEndian>()?,
            public_stream_index: cursor.read_u16::<LittleEndian>()?,
            pdb_dll_version: cursor.read_u16::<LittleEndian>()?,
            sym_record_stream: cursor.read_u16::<LittleEndian>()?,
            pdb_dll_rbld: cursor.read_u16::<LittleEndian>()?,
            mod_info_size: cursor.read_u32::<LittleEndian>()?,
            section_contribution_size: cursor.read_u32::<LittleEndian>()?,
            section_map_size: cursor.read_u32::<LittleEndian>()?,
            source_info_size: cursor.read_u32::<LittleEndian>()?,
            type_server_map_size: cursor.read_u32::<LittleEndian>()?,
            mfc_type_server_index: cursor.read_u32::<LittleEndian>()?,
            optional_dbg_header_size: cursor.read_u32::<LittleEndian>()?,
            ec_substream_size: cursor.read_u32::<LittleEndian>()?,
            flags: cursor.read_u16::<LittleEndian>()?,
            machine: cursor.read_u16::<LittleEndian>()?,
            padding: cursor.read_u32::<LittleEndian>()?,
        };

        // Parse Module Info Substream
        let mut modules = Vec::new();
        let start_pos = cursor.position() as usize;
        let end_pos = start_pos + header.mod_info_size as usize;
        cursor.set_position(start_pos as u64);

        while (cursor.position() as usize) < end_pos {
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

            // Read null-terminated module name
            let module_name = {
                let mut name_bytes = Vec::new();
                while let Ok(b) = cursor.read_u8() {
                    if b == 0 { break; }
                    name_bytes.push(b);
                }
                String::from_utf8_lossy(&name_bytes).to_string()
            };

            // Read null-terminated obj file name
            let obj_file_name = {
                let mut name_bytes = Vec::new();
                while let Ok(b) = cursor.read_u8() {
                    if b == 0 { break; }
                    name_bytes.push(b);
                }
                String::from_utf8_lossy(&name_bytes).to_string()
            };

            modules.push(ModInfo {
                unused1,
                section_contrib: SectionContribEntry {
                    section,
                    offset,
                    size,
                    characteristics,
                    module_index,
                    data_crc,
                    reloc_crc,
                },
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
        }

        Ok((header, modules))
    }

    // pub fn parse_module_info(&mut self) -> std::io::Result<ModuleInfoSubstream> {
    //     let dbi_index = 3;
    //     let blocks = &self.stream_blocks[dbi_index];
    //     let block_size = self.superblock.block_size as usize;
        
    //     let mut remaining = size as usize;
    //     let mut stream_data = Vec::with_capacity(self.stream_sizes[dbi_index] as usize);

    //     for &block_idx in blocks {
    //         let pos = block_idx as u64 * self.superblock.block_size as u64;
    //         self.file.seek(std::io::SeekFrom::Start(pos))?;
    //         let mut buf = vec![0u8; block_size];
    //         self.file.read_exact(&mut buf)?;
    //         stream_data.extend_from_slice(&buf);
    //         remaining -= to_read;
    //     }

    //     stream_data.truncate(self.stream_sizes[dbi_index] as usize);

    //     ModuleInfoSubstream::parse(&stream_data, 0x2C)
    // }

    pub fn get_names(&mut self) -> Result<Vec<String>> {
        let stream_index = 1;
        let size = self.stream_sizes[stream_index];
        let blocks = &self.stream_blocks[stream_index];

        let mut buf = Vec::with_capacity(size as usize);
        for &block_idx in blocks {
            let pos = (block_idx as u64) * (self.superblock.block_size as u64);
            self.file.seek(SeekFrom::Start(pos))?;

            let mut tmp = vec![0u8; self.superblock.block_size as usize];
            self.file.read_exact(&mut tmp)?;
            buf.extend_from_slice(&tmp);
        }
        buf.truncate(size as usize);

        // instead of string make enum Named/Unnamed maybe
        // crude string parsing
        let mut strings = Vec::new();
        let mut cur = Vec::new();
        for &b in &buf {
            if b == 0 {
                if !cur.is_empty() {
                    strings.push(String::from_utf8_lossy(&cur).to_string());
                    cur.clear();
                }
            } else {
                cur.push(b);
            }
        }

        Ok(strings)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_pdb() {
        let path = r"C:\repos\reverse-engineering-exercises\exercises\windivert\target\release\windivert.pdb".into();
        let mut parser = PdbParser::new(path).unwrap();

        // println!("{:?}", parser.get_names().unwrap());
        // println!("{:?}", parser.parse_dbi_stream().unwrap());
        println!("{:?}", parser.parse_dbi_stream().unwrap().1.iter().map(|pr| pr.module_name.clone()).collect::<Vec<_>>());
    }
}