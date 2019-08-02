use core::str;

use super::asmfunc;
use super::graphic::Graphic;
use crate::arch::asmfunc::io_out8;

pub const PIC0_ICW1: i32 = 0x0020;
pub const PIC0_OCW2: i32 = 0x0020;
pub const PIC0_IMR:  i32 = 0x0021;
pub const PIC0_ICW2: i32 = 0x0021;
pub const PIC0_ICW3: i32 = 0x0021;
pub const PIC0_ICW4: i32 = 0x0021;
pub const PIC1_ICW1: i32 = 0x00a0;
pub const PIC1_OCW2: i32 = 0x00a0;
pub const PIC1_IMR:  i32 = 0x00a1;
pub const PIC1_ICW2: i32 = 0x00a1;
pub const PIC1_ICW3: i32 = 0x00a1;
pub const PIC1_ICW4: i32 = 0x00a1;

pub const PORT_KEYDAT: i32 = 0x0060;

pub fn init_pic() {
    asmfunc::io_out8(PIC0_IMR, 0xff);          /* 全ての割り込みを受け付けない */
    asmfunc::io_out8(PIC1_IMR, 0xff);          /* 全ての割り込みを受け付けない */

    asmfunc::io_out8(PIC0_ICW1, 0x11);         /* エッジトリガモード */
    asmfunc::io_out8(PIC0_ICW2, 0x20);         /* IRQ0-7は、INT20-27で受ける */
    asmfunc::io_out8(PIC0_ICW3,  1 << 2);      /* PIC1はIRQ2にて接続 */
    asmfunc::io_out8(PIC0_ICW4, 0x01);         /* x86モードかつノンバッファモード */

    asmfunc::io_out8(PIC1_ICW1, 0x11);         /* エッジトリガモード */
    asmfunc::io_out8(PIC1_ICW2, 0x28);         /* IRQ8-15は、INT28-2fで受ける */
    asmfunc::io_out8(PIC1_ICW3, 2);            /* PIC0にはIRQ2で接続 */
    asmfunc::io_out8(PIC1_ICW4, 0x01);         /* x86モードかつノンバッファモード */

    asmfunc::io_out8(PIC0_IMR, 0xfb);          /* 11111011 PIC1以外は全て禁止 */
    asmfunc::io_out8(PIC1_IMR, 0xff)           /* 11111111 全ての割り込みを受け付けない */
}

/* マウスを許可(11101111) */
pub fn allow_mouse_int() {
    asmfunc::io_out8(PIC1_IMR, 0xef);
}

#[no_mangle]
pub extern "C" fn inthandler27(exp: *const u32) {
    /* PIC0からの不完全割り込み対策 */
    /* Athlon64X2機などではチップセットの都合によりPICの初期化時にこの割り込みが一度だけ起こる */
    /* この割り込み処理関数は、その割り込みに対して何もしないでやり過ごす */
    /* なぜ何もしなくていいの？
        -> この割り込みはPIC初期化時の電気的なノイズによって発生したものなので、
        真面目に何か処理してやる必要がない                                    */
    asmfunc::io_out8(PIC0_OCW2, 0x67);
}
