pub fn io_hlt() {
    unsafe {
        llvm_asm!("
        hlt
        " :::: "intel");
    }
}

pub fn io_cli() {
    unsafe {
        llvm_asm!("
        cli
        " :::: "intel");
    }
}

pub fn io_sti() {
    unsafe {
        llvm_asm!("
        sti
        " :::: "intel");
    }
}

pub fn io_stihlt() {
    io_sti();
    io_hlt();
}

pub fn io_in8(port: i32) -> i32 {
    let mut res: i32 = 0;
    unsafe {
        llvm_asm!("
        mov eax, 0
        in al, dx"
        : "={eax}"(res) // Output Operand
        : "{edx}"(port) // Input Operand
        : "memory"      // 変更される可能性があるレジスタ
        : "intel");     // Option
    }
    return res;
}

//pub fn io_out8(port: i32, data: i32) {
pub fn io_out8(port: i32, data: u8) {
    unsafe {
        llvm_asm!("
        out dx, al
        "
        : // output
        : "{edx}"(port), "{al}"(data) // input
        :
        : "intel");
    }
}

#[cfg(all(not(test)))]
pub fn io_load_eflags() -> u32 {
    let mut eflags: u32 = 0;
    unsafe {
        llvm_asm!("
        pushfd
        pop $0
        "
        : "=r"(eflags)
        :
        :
        : "intel");
    }
    return eflags;
}

#[cfg(all(not(test)))]
pub fn io_store_eflags(eflags: u32) {
    unsafe {
        llvm_asm!("
        push $0
        popfd
        "
        :
        : "r"(eflags)
        : "cc"
        : "intel");
    }
}

#[cfg(all(test))]
pub fn io_load_eflags() -> u32 {
    let mut eflags: u32 = 0;
    unsafe {
        llvm_asm!("
        pushfq
        pop $0
        "
        : "=r"(eflags)
        :
        :
        : "intel");
    }
    return eflags;
}

#[cfg(all(test))]
pub fn io_store_eflags(eflags: u32) {
    unsafe {
        llvm_asm!("
        push $0
        popfq
        "
        :
        : "r"(eflags)
        : "cc"
        : "intel");
    }
}

pub fn load_gdtr(limit: u32, addr: u32) {
    unsafe {
        llvm_asm!("
        mov eax, $0
        mov [esp+6], ax
        mov eax, $1
        mov [esp+8], eax
        lgdt [esp+6]
        "
        :
        : "r"(limit),"r"(addr)
        : "memory"
        : "intel");
    }
}

pub fn load_idtr(limit: u32, addr: u32) {
    unsafe {
        llvm_asm!("
        mov eax, $0
        mov [esp+6], ax
        mov eax, $1
        mov [esp+8], eax
        lidt [esp+6]
        "
        :
        : "r"(limit),"r"(addr)
        : "memory"
        : "intel");
    }
}

pub fn load_cr0() -> u32 {
    let mut cr0: u32 = 0;
    unsafe {
        llvm_asm!("
        mov eax, cr0
        "
        : "={eax}"(cr0)
        :
        :
        : "intel");
    }
    return cr0;
}

pub fn store_cr0(cr0: u32) {
    unsafe {
        llvm_asm!("
        mov cr0, eax
        "
        :
        : "{eax}"(cr0)
        :
        : "intel");
    }
}

pub fn set_pg_flag() {
    unsafe {
        // llvm_asm!("
        //     push eax
        //     mov eax, cr0
        //     or eax, 0x80000000
        //     mov cr0, eax
        //     pop eax
        //     jmp .a
        // .a:
        //     ret
        // "
        // :
        // :
        // :
        // : "intel");
        llvm_asm!("
            push eax
            mov eax, cr0
            or eax, 0x80000000
            mov cr0, eax
            pop eax
            ret
        "
        :
        :
        :
        : "intel");
    }
}

pub fn load_cr1() -> u32 {
    let mut cr1: u32 = 0;
    unsafe {
        llvm_asm!("
        mov eax, cr1
        "
        : "={eax}"(cr1)
        :
        :
        : "intel");
    }
    return cr1;
}

pub fn store_cr1(cr1: u32) {
    unsafe {
        llvm_asm!("
        mov cr1, eax
        "
        :
        : "{eax}"(cr1)
        :
        : "intel");
    }
}

pub fn load_cr2() -> u32 {
    let mut cr2: u32 = 0;
    unsafe {
        llvm_asm!("
        mov eax, cr2
        "
        : "={eax}"(cr2)
        :
        :
        : "intel");
    }
    return cr2;
}

pub fn store_cr2(cr2: u32) {
    unsafe {
        llvm_asm!("
        mov cr2, eax
        "
        :
        : "{eax}"(cr2)
        :
        : "intel");
    }
}

pub fn load_cr3() -> u32 {
    let mut cr3: u32 = 0;
    unsafe {
        llvm_asm!("
        mov eax, cr3
        "
        : "={eax}"(cr3)
        :
        :
        : "intel");
    }
    return cr3;
}

pub fn store_cr3(cr3: u32) {
    unsafe {
        llvm_asm!("
        mov cr3, eax
        "
        :
        : "{eax}"(cr3)
        :
        : "intel");
    }
}

pub fn load_tr(tr: u32) {
    unsafe {
        llvm_asm!("
        ltr eax
        "
        :
        : "{eax}"(tr)
        :
        : "intel");
    }
}

pub fn farjmp(eip: u32, _cs: u32) {
    unsafe {
        llvm_asm!("
        jmp far [esp+4]
        "
        :
        :
        :
        : "intel");
    }
}

//pub fn set_asm_inthandler(handler: *const u32) -> *const u32 {
//    let f = || asm_inthandler(handler);
//    return f as *const u32
//}

//pub fn asm_inthandler(handler: *const u32) {
//    unsafe {
//        llvm_asm!("
//            push es
//            push ds
//            pushad
//            mov eax, esp
//            push eax
//            mov ax, ss
//            mov ds, ax
//            mov es, ax
//            call %0
//            pop eax
//            popad
//            pop ds
//            pop es
//            iret
//            "
//            :
//            : "r"(handler)
//            :
//            : "intel")
//    }
//}
