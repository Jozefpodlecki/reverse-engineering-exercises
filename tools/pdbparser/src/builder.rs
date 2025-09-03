use std::{fs::File, io::{BufWriter, Cursor, Read, Seek, SeekFrom, Write}, path::Path};

use anyhow::Result;
use byteorder::{WriteBytesExt, LittleEndian};

pub struct Superblock {
    signature: Vec<u8>,
    block_size: u32,
    free_block_map_block: u32,
    num_blocks: u32,
    num_directory_bytes: u32,
    unknown: u32,
    block_map_addr: u32
}

pub struct StreamDirectory {
    num_streams: u32,
    stream_sizes: Vec<u32>,
    stream_blocks: Vec<Vec<u32>>
}

pub struct PdbData {
    superblock: Superblock,
    stream_directory: StreamDirectory
}

pub enum FreeBlockMapOp<'a> {
    Bytes(&'a [u8]),
    Repeat(u8, usize),
    FillRest(u8),
}


impl PdbData {
    pub fn new() -> Self {
        let signature = b"Microsoft C/C++ MSF 7.00\r\n\x1ADS\0\0\0".to_vec();

        let superblock = Superblock {
            signature,
            block_size: 0x1000,
            free_block_map_block: 1,
            num_blocks: 0x147,
            num_directory_bytes: 0x674,
            unknown: 0,
            block_map_addr: 0x0146,
        };

        let stream_directory = StreamDirectory { 
            num_streams: 5,
            stream_sizes: vec![],
            stream_blocks: vec![]
        };

        Self {
            superblock,
            stream_directory
        }
    }
}

pub struct PdbBuilder<W: Write + Seek> {
    data: PdbData,
    writer: W,
}

impl<W: Write + Seek> PdbBuilder<W> {
    pub fn new(writer: W, data: PdbData) -> Self {
        Self { writer, data }
    }

    pub fn write_header(&mut self) -> Result<()> {
        self.writer.write_all(&self.data.superblock.signature)?;
        println!("{}", self.data.superblock.signature.len());
        self.writer.write_u32::<LittleEndian>(self.data.superblock.block_size)?;
        self.writer.write_u32::<LittleEndian>(self.data.superblock.free_block_map_block)?;
        self.writer.write_u32::<LittleEndian>(self.data.superblock.num_blocks)?;
        self.writer.write_u32::<LittleEndian>(self.data.superblock.num_directory_bytes)?;
        self.writer.write_u32::<LittleEndian>(self.data.superblock.unknown)?;
        self.writer.write_u32::<LittleEndian>(self.data.superblock.block_map_addr)?;
        // 32 + 24 = 56 0x2C

        // pad remaining
        let zero_buf = [0u8; 4096 - 56];
        self.writer.write_all(&zero_buf)?;
        
        self.write_free_block_map(&[
            FreeBlockMapOp::Bytes(&[0xF8, 0x04]),
            FreeBlockMapOp::Repeat(0x00, 38),
            FreeBlockMapOp::Bytes(&[0x80]),
            FreeBlockMapOp::FillRest(0xFF),
        ])?;

        self.write_free_block_map(&[
            FreeBlockMapOp::Bytes(&[0x00, 0xFB]),
            FreeBlockMapOp::FillRest(0xFF),
        ])?;

        let zero_buf = [0u8; 4096];
        self.writer.write_all(&zero_buf)?;

        Ok(())
    }

    pub fn write_free_block_map(&mut self, ops: &[FreeBlockMapOp]) -> Result<()> {
        let mut written = 0;

        for op in ops {
            match op {
                FreeBlockMapOp::Bytes(bytes) => {
                    self.writer.write_all(bytes)?;
                    written += bytes.len();
                }
                FreeBlockMapOp::Repeat(byte, count) => {
                    let buf = vec![*byte; *count];
                    self.writer.write_all(&buf)?;
                    written += count;
                }
                FreeBlockMapOp::FillRest(byte) => {
                    if written < self.data.superblock.block_size as usize {
                        let buf = vec![*byte; self.data.superblock.block_size as usize - written];
                        self.writer.write_all(&buf)?;
                        written = self.data.superblock.block_size as usize;
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn create_pdb<P: AsRef<Path>>(path: P) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    let data = PdbData::new();
    let mut pdb = PdbBuilder::new(writer, data);
    pdb.write_header()?;

    Ok(())
}