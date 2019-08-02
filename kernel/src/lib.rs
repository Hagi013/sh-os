#![no_std]
#![no_main]
#![feature(lang_items, start, asm, const_raw_ptr_deref)]
#![feature(const_fn)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(alloc)]

use core::panic::PanicInfo;
use core::str;
use core::fmt;
use core::alloc::Layout;
#[macro_use]
use core::fmt::{ Write, Display };


#[macro_use]
extern crate alloc;

use alloc::string::String;
//use alloc::borrow::ToOwned;
use alloc::string::ToString;

#[allow(unused_imports)]
#[cfg(all(not(test), target_arch = "x86"))]
#[macro_use]
pub mod arch;
use self::arch::boot_info::BootInfo;
use self::arch::graphic::Graphic;
use self::arch::graphic::MouseGraphic;
use self::arch::graphic::Printer;
use self::arch::asmfunc;
use self::arch::dsctbl::DscTbl;
use self::arch::pic;
use self::arch::keyboard;

pub mod sync;
use self::sync::queue;

#[allow(unused_imports)]
pub mod allocator;
use self::allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap;

#[allow(unused_imports)]
pub mod spin;
use spin::mutex::Mutex;
use spin::mutex::MutexGuard;

use alloc::collections::vec_deque::VecDeque;

fn init_heap() {
//    let heap_start: usize = 0x00400000;
//    let heap_end: usize = 0xbfffffff;
    let heap_start: usize = 0x00800000;
     let heap_end: usize = 0x00f00000;

    let heap_size: usize = heap_end - heap_start;
    let mut printer = Printer::new(0, 80, 10);
    write!(printer, "{:x}", heap_size).unwrap();
    ALLOCATOR.init(heap_start, heap_size);
}

#[start]
#[no_mangle]
pub extern fn init_os() {

    Graphic::init();
//    Graphic::putfont_asc(210, 120, 10, "12345");
//    Graphic::putfont_asc(210, 140, 10, "abcd");
    Graphic::putfont_asc(210, 150, 10, "rio-os");

    pic::init_pic();
    let dsc_tbl: DscTbl = DscTbl::init_gdt_idt();
    asmfunc::io_sti();

    let mouse: MouseGraphic = MouseGraphic::new();
    mouse.init_mouse_cursor(14);

    init_heap();

    let a: String = "String Alloc!!".to_string();
    Graphic::putfont_asc(210, 180, 10, &a);

    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_front(1);
    v.push_front(2);
    v.push_back(3);
    for i in 0..v.len() {
        let mut printer = Printer::new((210 + i * 10) as u32, (480 + i * 10) as u32, 10);
        write!(printer, "{:?}", v.pop_front().unwrap()).unwrap();
    }

//    let b: String = "String Alloc!!2222".to_string();
//    Graphic::putfont_asc(210, 200, 10, &b);

    keyboard::allow_pic1_keyboard_int();
    pic::allow_mouse_int();

    let mut idx: u32 = 10;
    loop {
        if keyboard::is_existing() {
            asmfunc::io_cli();
            let data: i32 = keyboard::get_data().unwrap();
            asmfunc::io_sti();
            Graphic::putfont_asc_from_keyboard(idx, 15, 10, data);
            idx += 8;
        }
    }
}

#[no_mangle]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    Graphic::putfont_asc(0, 100, 10, "panic!!!!!");
    let mut printer = Printer::new(0, 120, 10);
    write!(printer, "{:?}", _info.location().unwrap().file()).unwrap();
    let mut printer = Printer::new(0, 140, 10);
    write!(printer, "{:?}", _info.location().unwrap().line()).unwrap();
    let mut printer = Printer::new(0, 160, 10);
    write!(printer, "{:?}", _info.location().unwrap().column()).unwrap();

    loop {
        asmfunc::io_hlt();
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn _Unwind_Resume(_ex_obj: *mut ()) { }

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    Graphic::putfont_asc(0, 180, 10, "alloc_error_handler!!!!!");
    let mut printer = Printer::new(0, 200, 10);
    write!(printer, "{:?}", layout.size()).unwrap();

    loop {
        asmfunc::io_hlt();
    }
}