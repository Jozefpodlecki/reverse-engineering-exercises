use yew::prelude::*;
use js_sys::{Date, Intl::{self, DateTimeStyle}, JsString};
use wasm_bindgen::JsValue;
use yew_icons::{Icon, IconData};

pub fn format_timestamp(timestamp: f64) -> String {
    let date = Date::new(&timestamp.into());
    
    let options = Intl::DateTimeFormatOptions::new();
    options.set_date_style(DateTimeStyle::Medium);
    options.set_time_style(DateTimeStyle::Short);
    
    date.to_locale_string("en-US", &options).into()
}

pub fn format_unix_timestamp(timestamp: u32) -> String {
    let date = Date::new(&(timestamp as f64 * 1000.0).into());
    
    let options = Intl::DateTimeFormatOptions::new();
    options.set_date_style(DateTimeStyle::Medium);
    options.set_time_style(DateTimeStyle::Short);
    
    date.to_locale_string("en-US", &options).into()
}

pub fn format_hex_bytes(data: &[u8], max_bytes: usize) -> String {
    let len = data.len().min(max_bytes);
    
    if len == 0 {
        return "Empty file".to_string();
    }

    data[..len]
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn machine_to_string(machine: u16) -> &'static str {
    match machine {
        0x014c => "Intel 386",
        0x8664 => "AMD64",
        0x0200 => "Intel Itanium",
        0x01c4 => "ARM",
        0xaa64 => "ARM64",
        0x01c2 => "ARMv7 Thumb",
        0x01d3 => "MIPS",
        0x01f0 => "PowerPC",
        0x01f1 => "PowerPC FP",
        _ => "Unknown",
    }
}

pub fn dll_characteristics_to_strings(chars: u16) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if chars & 0x0001 != 0 { flags.push("HIGH_ENTROPY_VA"); }
    if chars & 0x0010 != 0 { flags.push("DYNAMIC_BASE"); }
    if chars & 0x0020 != 0 { flags.push("FORCE_INTEGRITY"); }
    if chars & 0x0040 != 0 { flags.push("NX_COMPAT"); }
    if chars & 0x0080 != 0 { flags.push("NO_ISOLATION"); }
    if chars & 0x0100 != 0 { flags.push("NO_SEH"); }
    if chars & 0x0200 != 0 { flags.push("NO_BIND"); }
    if chars & 0x0400 != 0 { flags.push("APPCONTAINER"); }
    if chars & 0x0800 != 0 { flags.push("WDM_DRIVER"); }
    if chars & 0x1000 != 0 { flags.push("GUARD_CF"); }
    if chars & 0x2000 != 0 { flags.push("TERMINAL_SERVER_AWARE"); }
    flags
}

pub fn file_characteristics_to_strings(chars: u16) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if chars & 0x0001 != 0 { flags.push("RELOCS_STRIPPED"); }
    if chars & 0x0002 != 0 { flags.push("EXECUTABLE_IMAGE"); }
    if chars & 0x0004 != 0 { flags.push("LINE_NUMS_STRIPPED"); }
    if chars & 0x0008 != 0 { flags.push("LOCAL_SYMS_STRIPPED"); }
    if chars & 0x0010 != 0 { flags.push("AGGRESSIVE_WS_TRIM"); }
    if chars & 0x0020 != 0 { flags.push("LARGE_ADDRESS_AWARE"); }
    if chars & 0x0040 != 0 { flags.push("RESERVED"); }
    if chars & 0x0080 != 0 { flags.push("BYTES_REVERSED_LO"); }
    if chars & 0x0100 != 0 { flags.push("32BIT_MACHINE"); }
    if chars & 0x0200 != 0 { flags.push("DEBUG_STRIPPED"); }
    if chars & 0x0400 != 0 { flags.push("REMOVABLE_RUN_FROM_SWAP"); }
    if chars & 0x0800 != 0 { flags.push("NET_RUN_FROM_SWAP"); }
    if chars & 0x1000 != 0 { flags.push("SYSTEM"); }
    if chars & 0x2000 != 0 { flags.push("DLL"); }
    if chars & 0x4000 != 0 { flags.push("UP_SYSTEM_ONLY"); }
    if chars & 0x8000 != 0 { flags.push("BYTES_REVERSED_HI"); }
    flags
}

pub fn section_characteristics_to_strings(chars: u32) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if chars & 0x00000020 != 0 { flags.push("CODE"); }
    if chars & 0x00000040 != 0 { flags.push("INITIALIZED_DATA"); }
    if chars & 0x00000080 != 0 { flags.push("UNINITIALIZED_DATA"); }
    if chars & 0x00000200 != 0 { flags.push("LINKER_INFO"); }
    if chars & 0x00000800 != 0 { flags.push("LINKER_REMOVE"); }
    if chars & 0x00001000 != 0 { flags.push("LINKER_COMDAT"); }
    if chars & 0x00004000 != 0 { flags.push("NO_DEFER_SPEC_EXC"); }
    if chars & 0x00008000 != 0 { flags.push("GPREL"); }
    if chars & 0x02000000 != 0 { flags.push("DISCARDABLE"); }
    if chars & 0x04000000 != 0 { flags.push("NOT_CACHED"); }
    if chars & 0x08000000 != 0 { flags.push("NOT_PAGED"); }
    if chars & 0x10000000 != 0 { flags.push("SHARED"); }
    if chars & 0x20000000 != 0 { flags.push("EXECUTE"); }
    if chars & 0x40000000 != 0 { flags.push("READ"); }
    if chars & 0x80000000 != 0 { flags.push("WRITE"); }
    flags
}

pub fn subsystem_to_string(subsystem: u16) -> &'static str {
    match subsystem {
        0 => "Unknown",
        1 => "Native",
        2 => "Windows GUI",
        3 => "Windows CUI",
        5 => "OS/2 CUI",
        7 => "POSIX CUI",
        8 => "Native Windows",
        9 => "Windows CE",
        10 => "EFI Application",
        11 => "EFI Boot Service Driver",
        12 => "EFI Runtime Driver",
        13 => "EFI ROM",
        14 => "XBOX",
        _ => "Unknown",
    }
}