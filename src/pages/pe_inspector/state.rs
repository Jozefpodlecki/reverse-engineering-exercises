use std::{ops::Deref, rc::Rc};

use pelite::pe64::{Pe, PeFile, imports::Import};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::pages::pe_inspector::types::*;

#[derive(Default, Clone, PartialEq)]
pub enum InspectorUiState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct RcBytes(pub Rc<[u8]>);

impl Default for RcBytes {
    fn default() -> Self {
        Self(Rc::from([]))
    }
}

impl Deref for RcBytes {
    type Target = [u8];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<u8>> for RcBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(Rc::from(bytes))
    }
}

impl From<&[u8]> for RcBytes {
    fn from(bytes: &[u8]) -> Self {
        Self(Rc::from(bytes))
    }
}

impl Serialize for RcBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for RcBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        Ok(RcBytes(Rc::from(bytes)))
    }
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PeState {
    pub last_modified: f64,
    pub file_name: String,
    pub size: u64,
    pub data: RcBytes,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedPe {
    pub dos_header: DosHeader,
    pub file_header: FileHeader,
    pub optional_header: OptionalHeader,
    pub sections: Vec<Section>,
    pub imports: Vec<ImportModule>,
    pub exports: Vec<Export>,
    pub relocations: Vec<Relocation>,
    pub exception_handlers: Vec<ExceptionHandler>,
}

impl ParsedPe {
    pub fn from_bytes<'a>(data: &'a [u8]) -> Result<Self, String> {
        
        match PeFile::from_bytes(data) {
            Ok(file) => {
                let parsed = ParsedPe::from_pelite(file).map_err(|err| err.to_string())?;
                Ok(parsed)
            },
            Err(err) => {
                let error_msg = match err {
                    pelite::Error::Null => "PE file is null or empty".to_string(),
                    pelite::Error::Bounds => "PE file has invalid bounds or truncated data".to_string(),
                    pelite::Error::ZeroFill => "PE file has zero fill beyond file size".to_string(),
                    pelite::Error::Unmapped => "PE file has unmapped sections or invalid RVA".to_string(),
                    pelite::Error::Misaligned => "PE file has misaligned section or header data".to_string(),
                    pelite::Error::BadMagic => "Invalid DOS magic - missing MZ header".to_string(),
                    pelite::Error::PeMagic => "Invalid PE magic - missing PE signature".to_string(),
                    pelite::Error::Insanity => "PE file contains insane or corrupted values".to_string(),
                    pelite::Error::Invalid => "PE file has invalid structure or parameters".to_string(),
                    pelite::Error::Overflow => "PE file has arithmetic overflow in calculations".to_string(),
                    pelite::Error::Encoding => "PE file has invalid string encoding".to_string(),
                    pelite::Error::Aliasing => "PE file has aliasing or overlapping sections".to_string(),
                };

                Err(error_msg)
            },
        }
    }

    pub fn from_pelite(file: PeFile) -> Result<Self, String> {
        
        let dos_header = Self::extract_dos_header(&file);
        let file_header = Self::extract_file_header(&file);
        let optional_header = Self::extract_optional_header(&file);
        let sections = Self::extract_sections(&file);
        let imports = Self::extract_imports(&file);
        let exports = Self::extract_exports(&file);
        let relocations = Self::extract_relocations(&file);
        let exception_handlers = Self::extract_exception_handlers(&file);

        Ok(Self {
            dos_header,
            file_header,
            optional_header,
            sections,
            imports,
            exports,
            relocations,
            exception_handlers,
        })
    }

    fn extract_dos_header(file: &PeFile) -> DosHeader {
        let dos = file.dos_header();
        DosHeader {
            magic: dos.e_magic,
            e_lfanew: dos.e_lfanew,
        }
    }

    fn extract_file_header(file: &PeFile) -> FileHeader {
        let nt = file.nt_headers();
        let file_header = nt.FileHeader;
        
        FileHeader {
            machine: file_header.Machine,
            number_of_sections: file_header.NumberOfSections,
            time_date_stamp: file_header.TimeDateStamp,
            pointer_to_symbol_table: file_header.PointerToSymbolTable,
            number_of_symbols: file_header.NumberOfSymbols,
            size_of_optional_header: file_header.SizeOfOptionalHeader,
            characteristics: file_header.Characteristics,
        }
    }

    fn extract_optional_header(file: &PeFile) -> OptionalHeader {
        let optional = file.optional_header();
        
        OptionalHeader {
            magic: optional.Magic,
            major_linker_version: optional.LinkerVersion.Major,
            minor_linker_version: optional.LinkerVersion.Minor,
            size_of_code: optional.SizeOfCode,
            size_of_initialized_data: optional.SizeOfInitializedData,
            size_of_uninitialized_data: optional.SizeOfUninitializedData,
            address_of_entry_point: optional.AddressOfEntryPoint,
            base_of_code: optional.BaseOfCode,
            image_base: optional.ImageBase,
            section_alignment: optional.SectionAlignment,
            file_alignment: optional.FileAlignment,
            major_os_version: optional.OperatingSystemVersion.Major,
            minor_os_version: optional.OperatingSystemVersion.Minor,
            major_image_version: optional.ImageVersion.Major,
            minor_image_version: optional.ImageVersion.Minor,
            major_subsystem_version: optional.SubsystemVersion.Major,
            minor_subsystem_version: optional.SubsystemVersion.Minor,
            win32_version_value: optional.Win32VersionValue,
            size_of_image: optional.SizeOfImage,
            size_of_headers: optional.SizeOfHeaders,
            checksum: optional.CheckSum,
            subsystem: optional.Subsystem,
            dll_characteristics: optional.DllCharacteristics,
            size_of_stack_reserve: optional.SizeOfStackReserve,
            size_of_stack_commit: optional.SizeOfStackCommit,
            size_of_heap_reserve: optional.SizeOfHeapReserve,
            size_of_heap_commit: optional.SizeOfHeapCommit,
            loader_flags: optional.LoaderFlags,
            number_of_rva_and_sizes: optional.NumberOfRvaAndSizes,
        }
    }

    fn extract_sections(file: &PeFile) -> Vec<Section> {
        let image_base = file.optional_header().ImageBase;

        file.section_headers()
            .iter()
            .map(|section| Section {
                name: match section.name() {
                    Ok(name) => name.to_string(),
                    Err(err) => hex::encode_upper(section.name_bytes()),
                },
                virtual_size: section.VirtualSize,
                virtual_address_rva: section.VirtualAddress,
                virtual_address_va: image_base + section.VirtualAddress as u64,
                size_of_raw_data: section.SizeOfRawData,
                pointer_to_raw_data: section.PointerToRawData,
                pointer_to_relocations: section.PointerToRelocations,
                pointer_to_linenumbers: section.PointerToLinenumbers,
                number_of_relocations: section.NumberOfRelocations,
                number_of_linenumbers: section.NumberOfLinenumbers,
                characteristics: section.Characteristics,
            })
            .collect()
    }

    fn extract_imports(file: &PeFile) -> Vec<ImportModule> {
        let mut modules = Vec::new();
        
        let imports = match file.imports() {
            Ok(imports) => imports,
            Err(_) => return modules,
        };

        for desc in imports.iter() {
            let dll_name = match desc.dll_name() {
                Ok(name) => name.to_string(),
                Err(_) => continue,
            };
            
            let int = match desc.int() {
                Ok(int) => int,
                Err(_) => continue,
            };
            
            let mut functions = Vec::new();
            
            for import_result in int {
                let import = match import_result {
                    Ok(import) => import,
                    Err(_) => continue,
                };
                
                let function = match import {
                    Import::ByName { hint, name } => ImportFunction {
                        name: Some(name.to_string()),
                        hint: hint as u16,
                        rva: 0,
                    },
                    Import::ByOrdinal { ord } => ImportFunction {
                        name: None,
                        hint: 0,
                        rva: ord as u32,
                    },
                };
                
                functions.push(function);
            }
            
            modules.push(ImportModule {
                name: dll_name,
                functions,
            });
        }

        modules
    }

    fn extract_exports(file: &PeFile) -> Vec<Export> {
        let mut exports = Vec::new();
        
        let export_dir = match file.exports() {
            Ok(dir) => dir,
            Err(_) => return exports,
        };
        
        let by = match export_dir.by() {
            Ok(by) => by,
            Err(_) => return exports,
        };
        
        for (name_result, index) in by.iter_name_indices() {
            let name = match name_result {
                Ok(name) => name.to_string(),
                Err(_) => continue,
            };
            
            let export_result = by.index(index);
            let export = match export_result {
                Ok(exp) => exp,
                Err(_) => continue,
            };
            
            if let Some(rva) = export.symbol() {
                let ordinal = (index as u32 + by.ordinal_base() as u32) as u16;
                
                exports.push(Export {
                    name,
                    ordinal,
                    rva,
                });
            }
        }
        
        exports
    }

    fn extract_relocations(file: &PeFile) -> Vec<Relocation> {
        let mut relocations = Vec::new();
        
        let reloc_dir = match file.base_relocs() {
            Ok(dir) => dir,
            Err(_) => return relocations,
        };
        
        for block in reloc_dir.iter_blocks() {
            let mut entries = Vec::new();
            
            for word in block.words() {
                let typ = block.type_of(word);
                if typ != pelite::image::IMAGE_REL_BASED_ABSOLUTE {
                    let rva = block.rva_of(word);
                    entries.push(RelocationEntry {
                        rva,
                        typ: typ.into(),
                    });
                }
            }
            
            relocations.push(Relocation {
                virtual_address: block.image().VirtualAddress,
                size_of_block: block.image().SizeOfBlock,
                entries,
            });
        }
        
        relocations
    }

    fn extract_exception_handlers(file: &PeFile) -> Vec<ExceptionHandler> {
        let mut handlers = Vec::new();
        
        if let Ok(exception_dir) = file.exception() {
            for entry in exception_dir.functions() {
                let entry = entry.image();
                handlers.push(ExceptionHandler {
                    begin_address: entry.BeginAddress,
                    end_address: entry.EndAddress,
                    unwind_address: entry.UnwindData,
                });
            }
        }
        
        handlers
    }
}