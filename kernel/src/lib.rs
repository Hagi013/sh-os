#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![feature(no_std)]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[no_mangle]
fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

#[no_mangle]
#[start]
pub extern fn init_os() {
    loop {
        hlt()
    }
}

#[lang = "eh_personality"]
extern fn en_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}