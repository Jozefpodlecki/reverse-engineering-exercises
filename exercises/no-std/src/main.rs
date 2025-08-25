
#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::panic::PanicInfo;

use windows_sys::Win32::System::Console::GetStdHandle;
use windows_sys::Win32::System::Console::WriteConsoleA;
use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;
use windows_sys::Win32::System::Threading::ExitProcess;
use windows_sys::Win32::System::Threading::Sleep;

#[cfg(not(test))]
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    let message = "hello world\n";
	unsafe {
		let console = GetStdHandle(STD_OUTPUT_HANDLE);

		WriteConsoleA(
			console,
			message.as_ptr(),
			message.len() as u32,
			core::ptr::null_mut(),
			core::ptr::null(),
		);

		Sleep(10_000);

		ExitProcess(0)
	}
}
