#![feature(lang_items)]
#![no_main]
#![no_std]
#![feature(asm)]

// import asm function
extern "C" {
    #[cfg(any(target_arch = "x86"))]
    fn io_hlt();
}

// public interface
pub struct BootInfo {
    cyls: u32,      /* ブートセクタはどこまでディスクを読み込んだのか */
    leds: u32,      /* ブートの時のキーボードのLEDの状態 */
    vmode: u32,     /* ビデオモード 何ビットカラーか */
    scrnx: u16,     /* 画像解像度 */
    scrny: u16,     /* 画像解像度 */
    vram: u32,      /* vram */
}

impl BootInfo {
    pub fn new() -> Self {
        BootInfo {
            cyls:   unsafe   { *(0x00000ff0 as *const u32) },
            leds:   unsafe   { *(0x00000ff1 as *const u32) },
            vmode:  unsafe   { *(0x00000ff2 as *const u32) },
            scrnx:  unsafe   { *(0x00000ff4 as *const u16) },
            scrny:  unsafe   { *(0x00000ff6 as *const u16) },
            vram:   unsafe   { *(0x00000ff8 as *mut   u32) },
        }
    }

    // 後で移動する必要がありそう
    fn draw_fonts_on_back(&self, x: &u32, y: &u32, color: u8, font: *mut u16) {
        for i in 0..15 {
            unsafe {
                let p: *mut u8 = (self.vram + (y + i) * (self.scrnx as u32) + x) as *mut u8; /* VRAMと画面上の点との関係 [VRAM] + x + y * xsize(screenの横幅) */
                let d: *mut u16 = font.offset(i as isize);
                if (*d & 0x80) != 0 { *(p.offset(i as isize)) = color; }
                if (*d & 0x40) != 0 { *(p.offset(i as isize)) = color; }
                if (*d & 0x20) != 0 { *(p.offset(i as isize)) = color; }
                if (*d & 0x10) != 0 { *(p.offset(i as isize)) = color; }
                if (*d & 0x08) != 0 { *(p.offset(i as isize)) = color; }
                if (*d & 0x04) != 0 { *(p.offset(i as isize)) = color; }
                if (*d & 0x02) != 0 { *(p.offset(i as isize)) = color; }
                if (*d & 0x01) != 0 { *(p.offset(i as isize)) = color; }
            }
        }
    }
}

pub extern fn hlt() {
    unsafe{
        io_hlt();
    }
}
