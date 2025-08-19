
#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    loop {}
}
