#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[allow(unused_imports)]
#[cfg(all(not(test), target_arch = "x86"))]
#[macro_use]
extern crate asm;

use asm::BootInfo;
use asm::hlt;

#[no_mangle]
#[start]
pub extern fn init_os() {
    // let bootInfo: BootInfo = BootInfo::new();
//    let mut memory_address = unsafe { *(0x000a0000 as *mut u32) };
//    for i in 0..0xffff {
//        unsafe {
//            let memory: *mut u8 = (memory_address + i) as *mut u8;
//            *memory = (*(i as *mut u8) & 0x0f);
//        }
//    }

    loop {
//        hlt();
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
