#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Setup Register"]
    pub smc_cs_number: [SMC_CS_NUMBER; 4],
    _reserved1: [u8; 64usize],
    #[doc = "0x80 - SMC Off-Chip Memory Scrambling Register"]
    pub smc_ocms: SMC_OCMS,
    #[doc = "0x84 - SMC Off-Chip Memory Scrambling KEY1 Register"]
    pub smc_key1: SMC_KEY1,
    #[doc = "0x88 - SMC Off-Chip Memory Scrambling KEY2 Register"]
    pub smc_key2: SMC_KEY2,
    _reserved4: [u8; 88usize],
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    pub smc_wpmr: SMC_WPMR,
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    pub smc_wpsr: SMC_WPSR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMC_CS_NUMBER {
    #[doc = "0x00 - SMC Setup Register"]
    pub smc_setup: self::smc_cs_number::SMC_SETUP,
    #[doc = "0x04 - SMC Pulse Register"]
    pub smc_pulse: self::smc_cs_number::SMC_PULSE,
    #[doc = "0x08 - SMC Cycle Register"]
    pub smc_cycle: self::smc_cs_number::SMC_CYCLE,
    #[doc = "0x0c - SMC Mode Register"]
    pub smc_mode: self::smc_cs_number::SMC_MODE,
}
#[doc = r"Register block"]
#[doc = "SMC Setup Register"]
pub mod smc_cs_number;
#[doc = "SMC Off-Chip Memory Scrambling Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_ocms](smc_ocms) module"]
pub type SMC_OCMS = crate::Reg<u32, _SMC_OCMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_OCMS;
#[doc = "`read()` method returns [smc_ocms::R](smc_ocms::R) reader structure"]
impl crate::Readable for SMC_OCMS {}
#[doc = "`write(|w| ..)` method takes [smc_ocms::W](smc_ocms::W) writer structure"]
impl crate::Writable for SMC_OCMS {}
#[doc = "SMC Off-Chip Memory Scrambling Register"]
pub mod smc_ocms;
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_key1](smc_key1) module"]
pub type SMC_KEY1 = crate::Reg<u32, _SMC_KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_KEY1;
#[doc = "`write(|w| ..)` method takes [smc_key1::W](smc_key1::W) writer structure"]
impl crate::Writable for SMC_KEY1 {}
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register"]
pub mod smc_key1;
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_key2](smc_key2) module"]
pub type SMC_KEY2 = crate::Reg<u32, _SMC_KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_KEY2;
#[doc = "`write(|w| ..)` method takes [smc_key2::W](smc_key2::W) writer structure"]
impl crate::Writable for SMC_KEY2 {}
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register"]
pub mod smc_key2;
#[doc = "SMC Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_wpmr](smc_wpmr) module"]
pub type SMC_WPMR = crate::Reg<u32, _SMC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_WPMR;
#[doc = "`read()` method returns [smc_wpmr::R](smc_wpmr::R) reader structure"]
impl crate::Readable for SMC_WPMR {}
#[doc = "`write(|w| ..)` method takes [smc_wpmr::W](smc_wpmr::W) writer structure"]
impl crate::Writable for SMC_WPMR {}
#[doc = "SMC Write Protection Mode Register"]
pub mod smc_wpmr;
#[doc = "SMC Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_wpsr](smc_wpsr) module"]
pub type SMC_WPSR = crate::Reg<u32, _SMC_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_WPSR;
#[doc = "`read()` method returns [smc_wpsr::R](smc_wpsr::R) reader structure"]
impl crate::Readable for SMC_WPSR {}
#[doc = "SMC Write Protection Status Register"]
pub mod smc_wpsr;
