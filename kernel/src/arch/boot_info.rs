pub const ADR_BOOTINFO: u32 = 0x00000ff0;

// public interface
pub struct BootInfo {
    cyls: u32,      /* ブートセクタはどこまでディスクを読み込んだのか */
    leds: u32,      /* ブートの時のキーボードのLEDの状態 */
    vmode: u32,     /* ビデオモード 何ビットカラーか */
    pub scrnx: u16,     /* 画像解像度 */
    pub scrny: u16,     /* 画像解像度 */
    pub vram: u32,      /* vram */
}

impl BootInfo {
    pub fn new() -> Self {
        BootInfo {
            cyls:   unsafe   { *(ADR_BOOTINFO as *const u32) },
            leds:   unsafe   { *((ADR_BOOTINFO + 0x01) as *const u32) },
            vmode:  unsafe   { *((ADR_BOOTINFO + 0x02) as *const u32) },
            scrnx:  unsafe   { *((ADR_BOOTINFO + 0x04) as *const u16) },
            scrny:  unsafe   { *((ADR_BOOTINFO + 0x06) as *const u16) },
            vram:   unsafe   { *((ADR_BOOTINFO + 0x08) as *mut   u32) },
        }
    }

    pub fn draw_fonts_on_back(&self, mut x: u32, y: u32, color: u8, font: &[u8]) {
        for (i, &c) in font.iter().enumerate() {
            self.draw_fonts(x, y, color, c);
            x += 8;
        }
    }
    fn draw_fonts(&self, x: u32, y: u32, color: u8, font: u8) {
        for i in 0..15 {
            unsafe {
                let p: *mut u8 = (self.vram + (y + i) * (self.scrnx as u32) + x) as *mut u8; /* VRAMと画面上の点との関係 [VRAM] + x + y * xsize(screenの横幅) */
                if (font & 0x80) != 0 { *(p.offset(i as isize)) = color; }
                if (font & 0x40) != 0 { *(p.offset(i as isize)) = color; }
                if (font & 0x20) != 0 { *(p.offset(i as isize)) = color; }
                if (font & 0x10) != 0 { *(p.offset(i as isize)) = color; }
                if (font & 0x08) != 0 { *(p.offset(i as isize)) = color; }
                if (font & 0x04) != 0 { *(p.offset(i as isize)) = color; }
                if (font & 0x02) != 0 { *(p.offset(i as isize)) = color; }
                if (font & 0x01) != 0 { *(p.offset(i as isize)) = color; }
            }
        }
    }
}

//    let mut bootInfo: BootInfo = BootInfo::new();
//    let i: &[u8] = b"a";
//    bootInfo.draw_fonts_on_back(100, 200, 7, i);

