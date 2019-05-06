use core::slice::Iter;

//use alloc::vec::Vec;

use super::asmfunc;
use super::boot_info::BootInfo;
use super::hankaku::Hankaku;


pub struct Graphic {
    boot_info: BootInfo,
    hankaku: [u8; 4096],
}

impl Graphic {
    pub fn new(bi: BootInfo) -> Self {
        Graphic {
            boot_info: bi,
            hankaku: Hankaku::new().get_fonts(),
        }
    }

    pub fn init() {
        Graphic::set_palette();
    }

    fn set_palette() {
        // 割り込み許可フラグの値を記録する
        let eflags: u32 = asmfunc::io_load_eflags();
        asmfunc::io_cli(); // 割り込みを禁止する
        asmfunc::io_out8(0x03c8, 0);

        for rgb in RGB::iterator() {
            asmfunc::io_out8(0x03c9, (rgb.r() >> 2) as i32);
            asmfunc::io_out8(0x03c9, (rgb.b() >> 2) as i32);
            asmfunc::io_out8(0x03c9, (rgb.b() >> 2) as i32);
        }
         asmfunc::io_store_eflags(eflags);
    }

    pub fn init_screen(&self) {
        let x: &u16 = &self.boot_info.scrnx;
        let y: &u16 = &self.boot_info.scrny;

        self.boxfill(RGB::DarkLightBlue.palette_no(), 0, 0,      x - 1, y - 29);
        self.boxfill(RGB::Gray.palette_no(),          0, y - 28, x - 1, y - 28);
        self.boxfill(RGB::White.palette_no(),         0, y - 27, x - 1, y - 27);
        self.boxfill(RGB::Gray.palette_no(),          0, y - 26, x - 1, y -  1);

        self.boxfill(RGB::White.palette_no(),         3, y - 24,    59, y - 24);
        self.boxfill(RGB::White.palette_no(),         2, y - 24,     2, y -  4);
        self.boxfill(RGB::DarkGray.palette_no(),      3, y -  4,    59, y -  4);
        self.boxfill(RGB::DarkGray.palette_no(),     59, y - 23,    59, y -  5);
        self.boxfill(RGB::Black.palette_no(),         2, y -  3,    59, y -  3);
        self.boxfill(RGB::Black.palette_no(),        60, y - 24,    60, y -  3);

        self.boxfill(RGB::DarkGray.palette_no(), x - 47, y - 24, x - 4, y - 24);
        self.boxfill(RGB::DarkGray.palette_no(), x - 47, y - 23, x -47, y -  4);
        self.boxfill(RGB::White.palette_no(),    x - 47, y -  3, x - 4, y -  3);
        self.boxfill(RGB::White.palette_no(),    x -  3, y - 24, x - 3, y -  3);
    }

    fn boxfill(&self, color: u8, from_x: u16, from_y: u16, to_x: u16, to_y: u16) {
        for y in from_y..to_y {
            for x in from_x..to_x {
                let address: *mut u8 = (self.boot_info.vram + ((y * self.boot_info.scrnx) + x) as u32) as *mut u8;
                unsafe { *address = color; }
            }
        }
    }

    // ToDo ここのsについて、どう渡すかを後で考える
//    pub fn putfont_asc(&self, x: u16, y: u16, c: u8, s: *mut u8) {
//        let mut idx: isize = 0;
//        unsafe {
//            while *s.offset(&idx) != 0x00 {
//                self.putfont(&(&x + (8 * &idx)), &y, &c, s.offset(&idx * 16));
//                idx += 1;
//            }
//        }
//    }
    pub fn putfont_asc(&self, x: u16, y: u16, c: u8) {
        let mut idx: isize = 0x5a;
        self.putfont(&x, &y, &c, (&idx * 16) as usize);
    }

//    fn putfont(&self, x: &u16, y: &u16, c: &u8, font_ptr: u8) {
    fn putfont(&self, x: &u16, y: &u16, c: &u8, font_ptr: usize) {
        // let idx: usize = 0x10;
        self.putfont_color(x, y, c, font_ptr);
    }

    // #[inline(always)]
    fn putfont_color(&self, x: &u16, y: &u16, c: &u8, idx: usize) {
        for i in 0..16 {
            let mut address = (self.boot_info.vram + ((y + i) * self.boot_info.scrnx + x) as u32) as *mut u8;
            let d: u8 = self.hankaku[idx + i as usize];
            unsafe {
                if (d & 0x80) != 0 { *(address.offset(0)) = *c }
                if (d & 0x40) != 0 { *(address.offset(1)) = *c }
                if (d & 0x20) != 0 { *(address.offset(2)) = *c }
                if (d & 0x10) != 0 { *(address.offset(3)) = *c }
                if (d & 0x08) != 0 { *(address.offset(4)) = *c }
                if (d & 0x04) != 0 { *(address.offset(5)) = *c }
                if (d & 0x02) != 0 { *(address.offset(6)) = *c }
                if (d & 0x01) != 0 { *(address.offset(7)) = *c }
            }
        }
    }
}

enum RGB {
    Black,          /*  0:黒 */
    Red,            /*  1:明るい赤 */
    Green,          /*  2:明るい緑 */
    Yellow,         /*  3:明るい黄色 */
    Blue,           /*  4:明るい青 */
    Purple,         /*  5:明るい紫 */
    LightBlue,      /*  6:明るい水色 */
    White,          /*  7:白 */
    Gray,           /*  8:明るい灰色 */
    DarkRed,        /*  9:暗い赤 */
    DarkGreen,      /* 10:暗い緑 */
    DarkYellow,     /* 11:暗い黄色 */
    DarkBlue,       /* 12:暗い青 */
    DarkPurple,     /* 13:暗い紫 */
    DarkLightBlue,  /* 14:暗い水色 */
    DarkGray,       /* 15:暗い灰色 */
}

impl RGB {
    fn value(&self) -> RGBElement {
        match self {
            RGB::Black => RGBElement::new(0x00, 0x00, 0x00),
            RGB::Red => RGBElement::new(0xff, 0x00, 0x00),
            RGB::Green => RGBElement::new(0x00, 0xff, 0x00),
            RGB::Yellow => RGBElement::new(0xff, 0xff, 0x00),
            RGB::Blue => RGBElement::new(0x00, 0x00, 0xff),
            RGB::Purple => RGBElement::new(0xff, 0x00, 0xff),
            RGB::LightBlue => RGBElement::new(0x00, 0xff, 0xff),
            RGB::White => RGBElement::new(0xff, 0xff, 0xff),
            RGB::Gray => RGBElement::new(0xc6, 0xc6, 0xc6),
            RGB::DarkRed => RGBElement::new(0x84, 0x00, 0x00),
            RGB::DarkGreen => RGBElement::new(0x00, 0x84, 0x00),
            RGB::DarkYellow => RGBElement::new(0x84, 0x84, 0x00),
            RGB::DarkBlue => RGBElement::new(0x00, 0x00, 0x84),
            RGB::DarkPurple => RGBElement::new(0x84, 0x00, 0x84),
            RGB::DarkLightBlue => RGBElement::new(0x00, 0x84, 0x84),
            RGB::DarkGray => RGBElement::new(0x84, 0x84, 0x84),
        }
    }

    fn palette_no(&self) -> u8 {
        match self {
            RGB::Black => 0,
            RGB::Red => 1,
            RGB::Green => 2,
            RGB::Yellow => 3,
            RGB::Blue => 4,
            RGB::Purple => 5,
            RGB::LightBlue => 6,
            RGB::White => 7,
            RGB::Gray => 8,
            RGB::DarkRed => 9,
            RGB::DarkGreen => 10,
            RGB::DarkYellow => 11,
            RGB::DarkBlue => 12,
            RGB::DarkPurple => 13,
            RGB::DarkLightBlue =>14,
            RGB::DarkGray => 15,
        }
    }

    fn r(&self) -> u8 { self.value().r }

    fn g(&self) -> u8 { self.value().g }

    fn b(&self) -> u8 { self.value().b }

    fn iterator() -> Iter<'static, RGB> {
        static RGBS: [RGB; 16] = [
            RGB::Black,          /*  0:黒 */
            RGB::Red,            /*  1:明るい赤 */
            RGB::Green,          /*  2:明るい緑 */
            RGB::Yellow,         /*  3:明るい黄色 */
            RGB::Blue,           /*  4:明るい青 */
            RGB::Purple,         /*  5:明るい紫 */
            RGB::LightBlue,      /*  6:明るい水色 */
            RGB::White,          /*  7:白 */
            RGB::Gray,           /*  8:明るい灰色 */
            RGB::DarkRed,        /*  9:暗い赤 */
            RGB::DarkGreen,      /* 10:暗い緑 */
            RGB::DarkYellow,     /* 11:暗い黄色 */
            RGB::DarkBlue,       /* 12:暗い青 */
            RGB::DarkPurple,     /* 13:暗い紫 */
            RGB::DarkLightBlue,  /* 14:暗い水色 */
            RGB::DarkGray,       /* 15:暗い灰色 */
        ];
        return RGBS.into_iter();
    }
}

struct RGBElement {
    r: u8,
    g: u8,
    b: u8,
}

impl RGBElement {
    fn new(r: u8, g: u8, b: u8) -> Self {
        RGBElement {
            r,
            g,
            b,
        }
    }
}