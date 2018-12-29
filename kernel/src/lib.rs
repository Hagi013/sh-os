#![no_std]
#![no_main]
#![feature(lang_items, start, asm)]

use core::panic::PanicInfo;

#[allow(unused_imports)]
#[cfg(all(not(test), target_arch = "x86"))]
#[macro_use]
pub mod arch;
use self::arch::BootInfo;

#[start]
#[no_mangle]
pub extern fn init_os() {
    let bootInfo: BootInfo = BootInfo::new();
    let i: &[u8] = b"a";
    bootInfo.draw_fonts_on_back(100, 200, 7, i);

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
        hlt();
    }
}



// import asm function
//extern "C" {
//    #[cfg(any(target_arch = "x86"))]
//    pub fn io_hlt();
//}
//
//// public interface
//pub struct BootInfo {
//    cyls: u32,      /* ブートセクタはどこまでディスクを読み込んだのか */
//    leds: u32,      /* ブートの時のキーボードのLEDの状態 */
//    vmode: u32,     /* ビデオモード 何ビットカラーか */
//    scrnx: u16,     /* 画像解像度 */
//    scrny: u16,     /* 画像解像度 */
//    vram: u32,      /* vram */
//}
//
//impl BootInfo {
//    pub fn new() -> Self {
//        BootInfo {
//            cyls:   unsafe   { *(0x00000ff0 as *const u32) },
//            leds:   unsafe   { *(0x00000ff1 as *const u32) },
//            vmode:  unsafe   { *(0x00000ff2 as *const u32) },
//            scrnx:  unsafe   { *(0x00000ff4 as *const u16) },
//            scrny:  unsafe   { *(0x00000ff6 as *const u16) },
//            vram:   unsafe   { *(0x00000ff8 as *mut   u32) },
//        }
//    }
//
//    // 後で移動する必要がありそう
//    pub fn draw_fonts_on_back(&self, mut x: u32, y: u32, color: u8, font: &[u8]) {
//        for (i, &c) in font.iter().enumerate() {
//            self.draw_fonts(x, y, color, c);
//            x += 8;
//        }
//    }
//
//    fn draw_fonts(&self, x: u32, y: u32, color: u8, font: u8) {
//        for i in 0..15 {
//            unsafe {
//                let p: *mut u8 = (self.vram + (y + i) * (self.scrnx as u32) + x) as *mut u8; /* VRAMと画面上の点との関係 [VRAM] + x + y * xsize(screenの横幅) */
//                if (font & 0x80) != 0 { *(p.offset(i as isize)) = color; }
//                if (font & 0x40) != 0 { *(p.offset(i as isize)) = color; }
//                if (font & 0x20) != 0 { *(p.offset(i as isize)) = color; }
//                if (font & 0x10) != 0 { *(p.offset(i as isize)) = color; }
//                if (font & 0x08) != 0 { *(p.offset(i as isize)) = color; }
//                if (font & 0x04) != 0 { *(p.offset(i as isize)) = color; }
//                if (font & 0x02) != 0 { *(p.offset(i as isize)) = color; }
//                if (font & 0x01) != 0 { *(p.offset(i as isize)) = color; }
//            }
//        }
//    }
//}

pub extern fn hlt() {
    unsafe {
//        io_hlt();
        asm!("hlt" :::: "intel");
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
