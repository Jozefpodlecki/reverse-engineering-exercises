use serde::{Deserialize, Serialize};


#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DosHeader {
    pub magic: u16,
    pub e_lfanew: u32,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionalHeader {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_os_version: u16,
    pub minor_os_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportModule {
    pub name: String,
    pub functions: Vec<ImportFunction>,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportFunction {
    pub name: Option<String>,
    pub hint: u16,
    pub rva: u32,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Export {
    pub name: String,
    pub ordinal: u16,
    pub rva: u32,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relocation {
    pub virtual_address: u32,
    pub size_of_block: u32,
    pub entries: Vec<RelocationEntry>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocationEntry {
    pub rva: u32,
    pub typ: RelocationType,
}

impl From<u8> for RelocationType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Absolute,
            1 => Self::High,
            2 => Self::Low,
            3 => Self::HighLow,
            4 => Self::HighAdj,
            5 => Self::MipsJmpAddr,
            6 => Self::Reserved,
            7 => Self::MipsJmpAddr16,
            9 => Self::Dir64,
            v => Self::Unknown(v),
        }
    }
}

impl RelocationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Absolute => "ABSOLUTE",
            Self::High => "HIGH",
            Self::Low => "LOW",
            Self::HighLow => "HIGHLOW",
            Self::HighAdj => "HIGHADJ",
            Self::MipsJmpAddr => "MIPS_JMPADDR",
            Self::Reserved => "RESERVED",
            Self::MipsJmpAddr16 => "MIPS_JMPADDR16",
            Self::Dir64 => "DIR64",
            Self::Unknown(_) => "UNKNOWN",
        }
    }
    
    pub fn value(&self) -> u8 {
        match self {
            Self::Absolute => 0,
            Self::High => 1,
            Self::Low => 2,
            Self::HighLow => 3,
            Self::HighAdj => 4,
            Self::MipsJmpAddr => 5,
            Self::Reserved => 6,
            Self::MipsJmpAddr16 => 7,
            Self::Dir64 => 9,
            Self::Unknown(v) => *v,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelocationType {
    Absolute,
    High,
    Low,
    HighLow,
    HighAdj,
    MipsJmpAddr,
    Reserved,
    MipsJmpAddr16,
    Dir64,
    Unknown(u8),
}


#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionHandler {
    pub begin_address: u32,
    pub end_address: u32,
    pub unwind_address: u32,
}