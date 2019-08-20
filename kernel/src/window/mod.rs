use alloc::string::{String, ToString};
use core::mem::transmute_copy;
use core::mem::replace;
use core::intrinsics::transmute;

const MAX_SHEETS: i16 = 256;

pub struct WindowsManager {
    head: Option<*mut Window>,
    tail: Option<*mut Window>,
    count: i16,
}

impl WindowsManager {
    pub fn new() -> Self {
        WindowsManager {
            head: None,
            tail: None,
            count: 0,
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
            let n_w_pointer: *mut Window = transmute_copy::<Window, *mut Window>(&n_w);
            if let Some(window) = self.tail {
                self.tail = Some(n_w_pointer);
                n_w.next = None;
            } else {
                self.tail = Some(n_w_pointer);
                if self.count == 0 && self.head.is_none() {
                    self.head = Some(n_w_pointer);
                    n_w.prev = None;
                    n_w.next = None;
                }
            }
            self.count += 1;
            return Ok(());
        }
    }

    pub fn remove(&mut self, t_w: Window) -> Result<(), String> {
        if self.count == 0 {
            return Err("Window Manager's count is 0.".to_string());
        }
        self.count -= 1;
        if self.count == 0 {
            self.head = None;
            self.tail = None;
            return Ok(());
        }

        unsafe {
            let w_pointer: *mut Window = transmute_copy::<Window, *mut Window>(&t_w);

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
        return n_w;
    }
}


#[derive(Copy, Clone)]
pub struct Window {
    base_x: u16,
    base_y: u16,
    xsize: u16,
    ysize: u16,
    height: u16,
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
            height: 0,
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
