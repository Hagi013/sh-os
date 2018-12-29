
bits 32

global io_hlt, io_cli, io_sti, io_sticli
global io_in8, io_in16, io_in32
global io_out8, io_out16, io_out32
global io_load_eflags, io_store_eflags
global load_gdtr, load_idtr
global load_cr0, store_cr0
global load_tr
global far_jmp

section .text

io_hlt:
    hlt
    ret

io_cli:
    cli
    ret

io_sti:
    sti
    ret

io_stihlt:
    sti
    hlt
    ret

io_in8:     ; io_in8(port: u16)
    mov     edx, [esp+4]    ; port
    mov     eax, 0
    in      al, dx          ; dxで指定したポートからalにバイトを入力する
    ret

io_in16:     ; io_in8(port: u32)
    mov     edx, [esp+4]
    mov     eax, 0
    in      ax, dx          ; dxで指定したポートからaxにワードを入力する
    ret

io_in32:
    mov     edx, [esp+4]
    in      eax, dx         ; dxで指定したポートからeaxにダブルワードを入力する
    ret

io_out8:    ; io_out(port: u32, data: u8)
    mov     edx, [esp+4]    ; port
    mov     al, [esp+8]     ; data
    out     dx, al
    ret

io_out16:
    mov     edx, [esp+4]
    mov     ax, [esp+8]
    out     dx, ax
    ret

io_out32:
    mov     edx, [esp+4]
    mov     eax, [esp+4]
    out     dx, eax
    ret

io_load_eflags: ; io_load_eflags(void) -> u32
    pushfd
    pop     eax
    ret

io_store_eflags: ; io_store_eflags(eflags: u32)
    mov     eax, [esp+4]
    push    eax
    popfd
    ret

load_gdtr:  ; load_gdtr(limit: u32, addr: u32)
    mov     ax, [esp+4]     ; limit
    mov     [esp+6], ax
    lgdt    [esp+6]
    ret

load_idtr:  ; load_idtr(limit: u32, addr: u32)
    mov     ax, [esp+4]
    mov     [esp+6], ax
    lgdt    [esp+6]
    ret

load_cr0:   ; load_cr0(void) -> u32
    mov     eax, cr0
    ret

store_cr0:  ; store_cr0(cr0: u32)
    mov     eax, [esp+4]
    mov     cr0, eax
    ret

load_tr:    ; load_tr(tr: u32)
    ltr     [esp+4]
    ret

farjmp:     ; farjmp(eip: u32, cs: u32)
    jmp far [esp+4]     ; eip, cs. JMP FARはfar jmpさせるための命令。CPUは指定番地からまず4バイト読み込んんで、その値をEIPに入れる
    ret                 ; さらにその隣の2バイトも読み込んでCSに入れる
