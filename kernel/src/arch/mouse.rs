use core::ptr;

use super::asmfunc;

use super::pic::PIC1_IMR;
use super::pic::PIC0_OCW2;
use super::pic::PIC1_OCW2;
use super::pic::PORT_KEYDAT;
use super::pic::KEYCMD_SENDTO_MOUSE;
use super::pic::MOUSECMD_ENABLE;

use super::pic::wait_kbc_sendready;

use super::super::queue::SimpleQueue;

use super::graphic::Printer;
use core::fmt::Write;
use crate::arch::pic::PORT_KEYCMD;

static mut MOUSE_QUEUE: Option<SimpleQueue<i32>> = None;

pub fn allow_mouse_int() {
    asmfunc::io_out8(PIC1_IMR, 0xef);
    wait_kbc_sendready();
    asmfunc::io_out8(PORT_KEYCMD, KEYCMD_SENDTO_MOUSE);
    wait_kbc_sendready();
    asmfunc::io_out8(PORT_KEYDAT, MOUSECMD_ENABLE);
    let queue: SimpleQueue<i32> = SimpleQueue::new();
    unsafe { MOUSE_QUEUE = Some(queue); }
}

#[no_mangle]
pub extern "C" fn inthandler2c(esp: *const usize) {
    asmfunc::io_out8(PIC1_OCW2, 0x64);
    asmfunc::io_out8(PIC0_OCW2, 0x62);
    let data: i32 = asmfunc::io_in8(PORT_KEYDAT);
    let mut printer = Printer::new(310, 400, 10);
    write!(printer, "{:?}", data).unwrap();
    unsafe {
        match ptr::read(&MOUSE_QUEUE) {
            Some(mut queue) => {
                queue.enqueue(data);
                MOUSE_QUEUE = Some(queue);
            },
            None => {
                let mut queue: SimpleQueue<i32> = SimpleQueue::new();
                queue.enqueue(data);
                MOUSE_QUEUE = Some(queue);
            }
        }
    }
}
