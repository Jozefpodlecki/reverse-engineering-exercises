
#![no_std]
#![no_main]
#![windows_subsystem = "console"]

#[cfg(not(test))]
use core::panic::PanicInfo;

use winapi::um::processthreadsapi::ExitProcess;

use crate::crt::{__scrt_common_main_seh, __security_init_cookie};

mod crt;
mod logger;

#[cfg(not(test))]
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
	__security_init_cookie();
	__scrt_common_main_seh();

	unsafe { core::hint::unreachable_unchecked() } 
}
