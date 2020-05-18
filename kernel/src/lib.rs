#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(lang_items, start, asm, const_raw_ptr_deref)]
#![feature(const_fn)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
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
use arch::boot_info::BootInfo;
use arch::graphic::Graphic;
use arch::graphic::MouseGraphic;
use arch::graphic::Printer;
use arch::asmfunc;
use arch::dsctbl::DscTbl;
use arch::pic;
use arch::keyboard;
use arch::mouse;
use arch::timer::{ timer_init, get_uptime };

pub mod window;
use window::{ Window, WindowsManager };

pub mod sync;
use sync::queue;

#[allow(unused_imports)]
pub mod allocator;
use allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap;

#[allow(unused_imports)]
pub mod spin;
use spin::mutex::Mutex;
use spin::mutex::MutexGuard;

#[allow(unused_imports)]
pub mod util;
use util::lazy_static;
#[macro_use]
use util::lazy_static::*;

use alloc::collections::vec_deque::VecDeque;
use core::borrow::Borrow;

fn init_heap() {
    let heap_start: usize = 0x00400000;
   // let heap_end: usize = 0xbfffffff;
//    let heap_start: usize = 0x00800000;
//     let heap_end: usize = 0x01ff0000;
    let heap_end: usize = 0x3fff0000;

    let heap_size: usize = heap_end - heap_start;
    let mut printer = Printer::new(0, 80, 10);
    write!(printer, "{:x}", heap_size).unwrap();
    ALLOCATOR.init(heap_start, heap_size);
}

#[cfg(not(test))]
#[start]
#[no_mangle]
pub extern fn init_os(argc: isize, argv: *const *const u8) -> isize {
//pub extern fn init_os() {

    Graphic::init();
    Graphic::putfont_asc(210, 150, 10, "rio-os");

    pic::init_pic();
    let dsc_tbl: DscTbl = DscTbl::init_gdt_idt();
    asmfunc::io_sti();

    init_heap();

    let mut window_manager: WindowsManager = WindowsManager::new();
    timer_init();
    keyboard::allow_pic1_keyboard_int();
    mouse::allow_mouse_int();

    let mouse: MouseGraphic = MouseGraphic::new();
    let mouse_state = mouse.init_mouse_cursor(14);

    let mut mouse_window: *mut Window = window_manager.create_window(mouse_state.1, mouse_state.2, mouse_state.3, mouse_state.4, mouse_state.0).unwrap();
    let mut idx: u32 = 10;
    loop {
        asmfunc::io_cli();
        if !keyboard::is_existing() && !mouse::is_existing() {
            asmfunc::io_stihlt();
            continue;
        }
        if keyboard::is_existing() {
            match keyboard::get_data() {
                Ok(data) => {
                    asmfunc::io_sti();
                    Graphic::putfont_asc_from_keyboard(idx, 15, 10, data);
                },
                Err(_) => asmfunc::io_sti(),
            };
            idx += 8;
        }
        if mouse::is_existing() {
            match mouse::get_data() {
                Ok(data) => {
                    asmfunc::io_sti();
                    match data {
                        Some(status) => {
                            let x: i32 = status.1;
                            let y: i32 = status.2;
                            mouse_window = match window_manager.move_window(mouse_window, x, y) {
                                Ok(m_w) => m_w,
                                Err(message) => {
                                    Graphic::putfont_asc(200, 200, 10, &message);
                                    mouse_window
                                }
                            };
                        },
                        None => {},
                    }
                },
                Err(message) => {
                    asmfunc::io_sti();
                    let mut printer = Printer::new(200, 215, 10);
                    write!(printer, "{:?}", message).unwrap();
                },
            };
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
    let mut printer = Printer::new(0, 180, 10);
    write!(printer, "{:?}", _info.message().unwrap()).unwrap();
    let mut printer = Printer::new(0, 300, 10);
    write!(printer, "{:?}", _info.payload().downcast_ref::<&str>()).unwrap();
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

//#[cfg(test)]
//#[no_mangle]
//pub extern "C" fn main() {
//    test_main();
//}
//
//#[allow(unused_imports)]
//#[cfg(all(test))]
//#[macro_use]
//pub mod arch;
//
//#[cfg(test)]
//fn test_runner(tests: &[&dyn Fn()]) {
////    println!("Running {} tests", tests.len());
//    for test in tests {
//        test();
//    }
//}
//
//#[test_case]
//fn trivial_assertion() {
////    println!("trivial assertion... ");
//    assert_eq!(1, 1);
////    println!("[ok]");
//}
