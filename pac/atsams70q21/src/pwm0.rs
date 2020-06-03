#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    pub pwm_clk: PWM_CLK,
    #[doc = "0x04 - PWM Enable Register"]
    pub pwm_ena: PWM_ENA,
    #[doc = "0x08 - PWM Disable Register"]
    pub pwm_dis: PWM_DIS,
    #[doc = "0x0c - PWM Status Register"]
    pub pwm_sr: PWM_SR,
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    pub pwm_ier1: PWM_IER1,
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    pub pwm_idr1: PWM_IDR1,
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    pub pwm_imr1: PWM_IMR1,
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    pub pwm_isr1: PWM_ISR1,
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    pub pwm_scm: PWM_SCM,
    #[doc = "0x24 - PWM DMA Register"]
    pub pwm_dmar: PWM_DMAR,
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    pub pwm_scuc: PWM_SCUC,
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    pub pwm_scup: PWM_SCUP,
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    pub pwm_scupupd: PWM_SCUPUPD,
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    pub pwm_ier2: PWM_IER2,
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    pub pwm_idr2: PWM_IDR2,
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    pub pwm_imr2: PWM_IMR2,
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    pub pwm_isr2: PWM_ISR2,
    #[doc = "0x44 - PWM Output Override Value Register"]
    pub pwm_oov: PWM_OOV,
    #[doc = "0x48 - PWM Output Selection Register"]
    pub pwm_os: PWM_OS,
    #[doc = "0x4c - PWM Output Selection Set Register"]
    pub pwm_oss: PWM_OSS,
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    pub pwm_osc: PWM_OSC,
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    pub pwm_ossupd: PWM_OSSUPD,
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    pub pwm_oscupd: PWM_OSCUPD,
    #[doc = "0x5c - PWM Fault Mode Register"]
    pub pwm_fmr: PWM_FMR,
    #[doc = "0x60 - PWM Fault Status Register"]
    pub pwm_fsr: PWM_FSR,
    #[doc = "0x64 - PWM Fault Clear Register"]
    pub pwm_fcr: PWM_FCR,
    #[doc = "0x68 - PWM Fault Protection Value Register 1"]
    pub pwm_fpv1: PWM_FPV1,
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    pub pwm_fpe: PWM_FPE,
    _reserved28: [u8; 12usize],
    #[doc = "0x7c - PWM Event Line 0 Mode Register 0"]
    pub pwm_elmr: [PWM_ELMR; 2],
    _reserved29: [u8; 28usize],
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    pub pwm_sspr: PWM_SSPR,
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    pub pwm_sspup: PWM_SSPUP,
    _reserved31: [u8; 8usize],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub pwm_smmr: PWM_SMMR,
    _reserved32: [u8; 12usize],
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    pub pwm_fpv2: PWM_FPV2,
    _reserved33: [u8; 32usize],
    #[doc = "0xe4 - PWM Write Protection Control Register"]
    pub pwm_wpcr: PWM_WPCR,
    #[doc = "0xe8 - PWM Write Protection Status Register"]
    pub pwm_wpsr: PWM_WPSR,
    _reserved35: [u8; 68usize],
    #[doc = "0x130 - PWM Comparison 0 Value Register"]
    pub pwm_cmp: [PWM_CMP; 8],
    _reserved36: [u8; 80usize],
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    pub pwm_ch_num: [PWM_CH_NUM; 4],
    _reserved37: [u8; 384usize],
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    pub pwm_cmupd0: PWM_CMUPD0,
    _reserved38: [u8; 28usize],
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    pub pwm_cmupd1: PWM_CMUPD1,
    _reserved39: [u8; 8usize],
    #[doc = "0x42c - PWM External Trigger Register (trg_num = 1)"]
    pub pwm_etrg1: PWM_ETRG1,
    #[doc = "0x430 - PWM Leading-Edge Blanking Register (trg_num = 1)"]
    pub pwm_lebr1: PWM_LEBR1,
    _reserved41: [u8; 12usize],
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    pub pwm_cmupd2: PWM_CMUPD2,
    _reserved42: [u8; 8usize],
    #[doc = "0x44c - PWM External Trigger Register (trg_num = 2)"]
    pub pwm_etrg2: PWM_ETRG2,
    #[doc = "0x450 - PWM Leading-Edge Blanking Register (trg_num = 2)"]
    pub pwm_lebr2: PWM_LEBR2,
    _reserved44: [u8; 12usize],
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    pub pwm_cmupd3: PWM_CMUPD3,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CMP {
    #[doc = "0x00 - PWM Comparison 0 Value Register"]
    pub pwm_cmpv: self::pwm_cmp::PWM_CMPV,
    #[doc = "0x04 - PWM Comparison 0 Value Update Register"]
    pub pwm_cmpvupd: self::pwm_cmp::PWM_CMPVUPD,
    #[doc = "0x08 - PWM Comparison 0 Mode Register"]
    pub pwm_cmpm: self::pwm_cmp::PWM_CMPM,
    #[doc = "0x0c - PWM Comparison 0 Mode Update Register"]
    pub pwm_cmpmupd: self::pwm_cmp::PWM_CMPMUPD,
}
#[doc = r"Register block"]
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmp;
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CH_NUM {
    #[doc = "0x00 - PWM Channel Mode Register (ch_num = 0)"]
    pub pwm_cmr: self::pwm_ch_num::PWM_CMR,
    #[doc = "0x04 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub pwm_cdty: self::pwm_ch_num::PWM_CDTY,
    #[doc = "0x08 - PWM Channel Duty Cycle Update Register (ch_num = 0)"]
    pub pwm_cdtyupd: self::pwm_ch_num::PWM_CDTYUPD,
    #[doc = "0x0c - PWM Channel Period Register (ch_num = 0)"]
    pub pwm_cprd: self::pwm_ch_num::PWM_CPRD,
    #[doc = "0x10 - PWM Channel Period Update Register (ch_num = 0)"]
    pub pwm_cprdupd: self::pwm_ch_num::PWM_CPRDUPD,
    #[doc = "0x14 - PWM Channel Counter Register (ch_num = 0)"]
    pub pwm_ccnt: self::pwm_ch_num::PWM_CCNT,
    #[doc = "0x18 - PWM Channel Dead Time Register (ch_num = 0)"]
    pub pwm_dt: self::pwm_ch_num::PWM_DT,
    #[doc = "0x1c - PWM Channel Dead Time Update Register (ch_num = 0)"]
    pub pwm_dtupd: self::pwm_ch_num::PWM_DTUPD,
}
#[doc = r"Register block"]
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod pwm_ch_num;
#[doc = "PWM Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_clk](pwm_clk) module"]
pub type PWM_CLK = crate::Reg<u32, _PWM_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CLK;
#[doc = "`read()` method returns [pwm_clk::R](pwm_clk::R) reader structure"]
impl crate::Readable for PWM_CLK {}
#[doc = "`write(|w| ..)` method takes [pwm_clk::W](pwm_clk::W) writer structure"]
impl crate::Writable for PWM_CLK {}
#[doc = "PWM Clock Register"]
pub mod pwm_clk;
#[doc = "PWM Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ena](pwm_ena) module"]
pub type PWM_ENA = crate::Reg<u32, _PWM_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_ENA;
#[doc = "`write(|w| ..)` method takes [pwm_ena::W](pwm_ena::W) writer structure"]
impl crate::Writable for PWM_ENA {}
#[doc = "PWM Enable Register"]
pub mod pwm_ena;
#[doc = "PWM Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dis](pwm_dis) module"]
pub type PWM_DIS = crate::Reg<u32, _PWM_DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_DIS;
#[doc = "`write(|w| ..)` method takes [pwm_dis::W](pwm_dis::W) writer structure"]
impl crate::Writable for PWM_DIS {}
#[doc = "PWM Disable Register"]
pub mod pwm_dis;
#[doc = "PWM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_sr](pwm_sr) module"]
pub type PWM_SR = crate::Reg<u32, _PWM_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SR;
#[doc = "`read()` method returns [pwm_sr::R](pwm_sr::R) reader structure"]
impl crate::Readable for PWM_SR {}
#[doc = "PWM Status Register"]
pub mod pwm_sr;
#[doc = "PWM Interrupt Enable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ier1](pwm_ier1) module"]
pub type PWM_IER1 = crate::Reg<u32, _PWM_IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_IER1;
#[doc = "`write(|w| ..)` method takes [pwm_ier1::W](pwm_ier1::W) writer structure"]
impl crate::Writable for PWM_IER1 {}
#[doc = "PWM Interrupt Enable Register 1"]
pub mod pwm_ier1;
#[doc = "PWM Interrupt Disable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_idr1](pwm_idr1) module"]
pub type PWM_IDR1 = crate::Reg<u32, _PWM_IDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_IDR1;
#[doc = "`write(|w| ..)` method takes [pwm_idr1::W](pwm_idr1::W) writer structure"]
impl crate::Writable for PWM_IDR1 {}
#[doc = "PWM Interrupt Disable Register 1"]
pub mod pwm_idr1;
#[doc = "PWM Interrupt Mask Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_imr1](pwm_imr1) module"]
pub type PWM_IMR1 = crate::Reg<u32, _PWM_IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_IMR1;
#[doc = "`read()` method returns [pwm_imr1::R](pwm_imr1::R) reader structure"]
impl crate::Readable for PWM_IMR1 {}
#[doc = "PWM Interrupt Mask Register 1"]
pub mod pwm_imr1;
#[doc = "PWM Interrupt Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_isr1](pwm_isr1) module"]
pub type PWM_ISR1 = crate::Reg<u32, _PWM_ISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_ISR1;
#[doc = "`read()` method returns [pwm_isr1::R](pwm_isr1::R) reader structure"]
impl crate::Readable for PWM_ISR1 {}
#[doc = "PWM Interrupt Status Register 1"]
pub mod pwm_isr1;
#[doc = "PWM Sync Channels Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_scm](pwm_scm) module"]
pub type PWM_SCM = crate::Reg<u32, _PWM_SCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SCM;
#[doc = "`read()` method returns [pwm_scm::R](pwm_scm::R) reader structure"]
impl crate::Readable for PWM_SCM {}
#[doc = "`write(|w| ..)` method takes [pwm_scm::W](pwm_scm::W) writer structure"]
impl crate::Writable for PWM_SCM {}
#[doc = "PWM Sync Channels Mode Register"]
pub mod pwm_scm;
#[doc = "PWM DMA Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dmar](pwm_dmar) module"]
pub type PWM_DMAR = crate::Reg<u32, _PWM_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_DMAR;
#[doc = "`write(|w| ..)` method takes [pwm_dmar::W](pwm_dmar::W) writer structure"]
impl crate::Writable for PWM_DMAR {}
#[doc = "PWM DMA Register"]
pub mod pwm_dmar;
#[doc = "PWM Sync Channels Update Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_scuc](pwm_scuc) module"]
pub type PWM_SCUC = crate::Reg<u32, _PWM_SCUC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SCUC;
#[doc = "`read()` method returns [pwm_scuc::R](pwm_scuc::R) reader structure"]
impl crate::Readable for PWM_SCUC {}
#[doc = "`write(|w| ..)` method takes [pwm_scuc::W](pwm_scuc::W) writer structure"]
impl crate::Writable for PWM_SCUC {}
#[doc = "PWM Sync Channels Update Control Register"]
pub mod pwm_scuc;
#[doc = "PWM Sync Channels Update Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_scup](pwm_scup) module"]
pub type PWM_SCUP = crate::Reg<u32, _PWM_SCUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SCUP;
#[doc = "`read()` method returns [pwm_scup::R](pwm_scup::R) reader structure"]
impl crate::Readable for PWM_SCUP {}
#[doc = "`write(|w| ..)` method takes [pwm_scup::W](pwm_scup::W) writer structure"]
impl crate::Writable for PWM_SCUP {}
#[doc = "PWM Sync Channels Update Period Register"]
pub mod pwm_scup;
#[doc = "PWM Sync Channels Update Period Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_scupupd](pwm_scupupd) module"]
pub type PWM_SCUPUPD = crate::Reg<u32, _PWM_SCUPUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SCUPUPD;
#[doc = "`write(|w| ..)` method takes [pwm_scupupd::W](pwm_scupupd::W) writer structure"]
impl crate::Writable for PWM_SCUPUPD {}
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod pwm_scupupd;
#[doc = "PWM Interrupt Enable Register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ier2](pwm_ier2) module"]
pub type PWM_IER2 = crate::Reg<u32, _PWM_IER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_IER2;
#[doc = "`write(|w| ..)` method takes [pwm_ier2::W](pwm_ier2::W) writer structure"]
impl crate::Writable for PWM_IER2 {}
#[doc = "PWM Interrupt Enable Register 2"]
pub mod pwm_ier2;
#[doc = "PWM Interrupt Disable Register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_idr2](pwm_idr2) module"]
pub type PWM_IDR2 = crate::Reg<u32, _PWM_IDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_IDR2;
#[doc = "`write(|w| ..)` method takes [pwm_idr2::W](pwm_idr2::W) writer structure"]
impl crate::Writable for PWM_IDR2 {}
#[doc = "PWM Interrupt Disable Register 2"]
pub mod pwm_idr2;
#[doc = "PWM Interrupt Mask Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_imr2](pwm_imr2) module"]
pub type PWM_IMR2 = crate::Reg<u32, _PWM_IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_IMR2;
#[doc = "`read()` method returns [pwm_imr2::R](pwm_imr2::R) reader structure"]
impl crate::Readable for PWM_IMR2 {}
#[doc = "PWM Interrupt Mask Register 2"]
pub mod pwm_imr2;
#[doc = "PWM Interrupt Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_isr2](pwm_isr2) module"]
pub type PWM_ISR2 = crate::Reg<u32, _PWM_ISR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_ISR2;
#[doc = "`read()` method returns [pwm_isr2::R](pwm_isr2::R) reader structure"]
impl crate::Readable for PWM_ISR2 {}
#[doc = "PWM Interrupt Status Register 2"]
pub mod pwm_isr2;
#[doc = "PWM Output Override Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_oov](pwm_oov) module"]
pub type PWM_OOV = crate::Reg<u32, _PWM_OOV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_OOV;
#[doc = "`read()` method returns [pwm_oov::R](pwm_oov::R) reader structure"]
impl crate::Readable for PWM_OOV {}
#[doc = "`write(|w| ..)` method takes [pwm_oov::W](pwm_oov::W) writer structure"]
impl crate::Writable for PWM_OOV {}
#[doc = "PWM Output Override Value Register"]
pub mod pwm_oov;
#[doc = "PWM Output Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_os](pwm_os) module"]
pub type PWM_OS = crate::Reg<u32, _PWM_OS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_OS;
#[doc = "`read()` method returns [pwm_os::R](pwm_os::R) reader structure"]
impl crate::Readable for PWM_OS {}
#[doc = "`write(|w| ..)` method takes [pwm_os::W](pwm_os::W) writer structure"]
impl crate::Writable for PWM_OS {}
#[doc = "PWM Output Selection Register"]
pub mod pwm_os;
#[doc = "PWM Output Selection Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_oss](pwm_oss) module"]
pub type PWM_OSS = crate::Reg<u32, _PWM_OSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_OSS;
#[doc = "`write(|w| ..)` method takes [pwm_oss::W](pwm_oss::W) writer structure"]
impl crate::Writable for PWM_OSS {}
#[doc = "PWM Output Selection Set Register"]
pub mod pwm_oss;
#[doc = "PWM Output Selection Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_osc](pwm_osc) module"]
pub type PWM_OSC = crate::Reg<u32, _PWM_OSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_OSC;
#[doc = "`write(|w| ..)` method takes [pwm_osc::W](pwm_osc::W) writer structure"]
impl crate::Writable for PWM_OSC {}
#[doc = "PWM Output Selection Clear Register"]
pub mod pwm_osc;
#[doc = "PWM Output Selection Set Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ossupd](pwm_ossupd) module"]
pub type PWM_OSSUPD = crate::Reg<u32, _PWM_OSSUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_OSSUPD;
#[doc = "`write(|w| ..)` method takes [pwm_ossupd::W](pwm_ossupd::W) writer structure"]
impl crate::Writable for PWM_OSSUPD {}
#[doc = "PWM Output Selection Set Update Register"]
pub mod pwm_ossupd;
#[doc = "PWM Output Selection Clear Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_oscupd](pwm_oscupd) module"]
pub type PWM_OSCUPD = crate::Reg<u32, _PWM_OSCUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_OSCUPD;
#[doc = "`write(|w| ..)` method takes [pwm_oscupd::W](pwm_oscupd::W) writer structure"]
impl crate::Writable for PWM_OSCUPD {}
#[doc = "PWM Output Selection Clear Update Register"]
pub mod pwm_oscupd;
#[doc = "PWM Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fmr](pwm_fmr) module"]
pub type PWM_FMR = crate::Reg<u32, _PWM_FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_FMR;
#[doc = "`read()` method returns [pwm_fmr::R](pwm_fmr::R) reader structure"]
impl crate::Readable for PWM_FMR {}
#[doc = "`write(|w| ..)` method takes [pwm_fmr::W](pwm_fmr::W) writer structure"]
impl crate::Writable for PWM_FMR {}
#[doc = "PWM Fault Mode Register"]
pub mod pwm_fmr;
#[doc = "PWM Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fsr](pwm_fsr) module"]
pub type PWM_FSR = crate::Reg<u32, _PWM_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_FSR;
#[doc = "`read()` method returns [pwm_fsr::R](pwm_fsr::R) reader structure"]
impl crate::Readable for PWM_FSR {}
#[doc = "PWM Fault Status Register"]
pub mod pwm_fsr;
#[doc = "PWM Fault Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fcr](pwm_fcr) module"]
pub type PWM_FCR = crate::Reg<u32, _PWM_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_FCR;
#[doc = "`write(|w| ..)` method takes [pwm_fcr::W](pwm_fcr::W) writer structure"]
impl crate::Writable for PWM_FCR {}
#[doc = "PWM Fault Clear Register"]
pub mod pwm_fcr;
#[doc = "PWM Fault Protection Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fpv1](pwm_fpv1) module"]
pub type PWM_FPV1 = crate::Reg<u32, _PWM_FPV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_FPV1;
#[doc = "`read()` method returns [pwm_fpv1::R](pwm_fpv1::R) reader structure"]
impl crate::Readable for PWM_FPV1 {}
#[doc = "`write(|w| ..)` method takes [pwm_fpv1::W](pwm_fpv1::W) writer structure"]
impl crate::Writable for PWM_FPV1 {}
#[doc = "PWM Fault Protection Value Register 1"]
pub mod pwm_fpv1;
#[doc = "PWM Fault Protection Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fpe](pwm_fpe) module"]
pub type PWM_FPE = crate::Reg<u32, _PWM_FPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_FPE;
#[doc = "`read()` method returns [pwm_fpe::R](pwm_fpe::R) reader structure"]
impl crate::Readable for PWM_FPE {}
#[doc = "`write(|w| ..)` method takes [pwm_fpe::W](pwm_fpe::W) writer structure"]
impl crate::Writable for PWM_FPE {}
#[doc = "PWM Fault Protection Enable Register"]
pub mod pwm_fpe;
#[doc = "PWM Event Line 0 Mode Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_elmr](pwm_elmr) module"]
pub type PWM_ELMR = crate::Reg<u32, _PWM_ELMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_ELMR;
#[doc = "`read()` method returns [pwm_elmr::R](pwm_elmr::R) reader structure"]
impl crate::Readable for PWM_ELMR {}
#[doc = "`write(|w| ..)` method takes [pwm_elmr::W](pwm_elmr::W) writer structure"]
impl crate::Writable for PWM_ELMR {}
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod pwm_elmr;
#[doc = "PWM Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_sspr](pwm_sspr) module"]
pub type PWM_SSPR = crate::Reg<u32, _PWM_SSPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SSPR;
#[doc = "`read()` method returns [pwm_sspr::R](pwm_sspr::R) reader structure"]
impl crate::Readable for PWM_SSPR {}
#[doc = "`write(|w| ..)` method takes [pwm_sspr::W](pwm_sspr::W) writer structure"]
impl crate::Writable for PWM_SSPR {}
#[doc = "PWM Spread Spectrum Register"]
pub mod pwm_sspr;
#[doc = "PWM Spread Spectrum Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_sspup](pwm_sspup) module"]
pub type PWM_SSPUP = crate::Reg<u32, _PWM_SSPUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SSPUP;
#[doc = "`write(|w| ..)` method takes [pwm_sspup::W](pwm_sspup::W) writer structure"]
impl crate::Writable for PWM_SSPUP {}
#[doc = "PWM Spread Spectrum Update Register"]
pub mod pwm_sspup;
#[doc = "PWM Stepper Motor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_smmr](pwm_smmr) module"]
pub type PWM_SMMR = crate::Reg<u32, _PWM_SMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_SMMR;
#[doc = "`read()` method returns [pwm_smmr::R](pwm_smmr::R) reader structure"]
impl crate::Readable for PWM_SMMR {}
#[doc = "`write(|w| ..)` method takes [pwm_smmr::W](pwm_smmr::W) writer structure"]
impl crate::Writable for PWM_SMMR {}
#[doc = "PWM Stepper Motor Mode Register"]
pub mod pwm_smmr;
#[doc = "PWM Fault Protection Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fpv2](pwm_fpv2) module"]
pub type PWM_FPV2 = crate::Reg<u32, _PWM_FPV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_FPV2;
#[doc = "`read()` method returns [pwm_fpv2::R](pwm_fpv2::R) reader structure"]
impl crate::Readable for PWM_FPV2 {}
#[doc = "`write(|w| ..)` method takes [pwm_fpv2::W](pwm_fpv2::W) writer structure"]
impl crate::Writable for PWM_FPV2 {}
#[doc = "PWM Fault Protection Value 2 Register"]
pub mod pwm_fpv2;
#[doc = "PWM Write Protection Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_wpcr](pwm_wpcr) module"]
pub type PWM_WPCR = crate::Reg<u32, _PWM_WPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_WPCR;
#[doc = "`write(|w| ..)` method takes [pwm_wpcr::W](pwm_wpcr::W) writer structure"]
impl crate::Writable for PWM_WPCR {}
#[doc = "PWM Write Protection Control Register"]
pub mod pwm_wpcr;
#[doc = "PWM Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_wpsr](pwm_wpsr) module"]
pub type PWM_WPSR = crate::Reg<u32, _PWM_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_WPSR;
#[doc = "`read()` method returns [pwm_wpsr::R](pwm_wpsr::R) reader structure"]
impl crate::Readable for PWM_WPSR {}
#[doc = "PWM Write Protection Status Register"]
pub mod pwm_wpsr;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmupd0](pwm_cmupd0) module"]
pub type PWM_CMUPD0 = crate::Reg<u32, _PWM_CMUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMUPD0;
#[doc = "`write(|w| ..)` method takes [pwm_cmupd0::W](pwm_cmupd0::W) writer structure"]
impl crate::Writable for PWM_CMUPD0 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod pwm_cmupd0;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmupd1](pwm_cmupd1) module"]
pub type PWM_CMUPD1 = crate::Reg<u32, _PWM_CMUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMUPD1;
#[doc = "`write(|w| ..)` method takes [pwm_cmupd1::W](pwm_cmupd1::W) writer structure"]
impl crate::Writable for PWM_CMUPD1 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod pwm_cmupd1;
#[doc = "PWM External Trigger Register (trg_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_etrg1](pwm_etrg1) module"]
pub type PWM_ETRG1 = crate::Reg<u32, _PWM_ETRG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_ETRG1;
#[doc = "`read()` method returns [pwm_etrg1::R](pwm_etrg1::R) reader structure"]
impl crate::Readable for PWM_ETRG1 {}
#[doc = "`write(|w| ..)` method takes [pwm_etrg1::W](pwm_etrg1::W) writer structure"]
impl crate::Writable for PWM_ETRG1 {}
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub mod pwm_etrg1;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_lebr1](pwm_lebr1) module"]
pub type PWM_LEBR1 = crate::Reg<u32, _PWM_LEBR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_LEBR1;
#[doc = "`read()` method returns [pwm_lebr1::R](pwm_lebr1::R) reader structure"]
impl crate::Readable for PWM_LEBR1 {}
#[doc = "`write(|w| ..)` method takes [pwm_lebr1::W](pwm_lebr1::W) writer structure"]
impl crate::Writable for PWM_LEBR1 {}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub mod pwm_lebr1;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmupd2](pwm_cmupd2) module"]
pub type PWM_CMUPD2 = crate::Reg<u32, _PWM_CMUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMUPD2;
#[doc = "`write(|w| ..)` method takes [pwm_cmupd2::W](pwm_cmupd2::W) writer structure"]
impl crate::Writable for PWM_CMUPD2 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod pwm_cmupd2;
#[doc = "PWM External Trigger Register (trg_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_etrg2](pwm_etrg2) module"]
pub type PWM_ETRG2 = crate::Reg<u32, _PWM_ETRG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_ETRG2;
#[doc = "`read()` method returns [pwm_etrg2::R](pwm_etrg2::R) reader structure"]
impl crate::Readable for PWM_ETRG2 {}
#[doc = "`write(|w| ..)` method takes [pwm_etrg2::W](pwm_etrg2::W) writer structure"]
impl crate::Writable for PWM_ETRG2 {}
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub mod pwm_etrg2;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_lebr2](pwm_lebr2) module"]
pub type PWM_LEBR2 = crate::Reg<u32, _PWM_LEBR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_LEBR2;
#[doc = "`read()` method returns [pwm_lebr2::R](pwm_lebr2::R) reader structure"]
impl crate::Readable for PWM_LEBR2 {}
#[doc = "`write(|w| ..)` method takes [pwm_lebr2::W](pwm_lebr2::W) writer structure"]
impl crate::Writable for PWM_LEBR2 {}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub mod pwm_lebr2;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmupd3](pwm_cmupd3) module"]
pub type PWM_CMUPD3 = crate::Reg<u32, _PWM_CMUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMUPD3;
#[doc = "`write(|w| ..)` method takes [pwm_cmupd3::W](pwm_cmupd3::W) writer structure"]
impl crate::Writable for PWM_CMUPD3 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod pwm_cmupd3;
