pub mod asmfunc;
pub mod graphic;
pub mod boot_info;


extern "C" {
    pub fn io_store_eflags(eflags: u32);
    pub fn io_load_eflags();
}
