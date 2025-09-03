use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct DbiStreamHeader {
    pub version_signature: u32,
    pub version_header: u32,
    pub age: u32,
    pub global_stream_index: u16,
    pub build_number: u16,
    pub public_stream_index: u16,
    pub pdb_dll_version: u16,
    pub sym_record_stream: u16,
    pub pdb_dll_rbld: u16,
    pub mod_info_size: u32,
    pub section_contribution_size: u32,
    pub section_map_size: u32,
    pub source_info_size: u32,
    pub type_server_map_size: u32,
    pub mfc_type_server_index: u32,
    pub optional_dbg_header_size: u32,
    pub ec_substream_size: u32,
    pub flags: u16,
    pub machine: u16,
    pub padding: u32,
}

impl DbiStreamHeader {
    pub fn parse<R: Read + Seek>(mut reader: R) -> std::io::Result<Self> {
        let version_signature = reader.read_u32::<LittleEndian>()?;
        let version_header = reader.read_u32::<LittleEndian>()?;
        let age = reader.read_u32::<LittleEndian>()?;
        let global_stream_index = reader.read_u16::<LittleEndian>()?;
        let build_number = reader.read_u16::<LittleEndian>()?;
        let public_stream_index = reader.read_u16::<LittleEndian>()?;
        let pdb_dll_version = reader.read_u16::<LittleEndian>()?;
        let sym_record_stream = reader.read_u16::<LittleEndian>()?;
        let pdb_dll_rbld = reader.read_u16::<LittleEndian>()?;
        let mod_info_size = reader.read_u32::<LittleEndian>()?;
        let section_contribution_size = reader.read_u32::<LittleEndian>()?;
        let section_map_size = reader.read_u32::<LittleEndian>()?;
        let source_info_size = reader.read_u32::<LittleEndian>()?;
        let type_server_map_size = reader.read_u32::<LittleEndian>()?;
        let mfc_type_server_index = reader.read_u32::<LittleEndian>()?;
        let optional_dbg_header_size = reader.read_u32::<LittleEndian>()?;
        let ec_substream_size = reader.read_u32::<LittleEndian>()?;
        let flags = reader.read_u16::<LittleEndian>()?;
        let machine = reader.read_u16::<LittleEndian>()?;
        let padding = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            version_signature,
            version_header,
            age,
            global_stream_index,
            build_number,
            public_stream_index,
            pdb_dll_version,
            sym_record_stream,
            pdb_dll_rbld,
            mod_info_size,
            section_contribution_size,
            section_map_size,
            source_info_size,
            type_server_map_size,
            mfc_type_server_index,
            optional_dbg_header_size,
            ec_substream_size,
            flags,
            machine,
            padding,
        })
    }
}
