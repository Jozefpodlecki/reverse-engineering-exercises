use std::fmt::{self, Display, Formatter};
use anyhow::Result;
use pelite::pe::Pe;

#[derive(Debug, Clone)]
pub struct PEInfo {
    pub magic: u16,
    pub linker_version: (u8, u8), // (major, minor)
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u64, // RVA
    pub base_of_code: u32,

    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,

    pub os_version: (u16, u16),
    pub image_version: (u16, u16),
    pub subsystem_version: (u16, u16),
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

#[derive(Debug, Clone, Copy)]
pub enum PEKind {
    Pe32,
    Pe64,
}

impl PEInfo {
    pub fn from_pe32(pe: pelite::pe32::PeFile) -> Result<Self> {
        let opt = pelite::pe32::Pe::optional_header(pe);
        Ok(Self {
            magic: opt.Magic,
            linker_version: (opt.LinkerVersion.Major, opt.LinkerVersion.Minor),
            size_of_code: opt.SizeOfCode,
            size_of_initialized_data: opt.SizeOfInitializedData,
            size_of_uninitialized_data: opt.SizeOfUninitializedData,
            address_of_entry_point: opt.AddressOfEntryPoint as u64,
            base_of_code: opt.BaseOfCode,

            image_base: opt.ImageBase as u64,
            section_alignment: opt.SectionAlignment,
            file_alignment: opt.FileAlignment,

            os_version: (opt.OperatingSystemVersion.Major, opt.OperatingSystemVersion.Minor),
            image_version: (opt.ImageVersion.Major, opt.ImageVersion.Minor),
            subsystem_version: (opt.SubsystemVersion.Major, opt.SubsystemVersion.Minor),
            win32_version_value: opt.Win32VersionValue,

            size_of_image: opt.SizeOfImage,
            size_of_headers: opt.SizeOfHeaders,
            checksum: opt.CheckSum,

            subsystem: opt.Subsystem,
            dll_characteristics: opt.DllCharacteristics,

            // In PE32 these are u32; normalize to u64 for a single struct
            size_of_stack_reserve: opt.SizeOfStackReserve as u64,
            size_of_stack_commit: opt.SizeOfStackCommit as u64,
            size_of_heap_reserve: opt.SizeOfHeapReserve as u64,
            size_of_heap_commit: opt.SizeOfHeapCommit as u64,

            loader_flags: opt.LoaderFlags,
            number_of_rva_and_sizes: opt.NumberOfRvaAndSizes,
        })
    }

    pub fn from_pe64(pe: pelite::pe64::PeFile) -> Result<Self> {
        let opt = pe.optional_header();
        Ok(Self {
            magic: opt.Magic,
            linker_version: (opt.LinkerVersion.Major, opt.LinkerVersion.Minor),
            size_of_code: opt.SizeOfCode,
            size_of_initialized_data: opt.SizeOfInitializedData,
            size_of_uninitialized_data: opt.SizeOfUninitializedData,
            address_of_entry_point: opt.AddressOfEntryPoint as u64,
            base_of_code: opt.BaseOfCode,

            image_base: opt.ImageBase,
            section_alignment: opt.SectionAlignment,
            file_alignment: opt.FileAlignment,

            os_version: (opt.OperatingSystemVersion.Major, opt.OperatingSystemVersion.Minor),
            image_version: (opt.ImageVersion.Major, opt.ImageVersion.Minor),
            subsystem_version: (opt.SubsystemVersion.Major, opt.SubsystemVersion.Minor),
            win32_version_value: opt.Win32VersionValue,

            size_of_image: opt.SizeOfImage,
            size_of_headers: opt.SizeOfHeaders,
            checksum: opt.CheckSum,

            subsystem: opt.Subsystem,
            dll_characteristics: opt.DllCharacteristics,

            size_of_stack_reserve: opt.SizeOfStackReserve,
            size_of_stack_commit: opt.SizeOfStackCommit,
            size_of_heap_reserve: opt.SizeOfHeapReserve,
            size_of_heap_commit: opt.SizeOfHeapCommit,

            loader_flags: opt.LoaderFlags,
            number_of_rva_and_sizes: opt.NumberOfRvaAndSizes,
        })
    }

    pub fn entry_va(&self) -> u64 {
        self.image_base.saturating_add(self.address_of_entry_point as u64)
    }
}

impl Display for PEInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // helper to keep columns aligned (left key, right value)
        fn kv(f: &mut Formatter<'_>, k: &str, v: impl AsRef<str>) -> fmt::Result {
            write!(f, "{:<28} {}\n", k, v.as_ref())
        }

        kv(f, "Magic", format!("0x{:04X}", self.magic))?;
        kv(
            f,
            "LinkerVersion",
            format!("{}.{}", self.linker_version.0, self.linker_version.1),
        )?;
        kv(f, "SizeOfCode", format!("{} (0x{:X})", self.size_of_code, self.size_of_code))?;
        kv(
            f,
            "SizeOfInitializedData",
            format!("{} (0x{:X})", self.size_of_initialized_data, self.size_of_initialized_data),
        )?;
        kv(
            f,
            "SizeOfUninitializedData",
            format!("{} (0x{:X})", self.size_of_uninitialized_data, self.size_of_uninitialized_data),
        )?;
        kv(
            f,
            "AddressOfEntryPoint (RVA)",
            format!("0x{:08X}", self.address_of_entry_point),
        )?;
        kv(f, "BaseOfCode", format!("0x{:08X}", self.base_of_code))?;

        kv(f, "ImageBase", format!("0x{:016X}", self.image_base))?;
        kv(f, "EntryPoint (VA)", format!("0x{:016X}", self.entry_va()))?;
        kv(f, "SectionAlignment", format!("{} (0x{:X})", self.section_alignment, self.section_alignment))?;
        kv(f, "FileAlignment", format!("{} (0x{:X})", self.file_alignment, self.file_alignment))?;

        kv(
            f,
            "OS Version",
            format!("{}.{}", self.os_version.0, self.os_version.1),
        )?;
        kv(
            f,
            "Image Version",
            format!("{}.{}", self.image_version.0, self.image_version.1),
        )?;
        kv(
            f,
            "Subsystem Version",
            format!("{}.{}", self.subsystem_version.0, self.subsystem_version.1),
        )?;
        kv(f, "Win32VersionValue", format!("0x{:08X}", self.win32_version_value))?;

        kv(f, "SizeOfImage", format!("{} (0x{:X})", self.size_of_image, self.size_of_image))?;
        kv(f, "SizeOfHeaders", format!("{} (0x{:X})", self.size_of_headers, self.size_of_headers))?;
        kv(f, "CheckSum", format!("0x{:08X}", self.checksum))?;

        kv(f, "Subsystem", format!("0x{:04X}", self.subsystem))?;
        kv(f, "DllCharacteristics", format!("0x{:04X}", self.dll_characteristics))?;

        kv(
            f,
            "SizeOfStackReserve",
            format!("{} (0x{:X})", self.size_of_stack_reserve, self.size_of_stack_reserve),
        )?;
        kv(
            f,
            "SizeOfStackCommit",
            format!("{} (0x{:X})", self.size_of_stack_commit, self.size_of_stack_commit),
        )?;
        kv(
            f,
            "SizeOfHeapReserve",
            format!("{} (0x{:X})", self.size_of_heap_reserve, self.size_of_heap_reserve),
        )?;
        kv(
            f,
            "SizeOfHeapCommit",
            format!("{} (0x{:X})", self.size_of_heap_commit, self.size_of_heap_commit),
        )?;

        kv(f, "LoaderFlags", format!("0x{:08X}", self.loader_flags))?;
        kv(
            f,
            "NumberOfRvaAndSizes",
            format!("{} (0x{:X})", self.number_of_rva_and_sizes, self.number_of_rva_and_sizes),
        )?;

        Ok(())
    }
}
