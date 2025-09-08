use winapi::shared::ntdef::VOID;
use winapi::um::{consoleapi::WriteConsoleA, winbase::STD_OUTPUT_HANDLE};
use winapi::um::processenv::GetStdHandle;

pub struct ConsoleLogger {
    handle: winapi::shared::ntdef::HANDLE,
}

impl ConsoleLogger {
    pub fn new() -> Self {
        let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        Self { handle }
    }

    pub fn log(&self, message: &str) {
        let bytes = message.as_bytes();
        let mut written = 0u32;

        unsafe {
            WriteConsoleA(
                self.handle,
                bytes.as_ptr() as *const VOID,
                bytes.len() as u32,
                &mut written,
                core::ptr::null_mut(),
            );
        }
    }
}