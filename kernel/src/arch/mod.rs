pub mod asmfunc;
pub mod graphic;
pub mod boot_info;
pub mod hankaku;

extern "C" {
    pub fn io_store_eflags(eflags: u32);
    pub fn io_load_eflags();
}
