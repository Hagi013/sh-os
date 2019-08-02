use core::ptr;

use super::asmfunc;
use super::graphic::Graphic;

use super::pic::PIC0_IMR;
use super::pic::PIC0_OCW2;
use super::pic::PIC1_IMR;
use super::pic::PORT_KEYDAT;

use super::super::queue::SimpleQueue;
use alloc::borrow::ToOwned;

use super::super::Printer;
use core::fmt::Write;

static mut KEYBOARD_QUEUE: Option<SimpleQueue<i32>> = None;

/* PIC1とキーボードを許可(11111001) */
pub fn allow_pic1_keyboard_int() {
    asmfunc::io_out8(PIC0_IMR, 0xf9);
    unsafe {
        let queue: SimpleQueue<i32> = SimpleQueue::new();
        KEYBOARD_QUEUE = Some(queue);
    };
}

#[no_mangle]
pub extern "C" fn inthandler21(exp: *const u32) {
    asmfunc::io_out8(PIC0_OCW2, 0x61);
    let data: i32 = asmfunc::io_in8(PORT_KEYDAT);

    if data >= 0x80 { return; }

    unsafe {
        match ptr::read(&KEYBOARD_QUEUE) {
            Some(mut queue) => {
                queue.enqueue(data);
                KEYBOARD_QUEUE = Some(queue);
            },
            None => {
                let mut queue: SimpleQueue<i32> = SimpleQueue::new();
                queue.enqueue(data);
                KEYBOARD_QUEUE = Some(queue);
            },
        }
    }
}

pub fn is_existing() -> bool {
    let mut printer = Printer::new(310, 400, 10);
    write!(printer, "{:?}", unsafe { ptr::read(&KEYBOARD_QUEUE).unwrap().is_existing() }).unwrap();
    unsafe {
        match ptr::read(&KEYBOARD_QUEUE) {
            Some(checker) => checker.is_existing(),
            None => false
        }
    }
}

pub fn get_data() -> Result<i32, ()> {
    unsafe {
        if let Some(mut queue) = ptr::read(&KEYBOARD_QUEUE) {
            let data = queue.dequeue().unwrap();
            KEYBOARD_QUEUE = Some(queue);
            Ok(data)
        } else {
            KEYBOARD_QUEUE = Some(SimpleQueue::new());
            return Err(());
        }
    }
}