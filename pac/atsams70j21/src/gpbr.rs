#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Backup Register 0"]
    pub sys_gpbr: [SYS_GPBR; 8],
}
#[doc = "General Purpose Backup Register 0"]
pub struct SYS_GPBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Backup Register 0"]
pub mod sys_gpbr;
