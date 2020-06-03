#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub tc_channel0: TC_CHANNEL,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - Channel Control Register (channel = 0)"]
    pub tc_channel1: TC_CHANNEL,
    _reserved2: [u8; 12usize],
    #[doc = "0x80 - Channel Control Register (channel = 0)"]
    pub tc_channel2: TC_CHANNEL,
    _reserved3: [u8; 12usize],
    #[doc = "0xc0 - Block Control Register"]
    pub tc_bcr: TC_BCR,
    #[doc = "0xc4 - Block Mode Register"]
    pub tc_bmr: TC_BMR,
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    pub tc_qier: TC_QIER,
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    pub tc_qidr: TC_QIDR,
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    pub tc_qimr: TC_QIMR,
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    pub tc_qisr: TC_QISR,
    #[doc = "0xd8 - Fault Mode Register"]
    pub tc_fmr: TC_FMR,
    _reserved10: [u8; 8usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub tc_wpmr: TC_WPMR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TC_CHANNEL {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub tc_ccr: self::tc_channel::TC_CCR,
    _reserved_1_tc_cmr: [u8; 4usize],
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub tc_smmr: self::tc_channel::TC_SMMR,
    #[doc = "0x0c - Register AB (channel = 0)"]
    pub tc_rab: self::tc_channel::TC_RAB,
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub tc_cv: self::tc_channel::TC_CV,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub tc_ra: self::tc_channel::TC_RA,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub tc_rb: self::tc_channel::TC_RB,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub tc_rc: self::tc_channel::TC_RC,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub tc_sr: self::tc_channel::TC_SR,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub tc_ier: self::tc_channel::TC_IER,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub tc_idr: self::tc_channel::TC_IDR,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub tc_imr: self::tc_channel::TC_IMR,
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    pub tc_emr: self::tc_channel::TC_EMR,
}
impl TC_CHANNEL {
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn tc_cmr_waveform_mode(&self) -> &self::tc_channel::TC_CMR_WAVEFORM_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const self::tc_channel::TC_CMR_WAVEFORM_MODE)
        }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn tc_cmr_waveform_mode_mut(&self) -> &mut self::tc_channel::TC_CMR_WAVEFORM_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4usize)
                as *mut self::tc_channel::TC_CMR_WAVEFORM_MODE)
        }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn tc_cmr_capture_mode(&self) -> &self::tc_channel::TC_CMR_CAPTURE_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const self::tc_channel::TC_CMR_CAPTURE_MODE)
        }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn tc_cmr_capture_mode_mut(&self) -> &mut self::tc_channel::TC_CMR_CAPTURE_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4usize)
                as *mut self::tc_channel::TC_CMR_CAPTURE_MODE)
        }
    }
}
#[doc = r"Register block"]
#[doc = "Channel Control Register (channel = 0)"]
pub mod tc_channel;
#[doc = "Block Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_bcr](tc_bcr) module"]
pub type TC_BCR = crate::Reg<u32, _TC_BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_BCR;
#[doc = "`write(|w| ..)` method takes [tc_bcr::W](tc_bcr::W) writer structure"]
impl crate::Writable for TC_BCR {}
#[doc = "Block Control Register"]
pub mod tc_bcr;
#[doc = "Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_bmr](tc_bmr) module"]
pub type TC_BMR = crate::Reg<u32, _TC_BMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_BMR;
#[doc = "`read()` method returns [tc_bmr::R](tc_bmr::R) reader structure"]
impl crate::Readable for TC_BMR {}
#[doc = "`write(|w| ..)` method takes [tc_bmr::W](tc_bmr::W) writer structure"]
impl crate::Writable for TC_BMR {}
#[doc = "Block Mode Register"]
pub mod tc_bmr;
#[doc = "QDEC Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_qier](tc_qier) module"]
pub type TC_QIER = crate::Reg<u32, _TC_QIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_QIER;
#[doc = "`write(|w| ..)` method takes [tc_qier::W](tc_qier::W) writer structure"]
impl crate::Writable for TC_QIER {}
#[doc = "QDEC Interrupt Enable Register"]
pub mod tc_qier;
#[doc = "QDEC Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_qidr](tc_qidr) module"]
pub type TC_QIDR = crate::Reg<u32, _TC_QIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_QIDR;
#[doc = "`write(|w| ..)` method takes [tc_qidr::W](tc_qidr::W) writer structure"]
impl crate::Writable for TC_QIDR {}
#[doc = "QDEC Interrupt Disable Register"]
pub mod tc_qidr;
#[doc = "QDEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_qimr](tc_qimr) module"]
pub type TC_QIMR = crate::Reg<u32, _TC_QIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_QIMR;
#[doc = "`read()` method returns [tc_qimr::R](tc_qimr::R) reader structure"]
impl crate::Readable for TC_QIMR {}
#[doc = "QDEC Interrupt Mask Register"]
pub mod tc_qimr;
#[doc = "QDEC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_qisr](tc_qisr) module"]
pub type TC_QISR = crate::Reg<u32, _TC_QISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_QISR;
#[doc = "`read()` method returns [tc_qisr::R](tc_qisr::R) reader structure"]
impl crate::Readable for TC_QISR {}
#[doc = "QDEC Interrupt Status Register"]
pub mod tc_qisr;
#[doc = "Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_fmr](tc_fmr) module"]
pub type TC_FMR = crate::Reg<u32, _TC_FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_FMR;
#[doc = "`read()` method returns [tc_fmr::R](tc_fmr::R) reader structure"]
impl crate::Readable for TC_FMR {}
#[doc = "`write(|w| ..)` method takes [tc_fmr::W](tc_fmr::W) writer structure"]
impl crate::Writable for TC_FMR {}
#[doc = "Fault Mode Register"]
pub mod tc_fmr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_wpmr](tc_wpmr) module"]
pub type TC_WPMR = crate::Reg<u32, _TC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_WPMR;
#[doc = "`read()` method returns [tc_wpmr::R](tc_wpmr::R) reader structure"]
impl crate::Readable for TC_WPMR {}
#[doc = "`write(|w| ..)` method takes [tc_wpmr::W](tc_wpmr::W) writer structure"]
impl crate::Writable for TC_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod tc_wpmr;
