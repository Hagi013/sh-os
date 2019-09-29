use alloc::string::{String, ToString};
use alloc::vec::Vec;

use core::mem::replace;

use super::arch::boot_info::BootInfo;

use super::Printer;
use core::fmt::Write;

use super::util::linked_list::LinkedList;
use core::borrow::Borrow;
use alloc::borrow::ToOwned;

/* ToDo
    0. windowを受け取った時のrendering
        - 実際の色付けや位置決めはwindowの管理外で、window.bufに含める
        - buf配列要素に対する添字(アドレス)は、それぞれbase_x, base_y, xsize, ysizeで決まる
        - 受け取った段階で一番上にrenderingさせる
    1. windowの追加時のrendering
    2. slide時のrendering
    3. top down時のrendering
        - どうやって順番を管理するか
*/

pub struct WindowsManager {
    linked_list: LinkedList<Window>,
    windows_map: Vec<*const u8>,
    scrnx: u16,
    scrny: u16,
    vram: u32,
}

impl WindowsManager {
    pub fn new() -> Self {
        let b: BootInfo = BootInfo::new();
        let mut window_size: usize = 0;
        window_size = (b.scrnx as usize) * (b.scrny as usize) + (b.scrnx as usize);
        WindowsManager {
            linked_list: LinkedList::new(),
            windows_map: vec![0 as *const u8; window_size],
            scrnx: b.scrnx,
            scrny: b.scrny,
            vram: b.vram,
        }
    }

    pub fn create_window(&mut self, base_x: i32, base_y: i32, xsize: u16, ysize: u16, buf: *mut u8) -> Result<Window, String> {
        let mut n_w = Window::new(base_x, base_y, xsize, ysize, buf);
        self.add(n_w);
        let height=  self.linked_list.get_position_from_data(n_w).ok_or("add method is not executed properly in create_window".to_string())?;

        self.refresh_map(base_x, base_y, &mut n_w, height);
        return Ok(n_w);
    }

//    pub fn close_window(&mut self, buf: *mut u8) -> Result<(), String> {
//    }

    // addするときは一番最後に入れる
    fn add(&mut self, mut n_w: Window) -> Result<(), String> {
        return self.linked_list.add(n_w);
    }

    pub fn remove(&mut self, mut t_w: Window) -> Result<(), String> {
        return self.linked_list.remove(t_w);
    }

    pub fn refresh_map(&mut self, base_x: i32, base_y: i32, w_r: &mut Window, height: usize) -> Result<(), String> {
        let (from_x, from_y, to_x, to_y) = self.get_adjusted_position(base_x, base_y, w_r.xsize, w_r.ysize);
        for h in height..self.linked_list.len() {
            let w = &mut self.linked_list.get_data_from_position(h).ok_or("".to_string())? as *mut Window;
            for y in from_y..to_y {
                for x in from_x..to_x {
                    self.windows_map[(y as usize) * (self.scrnx as usize) + (x as usize)] = w as *const u8;
                }
            }
        }
        Ok(())
    }

    pub fn refresh_windows(&mut self, base_x: i32, base_y: i32, w: &mut Window, from_height: usize, to_height: usize) -> Result<(), String> {
        let (from_x, from_y, to_x, to_y) = self.get_adjusted_position(base_x, base_y, w.xsize, w.ysize);
        let linked_list_len: usize = self.linked_list.get_position_from_data(w.to_owned()).ok_or("this window is not existing in LinkedList.".to_string())?;
        let height: usize = if to_height > linked_list_len { linked_list_len } else { to_height + 1 };
        let mut c = 0;
        for h in from_height..height {
            let current_window_ptr: *const Window = &mut self.linked_list.get_data_from_position(h).ok_or("get_data_from_position is error in refresh_windows")? as *const Window;
            for y in from_y..to_y {
                for x in from_x..to_x {
                    let mut address = ((self.vram) + (y as u32) * (self.scrnx as u32) + (x as u32)) as *mut u8;
                    unsafe {
                        *address = *(*current_window_ptr).buf.offset(((y - from_y) * self.scrnx + (x - from_x)) as isize);
//                        let mut printer = Printer::new(500, 560, 10);
//                        write!(printer, "{:?}", (*current_window_ptr).buf.offset(((y - from_y) * self.scrnx + (x - from_x)) as isize)).unwrap();
                        let mut printer = Printer::new(x as u32, y as u32, 10);
                        write!(printer, "{:?}", 1).unwrap();
                    }
//                    }
                }
            }
        }
        return Ok(());
    }

    pub fn move_window(&mut self, w: &mut Window, mut value_x: i32, value_y: i32) -> Result<Window, String> {
        // i32で保持されているデータだが、なぜかマイナスとして保持されておらず、16bit分だけ
        // コピーして、しっかりマナスとして判定させる
        // ToDo 今後根本となる原因を修正する必要はあるが(おそらくio_in8の返却がi32で返すところでおかしくなってる？)、
        //      一旦画面の大きさ的にx軸とy軸とで16bitの最大値を超えることはないため、こうする
        let x: i32 = 0x0000 + (value_x as i16) as i32;
        let y: i32 = 0x0000 + (value_y as i16) as i32;

        let mut printer = Printer::new(w.base_x as u32, w.base_y as u32, 10);
        write!(printer, "{:?}", 1).unwrap();

//        let mut printer = Printer::new(500, 265, 10);
//        write!(printer, "{:?}", w.base_y).unwrap();

        let mut mx: i32 = w.base_x + x;
        mx = if mx < 0 {
            0
        } else if mx > unsafe { self.scrnx as i32 - 1 } {
            unsafe { self.scrnx as i32 - 1 }
        } else {
            x + w.base_x
        };

        let mut my: i32 = w.base_y + y;
        my = if my < 0 {
            0
        } else if my > unsafe { self.scrny as i32 - 1 } {
            unsafe { self.scrny as i32 - 1 }
        } else {
            y + w.base_y
        };

        let old_base_x: i32 = w.base_x;
        let old_base_y: i32 = w.base_y;

        w.set_base_x(mx);
        w.set_base_y(my);

        let height = self.linked_list.get_position_from_data(w.to_owned()).ok_or("In move_window, window is not existing.".to_string())?;
        self.refresh_map(old_base_x, old_base_y, w, 0);
        self.refresh_map(mx, my, w, height);

        let to_height = if height == 0 { 0 } else { height };
        self.refresh_windows(old_base_x, old_base_y, w, 0, to_height);
        self.refresh_windows(mx, my, w, height, height);
        return Ok(w.to_owned());
    }

    fn get_adjusted_position(&self, base_x: i32, base_y: i32, xsize: u16, ysize: u16) -> (u16, u16, u16, u16) {
        let from_x = if base_x < 0 { 0 as u16 } else { base_x as u16 };
        let from_y = if base_y < 0 { 0 as u16 } else { base_y as u16};
        unsafe {
            let to_x: u16 = if from_x + xsize > self.scrnx { self.scrnx } else { from_x + xsize };
            let to_y: u16 = if from_y + ysize > self.scrny { self.scrny } else { from_y + ysize };
            return (from_x, from_y, to_x, to_y);
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Window {
    pub base_x: i32,
    base_y: i32,
    xsize: u16,
    ysize: u16,
    buf: *mut u8,
}

impl Window {
    pub fn new(base_x: i32, base_y: i32, xsize: u16, ysize: u16, buf: *mut u8) -> Window {
        Window {
            base_x,
            base_y,
            xsize,
            ysize,
            buf,
        }
    }

    fn set_base_x(&mut self, x: i32) {
        self.base_x = x;
    }

    fn set_base_y(&mut self, y: i32) {
        self.base_y = y;
    }
}

impl core::cmp::PartialEq<Window> for Window {
    #[inline]
    fn eq(&self, other: &Window) -> bool {
        self.base_x == other.base_x &&
        self.base_y == other.base_y &&
        self.xsize == other.xsize &&
        self.ysize == other.ysize &&
        self.buf == other.buf
    }

    #[inline]
    fn ne(&self, other: &Window) -> bool {
        self.base_x != other.base_x ||
        self.base_y != other.base_y ||
        self.xsize != other.xsize ||
        self.ysize != other.ysize ||
        self.buf != other.buf
    }

}