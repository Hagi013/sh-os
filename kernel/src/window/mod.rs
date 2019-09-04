use alloc::string::{String, ToString};
use alloc::vec::Vec;

use core::mem::replace;

use super::arch::boot_info::BootInfo;

use super::Printer;
use core::fmt::Write;

use super::util::linked_list::LinkedList;
use core::borrow::Borrow;

static mut SCRNX: u16 = 320;
static mut SCRNY: u16 = 240;
static mut VRAM: u32 = 0x000a0000;


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
}

impl WindowsManager {
    pub fn new() -> Self {
        let b: BootInfo = BootInfo::new();
        let mut window_size: usize = 0;
        unsafe {
            SCRNX = b.scrnx;
            SCRNY = b.scrny;
            VRAM =  b.vram;
            window_size = (SCRNX as usize) * (SCRNY as usize) + (SCRNX as usize);
        }
        WindowsManager {
            linked_list: LinkedList::new(),
            windows_map: vec![0 as *const u8; window_size],
        }
    }

    pub fn create_window(&mut self, base_x: u16, base_y: u16, xsize: u16, ysize: u16, buf: *mut u8) -> Result<Window, String> {
        let mut n_w = Window::new(base_x, base_y, xsize, ysize, buf);
        self.add(n_w);
        let height=  self.linked_list.get_position_from_data(n_w).ok_or("add method is not executed properly in create_window".to_string())?;

        self.refresh_map(base_x, base_y, xsize, ysize, n_w, height);
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

    // ToDo
    // 1. どの高さのwindowから更新が必要かを明示できる方が効率的なので、高さを指定できるようにする
    // 2. 更新する枠を最小限にするために、更新する部分のみ書き換えを行えるようにする
    pub fn refresh_map(&mut self, base_x: u16, base_y: u16, xsize: u16, ysize: u16, mut w_r: Window, height: usize) {
        let p_w_r = &mut w_r as *mut Window;
        let (from_x, from_y, to_x, to_y) = self.check_size(base_x, base_y, xsize, ysize);
        for h in height..self.linked_list.len() {
            for y in 0..from_y {
                let vy: usize = (to_y + y) as usize;
                for x in 0..from_x {
                    let vx: usize = (to_x + x) as usize;
                    unsafe {
                        self.windows_map[vy * (SCRNX as usize) + vx] = p_w_r as *const u8;
                    }
                }
            }
        }
    }

    pub fn move_window(&mut self, mut w: Window, x: u16, y: u16) {

    }

    fn check_size(&self, base_x: u16, base_y: u16, xsize: u16, ysize: u16) -> (u16, u16, u16, u16) {
        let from_x = if base_x < 0 { 0 } else { base_x };
        let from_y = if base_y < 0 { 0 } else { base_y };
        unsafe {
            let to_x = if base_x + xsize > SCRNX { SCRNX } else { base_x + xsize };
            let to_y = if base_y + ysize > SCRNY { SCRNY } else { base_y + ysize };
            return (from_x, from_y, to_x, to_y);
        }
    }
}


#[derive(Copy, Clone)]
pub struct Window {
    base_x: u16,
    base_y: u16,
    xsize: u16,
    ysize: u16,
    buf: *mut u8,
}

impl Window {
    pub fn new(base_x: u16, base_y: u16, xsize: u16, ysize: u16, buf: *mut u8) -> Window {
        Window {
            base_x,
            base_y,
            xsize,
            ysize,
            buf,
        }
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