#![no_std]
#![no_main]
#![feature(lang_items, start, asm)]
//#![feature(alloc)]

//#[macro_use]
//extern crate alloc;

use core::panic::PanicInfo;

#[allow(unused_imports)]
#[cfg(all(not(test), target_arch = "x86"))]
#[macro_use]
pub mod arch;

use self::arch::boot_info::BootInfo;
use self::arch::graphic::Graphic;
use self::arch::asmfunc;

#[start]
#[no_mangle]
pub extern fn init_os() {
    Graphic::init();
    let graphic: Graphic = Graphic::new(BootInfo::new());
    graphic.init_screen();

    let mut address = 0x000a0000 as u32;
    let raw = &mut address as *mut u32;
    let memory_address: u32 = unsafe { *raw };

//        for i in 0..0xfffffff {
////    for i in 0..(0xbffff - address) {
//        unsafe {
//            let memory: *mut u8 = (memory_address + i) as *mut u8;
////            *memory = (*(i as *mut u8) & 0x0f);
////            *memory = (*(i as *mut u8));
////                *memory = 0x4d;
//            *memory = 0x0f;
//        }
//    }

    loop {
        asmfunc::io_hlt();
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
