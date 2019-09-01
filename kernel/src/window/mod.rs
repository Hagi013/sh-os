use alloc::string::{String, ToString};
use core::mem::transmute_copy;
use core::mem::replace;
use core::intrinsics::transmute;

use super::sync::queue::SimpleQueue;

// SimpleQueueのCAPACITYと同じ値
const MAX_SHEETS: i16 = 30;
const ADR_BOOTINFO: u32 = 0x00000ff0;
const SCRNX: u16 = *(ADR_BOOTINFO + 0x04);
const SCRNY: u16 = *(ADR_BOOTINFO + 0x06);

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
    head: Option<*mut Window>,
    tail: Option<*mut Window>,
    count: i16,
    windows_map: [u8; (SCRNX * SCRNY + SCRNX) as usize],
    windows_order: SimpleQueue<*mut Window>,
}

impl WindowsManager {
    pub fn new() -> Self {
        WindowsManager {
            head: None,
            tail: None,
            count: 0,
            windows_map: [0; (SCRNX * SCRNY + SCRNX) as usize],
            windows_order: SimpleQueue::new(),
        }
    }

    pub fn count(&self) -> i16 {
        self.count
    }

    pub fn add(&mut self, mut n_w: Window) -> Result<(), String> {
        if self.count == MAX_SHEETS {
            return Err("windows count execeed MAX_SHEETS.".to_string());
        }

        unsafe {
            let n_w_pointer: *mut Window = &mut n_w as *mut Window;
            if let Some(window) = self.tail {
                self.tail = Some(n_w_pointer);
                n_w.next = None;
                (*window).next = Some(n_w_pointer);
                n_w.prev = Some(window);
            } else {
                self.tail = Some(n_w_pointer);
                if self.count == 0 && self.head.is_none() {
                    self.head = Some(n_w_pointer);
                    n_w.prev = None;
                    n_w.next = None;
                }
            }
            self.count += 1;
            // Windowを作成した時は先頭に追加するようにする
            self.windows_order.add(n_w_pointer, 0);
            return Ok(());
        }
    }

    pub fn remove(&mut self, mut t_w: Window) -> Result<(), String> {
        if self.count == 0 {
            return Err("Window Manager's count is 0.".to_string());
        }
        self.count -= 1;
        if self.count == 0 {
            self.head = None;
            self.tail = None;
            let w_pointer: *mut Window = &mut t_w as *mut Window;
            self.windows_order.remove_entry(w_pointer);
            return Ok(());
        }

        unsafe {
            let w_pointer: *mut Window = &mut t_w as *mut Window;

            if self.head.is_some() && self.tail.is_some() { // 基本的に要素が存在する場合は、headとtailは存在するはず
                let head: *mut Window = self.head.unwrap();
                let tail: *mut Window = self.tail.unwrap();

                if w_pointer != head && w_pointer != tail { // headとtailが今回の削除対象ではない場合
                    if let Some(mut prev) = (*w_pointer).prev {
                        // prevのnextを更新
                        // (*prev).next = (*w_pointer).next;
                        replace(&mut (*prev).next, replace(&mut (*w_pointer).next, None));
                    }
                    if let Some(mut next) = (*w_pointer).next {
                        replace(&mut (*next).prev, replace(&mut (*w_pointer).prev, None));
                    }
                    // 後処理(これは必要なのか？)
                    replace(&mut (*w_pointer).prev, None);
                    replace(&mut (*w_pointer).next, None);
                } else if w_pointer == head { // headが削除対象の場合
                    // *self.head = (*w_pointer).next;
                    replace(&mut self.head, replace(&mut (*w_pointer).next, None));
                    replace(&mut (*self.head.unwrap()).prev, None);
                } else if w_pointer == tail { // tailが削除対象の場合
                    replace(&mut self.tail, replace(&mut (*w_pointer).prev, None));
                    replace(&mut (*self.tail.unwrap()).next, None);
                } else {
                    self.head = None;
                    self.tail = None;
                }
                self.windows_order.remove_entry(w_pointer);
                return Ok(());
            } else {
                return Err("Element in WindowsManager is null.".to_string());
            }
            // ここに来ることはない
            return Ok(());
        }
    }

    pub fn create_window(&mut self, base_x: u16, base_y: u16, xsize: u16, ysize: u16, buf: *mut u8) -> Window {
        let mut n_w = Window::new(base_x, base_y, xsize, ysize, buf);
        self.add(n_w);
        self.refresh_map(base_x, base_y, xsize, ysize, n_w);
        return n_w;
    }

    pub fn refresh_map(&mut self, base_x: u16, base_y: u16, xsize: u16, ysize: u16, mut w_r: Window) {
        let p_w_r = &mut w_r as *mut Window;
        let (from_x, from_y, to_x, to_y) = self.check_size(base_x, base_y, xsize, ysize);
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
    priority: u16,
    buf: *mut u8,
    pub prev: Option<*mut Window>,
    pub next: Option<*mut Window>,
}

impl Window {
    pub fn new(base_x: u16, base_y: u16, xsize: u16, ysize: u16, buf: *mut u8) -> Window {
        Window {
            base_x,
            base_y,
            xsize,
            ysize,
            buf,
            priority: 0,
            next: None,
            prev: None,
        }
    }

//    fn create_new_window(base_x: u16, base_y: u16, xsize: u16, ysize: u16, buf: *mut u8) -> Window<'a> {
//        Window {
//            base_x,
//            base_y,
//            xsize,
//            ysize,
//            buf,
//            height: 0,
//            next: None,
//            prev: None,
//        }
//    }
}
