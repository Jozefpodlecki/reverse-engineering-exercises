use std::{ffi::OsStr, fs, iter::once, os::windows::ffi::OsStrExt, path::{Path, PathBuf}};
use anyhow::Result;
use log::info;
use minidump::Minidump;
use windows::{core::{PCWSTR, PWSTR}, Win32::{Foundation::{CloseHandle, HANDLE}, Storage::FileSystem::{CreateFileW, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_WRITE, FILE_SHARE_NONE}, System::{Diagnostics::Debug::{MiniDumpWithFullMemoryInfo, MiniDumpWriteDump}, Threading::{CreateProcessW, OpenProcess, SuspendThread, CREATE_SUSPENDED, PROCESS_ALL_ACCESS, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTUPINFOW}}}};

use crate::utils::kill_process_by_name;

pub struct Dump(PathBuf);

impl Dump {
    pub fn new(input: PathBuf) -> Self {
        Self(input)
    }

    pub fn run(&self) -> Result<()> {

        let dump_path = get_dump_path(&self.0);
        let file_name = self.0.file_name().unwrap().to_string_lossy();

        kill_process_by_name(&file_name)?;

        // if dump_path.exists() {
        //     fs::remove_file(&dump_path)?;
        // }

        if dump_path.exists() {
            let _dump = Minidump::read_path(&dump_path)?;
            return Ok(());
        }

        let pi = launch_process(&self.0)?;
        // unsafe {
        //     SuspendThread(pi.hThread);
        // }
        info!("Opening process with all access");
        let process_handle = unsafe {
            OpenProcess(PROCESS_ALL_ACCESS, false, pi.dwProcessId)?
        };
        write_minidump(pi.dwProcessId, process_handle, &dump_path)?;

        info!("Closing handles");
        unsafe {
            CloseHandle(pi.hProcess)?;
            CloseHandle(pi.hThread)?;
        }

        let _dump = Minidump::read_path(&dump_path)?;

        Ok(())
    }
}

fn launch_process(exe_path: &Path) -> Result<PROCESS_INFORMATION> {
    let mut cmd: Vec<u16> = OsStr::new(exe_path)
        .encode_wide()
        .chain(once(0))
        .collect();

    let mut si = STARTUPINFOW::default();
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

    let mut pi = PROCESS_INFORMATION::default();

    info!("Creating process {:?}", exe_path);
    unsafe {
        CreateProcessW(
            PCWSTR::null(),
            Some(PWSTR(cmd.as_mut_ptr())),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            PCWSTR::null(),
            &si,
            &mut pi,
        )?;
    }

    Ok(pi)
}

fn write_minidump(process_id: u32, process_handle: HANDLE, dump_path: &Path) -> Result<()> {
    info!("Creating {:?}", dump_path);    

    let file_path = to_pcwstr_from_path(dump_path);
    let file_handle = unsafe {
        CreateFileW(
            file_path,
            FILE_GENERIC_WRITE.0,
            FILE_SHARE_NONE,
            None,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?
    };

    info!("Writing minidump");
    unsafe {
        MiniDumpWriteDump(
            process_handle,
            process_id,
            file_handle,
            MiniDumpWithFullMemoryInfo,
            None,
            None,
            None,
        )?;
        CloseHandle(file_handle)?;
    }

    Ok(())
}

fn get_dump_path(input: &Path) -> PathBuf {
    let file_stem = input.file_stem().unwrap().to_string_lossy();
    input.parent().unwrap().join(format!("{}.dmp", file_stem))
}

fn to_pcwstr_from_path(str: &Path) -> PCWSTR {
    let wide: Vec<u16> = str.as_os_str().encode_wide().chain(once(0)).collect();
    PCWSTR(wide.as_ptr())
}

fn to_pcwstr(s: &str) -> PCWSTR {
    let wide: Vec<u16> = OsStr::new(s).encode_wide().chain(once(0)).collect();
    PCWSTR(wide.as_ptr())
}