use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use log::info;
use windows::Win32::{Foundation::CloseHandle, System::{Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS}, Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE}}};

pub fn kill_process_by_name(target_name: &str) -> windows::core::Result<()> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut entry = PROCESSENTRY32W::default();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                let exe_name_str = &entry.szExeFile[..entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)];
                let exe_name: OsString = OsString::from_wide(exe_name_str);

                if exe_name.to_string_lossy().eq_ignore_ascii_case(target_name) {

                    info!("Killing process {}", entry.th32ProcessID);
                    // let h_process = OpenProcess(PROCESS_TERMINATE, false, entry.th32ProcessID)?;
                    // TerminateProcess(h_process, 1)?;
                    // CloseHandle(h_process)?;
                }

                if !Process32NextW(snapshot, &mut entry).is_ok() {
                    break;
                }
            }
        }

        CloseHandle(snapshot)?;
    }
    Ok(())
}