#![no_std]
#![no_main]
#![feature(lang_items, start, asm, const_raw_ptr_deref)]
// #![feature(alloc)]
#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
#[macro_use]
extern crate alloc;


use core::panic::PanicInfo;
use core::str;
use core::fmt::Write;
// use alloc::string::String;
//use alloc::borrow::ToOwned;
//use alloc::string::ToString;

#[allow(unused_imports)]
#[cfg(all(not(test), target_arch = "x86"))]
#[macro_use]
pub mod arch;

use self::arch::boot_info::BootInfo;
use self::arch::graphic::Graphic;
use self::arch::graphic::MouseGraphic;
use self::arch::asmfunc;
use self::arch::dsctbl::DscTbl;
use self::arch::pic;

#[start]
#[no_mangle]
pub extern fn init_os() {

    Graphic::init();
    Graphic::putfont_asc(210, 120, 10, "12345");
    Graphic::putfont_asc(210, 140, 10, "abcd");
    Graphic::putfont_asc(210, 150, 10, "rio-os");

    pic::init_pic();
    let dsc_tbl: DscTbl = DscTbl::init_gdt_idt();
    asmfunc::io_sti();

    let mouse: MouseGraphic = MouseGraphic::new();
    mouse.init_mouse_cursor(14);

    pic::allow_pic1_keyboard_int();
    pic::allow_mouse_int();

    let num: i32 = 123;
    // Graphic::putfont_asc(10, 80, 10, &str::from_utf8_mut(&mut num.to_be_bytes()).unwrap());
//    let mut output = "".to_string();
//    write!(output, "{}", num);
//    Graphic::putfont_asc(10, 80, 10, &output);

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
