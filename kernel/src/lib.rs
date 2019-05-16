#![no_std]
#![no_main]
#![feature(lang_items, start, asm, const_raw_ptr_deref)]
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
use self::arch::hankaku;
use self::arch::dsctbl;
use self::arch::pic;

#[start]
#[no_mangle]
pub extern fn init_os() {
     Graphic::init();
     let graphic: Graphic = Graphic::new(BootInfo::new());
     graphic.init_screen();

    // graphic.putfont_asc(200, 140, 10);
    graphic.putfont_asc(210, 140, 10, "abcd");
    graphic.init_mouse_cursor(14);
//    graphic.putfont(&x, &y, &c, 0x90);
//    graphic.putfont(&(&x + 20), &(&y + 10), &c, 0x9a);
    pic::init_pic();

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
