#![no_std]
#![no_main]
#![feature(lang_items, start, asm)]

use core::panic::PanicInfo;

//#[allow(unused_imports)]
//#[cfg(all(not(test), target_arch = "x86"))]
//#[macro_use]
//extern crate asm;
//
//use asm::BootInfo;
//use asm::hlt;

#[start]
#[no_mangle]
pub extern fn init_os() {
//    let bootInfo: BootInfo = BootInfo::new();
//    let mut i = 20;
//    let c = &i;
//    bootInfo.draw_fonts_on_back(&100, &200, 7, *i);

    let mut address = 0x000a0000 as u32;
    let raw = &mut address as *mut u32;
    let memory_address: u32 = unsafe { *raw };

    for i in 0..0xffff {
        unsafe {
            let memory: *mut u8 = (memory_address + i) as *mut u8;
            *memory = (*(i as *mut u8) & 0x0f);
        }
    }
    loop {
//        hlt();
    }
}

#[no_mangle]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn _Unwind_Resume(_ex_obj: *mut ()) { }
