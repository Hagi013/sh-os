pub mod asmfunc;
pub mod graphic;
pub mod boot_info;


pub struct Test {
    color: u8,
}

impl Test {
    pub fn new(c: u8) -> Self {
        Test {
            color: c,
        }
    }

    pub fn echo(&self) {
//        let mut address = 0x000a0000 as u32;
//        let mut address = 0x000ffff as u32;
        let mut address = 0x000a0000 as u32;
        let raw = &mut address as *mut u32;
        let memory_address: u32 = unsafe { *raw };

//        for i in 0..0xfffffff {
        for i in 0..(0xbffff - address) {
            unsafe {
                let memory: *mut u8 = (memory_address + i) as *mut u8;
//            *memory = (*(i as *mut u8) & 0x0f);
//            *memory = (*(i as *mut u8));
//                *memory = 0x4d;
                *memory = self.color
            }
        }
    }
}

// let test: Test = Test::new(0x19); // これだとなぜか青のdotが侵食を始める
//    let test: Test = Test::new(0x0e);
//    test.echo();
