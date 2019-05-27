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
use self::arch::graphic::MouseGraphic;
use self::arch::asmfunc;
use self::arch::dsctbl::DscTbl;
use self::arch::pic::PIC;

#[start]
#[no_mangle]
pub extern fn init_os() {

    Graphic::init();
    Graphic::putfont_asc(210, 140, 10, "abcd");
    Graphic::putfont_asc(210, 150, 10, "rio-os");

    let mouse: MouseGraphic = MouseGraphic::new();
    mouse.init_mouse_cursor(14);

    PIC::init_pic();
    let dsc_tbl: DscTbl = DscTbl::init_gdt_idt();

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
