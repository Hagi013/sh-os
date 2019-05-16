use super::asmfunc;
use super::graphic;

const PIC0_ICW1: i32 = 0x0020;
const PIC0_OCW2: i32 = 0x0020;
const PIC0_IMR:  i32 = 0x0021;
const PIC0_ICW2: i32 = 0x0021;
const PIC0_ICW3: i32 = 0x0021;
const PIC0_ICW4: i32 = 0x0021;
const PIC1_ICW1: i32 = 0x00a0;
const PIC1_OCW2: i32 = 0x00a0;
const PIC1_IMR:  i32 = 0x00a1;
const PIC1_ICW2: i32 = 0x00a1;
const PIC1_ICW3: i32 = 0x00a1;
const PIC1_ICW4: i32 = 0x00a1;

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

pub fn inthandler21(exp: u32) {

//    loop {
//        asmfunc::io_hlt();
//    }
}