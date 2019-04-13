pub fn io_hlt() {
    unsafe {
        asm!("
        hlt
        " :::: "intel");
    }
}

pub fn io_cli() {
    unsafe {
        asm!("
        cli
        " :::: "intel");
    }
}

pub fn io_sti() {
    unsafe {
        asm!("
        sti
        " :::: "intel");
    }
}

pub fn io_stihlt() {
    io_sti();
    io_hlt();
}

pub fn io_in8(port: i32) -> i32 {
    let res: i32;
    unsafe {
        asm!("
        mov eax, 0
        in al, dx"
        : "={eax}"(res) // Output Operand
        : "{edx}"(port) // Input Operand
        : "memory"      // 変更される可能性があるレジスタ
        : "intel");     // Option
    }
    return res;
}

pub fn io_out8(port: i32, data: i32) {
    unsafe {
        asm!("
        out dx, al
        "
        : // output
        : "{edx}"(port), "{al}"(data) // input
        :
        : "intel");
    }
}

pub fn io_load_eflags() -> u32 {
    let eflags: u32;
    unsafe {
        asm!("
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

pub fn io_store_eflags(eflags: u32) {
    unsafe {
        asm!("
        push $0
        popfd
        "
        :
        : "r"(eflags)
        : "cc"
        : "intel");
    }
}

pub fn load_gdtr(limit: u32, addr: u32) {
    let load_address: u32 = limit + addr;
    unsafe {
        asm!("
        lgdt eax
        "
        :
        : "{eax}"(load_address)
        :
        : "intel");
    }
}

pub fn load_idtr(limit: u32, addr: u32) {
    let load_address: u32 = limit + addr;
    unsafe {
        asm!("
        lidt eax
        "
        :
        : "{eax}"(load_address)
        :
        : "intel");
    }
}

pub fn load_cr0() -> u32 {
    let cr0: u32;
    unsafe {
        asm!("
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
        asm!("
        mov cr0, eax
        "
        :
        : "{eax}"(cr0)
        :
        : "intel");
    }
}

pub fn load_tr(tr: u32) {
    unsafe {
        asm!("
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
        asm!("
        jmp far [exp+4]
        "
        :
        :
        :
        : "intel");
    }
}
