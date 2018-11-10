#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[cfg(any(target = "x86"))]
fn hlt() {
    unsafe {
        asm!("hlt" :::: "intel");
    }
}

#[no_mangle]
#[start]
pub extern fn init_os() {
    loop {}
}

#[lang = "eh_personality"]
extern fn en_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}