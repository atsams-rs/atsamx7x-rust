#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    pub pmc_scer: PMC_SCER,
    #[doc = "0x04 - System Clock Disable Register"]
    pub pmc_scdr: PMC_SCDR,
    #[doc = "0x08 - System Clock Status Register"]
    pub pmc_scsr: PMC_SCSR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    pub pmc_pcer0: PMC_PCER0,
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    pub pmc_pcdr0: PMC_PCDR0,
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    pub pmc_pcsr0: PMC_PCSR0,
    #[doc = "0x1c - UTMI Clock Register"]
    pub ckgr_uckr: CKGR_UCKR,
    #[doc = "0x20 - Main Oscillator Register"]
    pub ckgr_mor: CKGR_MOR,
    #[doc = "0x24 - Main Clock Frequency Register"]
    pub ckgr_mcfr: CKGR_MCFR,
    #[doc = "0x28 - PLLA Register"]
    pub ckgr_pllar: CKGR_PLLAR,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Master Clock Register"]
    pub pmc_mckr: PMC_MCKR,
    _reserved11: [u8; 4usize],
    #[doc = "0x38 - USB Clock Register"]
    pub pmc_usb: PMC_USB,
    _reserved12: [u8; 4usize],
    #[doc = "0x40 - Programmable Clock Register"]
    pub pmc_pck: [PMC_PCK; 8],
    #[doc = "0x60 - Interrupt Enable Register"]
    pub pmc_ier: PMC_IER,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub pmc_idr: PMC_IDR,
    #[doc = "0x68 - Status Register"]
    pub pmc_sr: PMC_SR,
    #[doc = "0x6c - Interrupt Mask Register"]
    pub pmc_imr: PMC_IMR,
    #[doc = "0x70 - Fast Startup Mode Register"]
    pub pmc_fsmr: PMC_FSMR,
    #[doc = "0x74 - Fast Startup Polarity Register"]
    pub pmc_fspr: PMC_FSPR,
    #[doc = "0x78 - Fault Output Clear Register"]
    pub pmc_focr: PMC_FOCR,
    _reserved20: [u8; 104usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub pmc_wpmr: PMC_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub pmc_wpsr: PMC_WPSR,
    _reserved22: [u8; 20usize],
    #[doc = "0x100 - Peripheral Clock Enable Register 1"]
    pub pmc_pcer1: PMC_PCER1,
    #[doc = "0x104 - Peripheral Clock Disable Register 1"]
    pub pmc_pcdr1: PMC_PCDR1,
    #[doc = "0x108 - Peripheral Clock Status Register 1"]
    pub pmc_pcsr1: PMC_PCSR1,
    #[doc = "0x10c - Peripheral Control Register"]
    pub pmc_pcr: PMC_PCR,
    #[doc = "0x110 - Oscillator Calibration Register"]
    pub pmc_ocr: PMC_OCR,
    #[doc = "0x114 - SleepWalking Enable Register 0"]
    pub pmc_slpwk_er0: PMC_SLPWK_ER0,
    #[doc = "0x118 - SleepWalking Disable Register 0"]
    pub pmc_slpwk_dr0: PMC_SLPWK_DR0,
    #[doc = "0x11c - SleepWalking Status Register 0"]
    pub pmc_slpwk_sr0: PMC_SLPWK_SR0,
    #[doc = "0x120 - SleepWalking Activity Status Register 0"]
    pub pmc_slpwk_asr0: PMC_SLPWK_ASR0,
    _reserved31: [u8; 12usize],
    #[doc = "0x130 - PLL Maximum Multiplier Value Register"]
    pub pmc_pmmr: PMC_PMMR,
    #[doc = "0x134 - SleepWalking Enable Register 1"]
    pub pmc_slpwk_er1: PMC_SLPWK_ER1,
    #[doc = "0x138 - SleepWalking Disable Register 1"]
    pub pmc_slpwk_dr1: PMC_SLPWK_DR1,
    #[doc = "0x13c - SleepWalking Status Register 1"]
    pub pmc_slpwk_sr1: PMC_SLPWK_SR1,
    #[doc = "0x140 - SleepWalking Activity Status Register 1"]
    pub pmc_slpwk_asr1: PMC_SLPWK_ASR1,
    #[doc = "0x144 - SleepWalking Activity In Progress Register"]
    pub pmc_slpwk_aipr: PMC_SLPWK_AIPR,
}
#[doc = "System Clock Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scer](pmc_scer) module"]
pub type PMC_SCER = crate::Reg<u32, _PMC_SCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SCER;
#[doc = "`write(|w| ..)` method takes [pmc_scer::W](pmc_scer::W) writer structure"]
impl crate::Writable for PMC_SCER {}
#[doc = "System Clock Enable Register"]
pub mod pmc_scer;
#[doc = "System Clock Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scdr](pmc_scdr) module"]
pub type PMC_SCDR = crate::Reg<u32, _PMC_SCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SCDR;
#[doc = "`write(|w| ..)` method takes [pmc_scdr::W](pmc_scdr::W) writer structure"]
impl crate::Writable for PMC_SCDR {}
#[doc = "System Clock Disable Register"]
pub mod pmc_scdr;
#[doc = "System Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scsr](pmc_scsr) module"]
pub type PMC_SCSR = crate::Reg<u32, _PMC_SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SCSR;
#[doc = "`read()` method returns [pmc_scsr::R](pmc_scsr::R) reader structure"]
impl crate::Readable for PMC_SCSR {}
#[doc = "System Clock Status Register"]
pub mod pmc_scsr;
#[doc = "Peripheral Clock Enable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcer0](pmc_pcer0) module"]
pub type PMC_PCER0 = crate::Reg<u32, _PMC_PCER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCER0;
#[doc = "`write(|w| ..)` method takes [pmc_pcer0::W](pmc_pcer0::W) writer structure"]
impl crate::Writable for PMC_PCER0 {}
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pmc_pcer0;
#[doc = "Peripheral Clock Disable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcdr0](pmc_pcdr0) module"]
pub type PMC_PCDR0 = crate::Reg<u32, _PMC_PCDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCDR0;
#[doc = "`write(|w| ..)` method takes [pmc_pcdr0::W](pmc_pcdr0::W) writer structure"]
impl crate::Writable for PMC_PCDR0 {}
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pmc_pcdr0;
#[doc = "Peripheral Clock Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcsr0](pmc_pcsr0) module"]
pub type PMC_PCSR0 = crate::Reg<u32, _PMC_PCSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCSR0;
#[doc = "`read()` method returns [pmc_pcsr0::R](pmc_pcsr0::R) reader structure"]
impl crate::Readable for PMC_PCSR0 {}
#[doc = "Peripheral Clock Status Register 0"]
pub mod pmc_pcsr0;
#[doc = "UTMI Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_uckr](ckgr_uckr) module"]
pub type CKGR_UCKR = crate::Reg<u32, _CKGR_UCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_UCKR;
#[doc = "`read()` method returns [ckgr_uckr::R](ckgr_uckr::R) reader structure"]
impl crate::Readable for CKGR_UCKR {}
#[doc = "`write(|w| ..)` method takes [ckgr_uckr::W](ckgr_uckr::W) writer structure"]
impl crate::Writable for CKGR_UCKR {}
#[doc = "UTMI Clock Register"]
pub mod ckgr_uckr;
#[doc = "Main Oscillator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mor](ckgr_mor) module"]
pub type CKGR_MOR = crate::Reg<u32, _CKGR_MOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_MOR;
#[doc = "`read()` method returns [ckgr_mor::R](ckgr_mor::R) reader structure"]
impl crate::Readable for CKGR_MOR {}
#[doc = "`write(|w| ..)` method takes [ckgr_mor::W](ckgr_mor::W) writer structure"]
impl crate::Writable for CKGR_MOR {}
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "Main Clock Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mcfr](ckgr_mcfr) module"]
pub type CKGR_MCFR = crate::Reg<u32, _CKGR_MCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_MCFR;
#[doc = "`read()` method returns [ckgr_mcfr::R](ckgr_mcfr::R) reader structure"]
impl crate::Readable for CKGR_MCFR {}
#[doc = "`write(|w| ..)` method takes [ckgr_mcfr::W](ckgr_mcfr::W) writer structure"]
impl crate::Writable for CKGR_MCFR {}
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "PLLA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_pllar](ckgr_pllar) module"]
pub type CKGR_PLLAR = crate::Reg<u32, _CKGR_PLLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_PLLAR;
#[doc = "`read()` method returns [ckgr_pllar::R](ckgr_pllar::R) reader structure"]
impl crate::Readable for CKGR_PLLAR {}
#[doc = "`write(|w| ..)` method takes [ckgr_pllar::W](ckgr_pllar::W) writer structure"]
impl crate::Writable for CKGR_PLLAR {}
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "Master Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_mckr](pmc_mckr) module"]
pub type PMC_MCKR = crate::Reg<u32, _PMC_MCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_MCKR;
#[doc = "`read()` method returns [pmc_mckr::R](pmc_mckr::R) reader structure"]
impl crate::Readable for PMC_MCKR {}
#[doc = "`write(|w| ..)` method takes [pmc_mckr::W](pmc_mckr::W) writer structure"]
impl crate::Writable for PMC_MCKR {}
#[doc = "Master Clock Register"]
pub mod pmc_mckr;
#[doc = "USB Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_usb](pmc_usb) module"]
pub type PMC_USB = crate::Reg<u32, _PMC_USB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_USB;
#[doc = "`read()` method returns [pmc_usb::R](pmc_usb::R) reader structure"]
impl crate::Readable for PMC_USB {}
#[doc = "`write(|w| ..)` method takes [pmc_usb::W](pmc_usb::W) writer structure"]
impl crate::Writable for PMC_USB {}
#[doc = "USB Clock Register"]
pub mod pmc_usb;
#[doc = "Programmable Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pck](pmc_pck) module"]
pub type PMC_PCK = crate::Reg<u32, _PMC_PCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCK;
#[doc = "`read()` method returns [pmc_pck::R](pmc_pck::R) reader structure"]
impl crate::Readable for PMC_PCK {}
#[doc = "`write(|w| ..)` method takes [pmc_pck::W](pmc_pck::W) writer structure"]
impl crate::Writable for PMC_PCK {}
#[doc = "Programmable Clock Register"]
pub mod pmc_pck;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_ier](pmc_ier) module"]
pub type PMC_IER = crate::Reg<u32, _PMC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_IER;
#[doc = "`write(|w| ..)` method takes [pmc_ier::W](pmc_ier::W) writer structure"]
impl crate::Writable for PMC_IER {}
#[doc = "Interrupt Enable Register"]
pub mod pmc_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_idr](pmc_idr) module"]
pub type PMC_IDR = crate::Reg<u32, _PMC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_IDR;
#[doc = "`write(|w| ..)` method takes [pmc_idr::W](pmc_idr::W) writer structure"]
impl crate::Writable for PMC_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod pmc_idr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_sr](pmc_sr) module"]
pub type PMC_SR = crate::Reg<u32, _PMC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SR;
#[doc = "`read()` method returns [pmc_sr::R](pmc_sr::R) reader structure"]
impl crate::Readable for PMC_SR {}
#[doc = "Status Register"]
pub mod pmc_sr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_imr](pmc_imr) module"]
pub type PMC_IMR = crate::Reg<u32, _PMC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_IMR;
#[doc = "`read()` method returns [pmc_imr::R](pmc_imr::R) reader structure"]
impl crate::Readable for PMC_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod pmc_imr;
#[doc = "Fast Startup Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_fsmr](pmc_fsmr) module"]
pub type PMC_FSMR = crate::Reg<u32, _PMC_FSMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_FSMR;
#[doc = "`read()` method returns [pmc_fsmr::R](pmc_fsmr::R) reader structure"]
impl crate::Readable for PMC_FSMR {}
#[doc = "`write(|w| ..)` method takes [pmc_fsmr::W](pmc_fsmr::W) writer structure"]
impl crate::Writable for PMC_FSMR {}
#[doc = "Fast Startup Mode Register"]
pub mod pmc_fsmr;
#[doc = "Fast Startup Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_fspr](pmc_fspr) module"]
pub type PMC_FSPR = crate::Reg<u32, _PMC_FSPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_FSPR;
#[doc = "`read()` method returns [pmc_fspr::R](pmc_fspr::R) reader structure"]
impl crate::Readable for PMC_FSPR {}
#[doc = "`write(|w| ..)` method takes [pmc_fspr::W](pmc_fspr::W) writer structure"]
impl crate::Writable for PMC_FSPR {}
#[doc = "Fast Startup Polarity Register"]
pub mod pmc_fspr;
#[doc = "Fault Output Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_focr](pmc_focr) module"]
pub type PMC_FOCR = crate::Reg<u32, _PMC_FOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_FOCR;
#[doc = "`write(|w| ..)` method takes [pmc_focr::W](pmc_focr::W) writer structure"]
impl crate::Writable for PMC_FOCR {}
#[doc = "Fault Output Clear Register"]
pub mod pmc_focr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_wpmr](pmc_wpmr) module"]
pub type PMC_WPMR = crate::Reg<u32, _PMC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_WPMR;
#[doc = "`read()` method returns [pmc_wpmr::R](pmc_wpmr::R) reader structure"]
impl crate::Readable for PMC_WPMR {}
#[doc = "`write(|w| ..)` method takes [pmc_wpmr::W](pmc_wpmr::W) writer structure"]
impl crate::Writable for PMC_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod pmc_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_wpsr](pmc_wpsr) module"]
pub type PMC_WPSR = crate::Reg<u32, _PMC_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_WPSR;
#[doc = "`read()` method returns [pmc_wpsr::R](pmc_wpsr::R) reader structure"]
impl crate::Readable for PMC_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod pmc_wpsr;
#[doc = "Peripheral Clock Enable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcer1](pmc_pcer1) module"]
pub type PMC_PCER1 = crate::Reg<u32, _PMC_PCER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCER1;
#[doc = "`write(|w| ..)` method takes [pmc_pcer1::W](pmc_pcer1::W) writer structure"]
impl crate::Writable for PMC_PCER1 {}
#[doc = "Peripheral Clock Enable Register 1"]
pub mod pmc_pcer1;
#[doc = "Peripheral Clock Disable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcdr1](pmc_pcdr1) module"]
pub type PMC_PCDR1 = crate::Reg<u32, _PMC_PCDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCDR1;
#[doc = "`write(|w| ..)` method takes [pmc_pcdr1::W](pmc_pcdr1::W) writer structure"]
impl crate::Writable for PMC_PCDR1 {}
#[doc = "Peripheral Clock Disable Register 1"]
pub mod pmc_pcdr1;
#[doc = "Peripheral Clock Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcsr1](pmc_pcsr1) module"]
pub type PMC_PCSR1 = crate::Reg<u32, _PMC_PCSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCSR1;
#[doc = "`read()` method returns [pmc_pcsr1::R](pmc_pcsr1::R) reader structure"]
impl crate::Readable for PMC_PCSR1 {}
#[doc = "Peripheral Clock Status Register 1"]
pub mod pmc_pcsr1;
#[doc = "Peripheral Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcr](pmc_pcr) module"]
pub type PMC_PCR = crate::Reg<u32, _PMC_PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PCR;
#[doc = "`read()` method returns [pmc_pcr::R](pmc_pcr::R) reader structure"]
impl crate::Readable for PMC_PCR {}
#[doc = "`write(|w| ..)` method takes [pmc_pcr::W](pmc_pcr::W) writer structure"]
impl crate::Writable for PMC_PCR {}
#[doc = "Peripheral Control Register"]
pub mod pmc_pcr;
#[doc = "Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_ocr](pmc_ocr) module"]
pub type PMC_OCR = crate::Reg<u32, _PMC_OCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_OCR;
#[doc = "`read()` method returns [pmc_ocr::R](pmc_ocr::R) reader structure"]
impl crate::Readable for PMC_OCR {}
#[doc = "`write(|w| ..)` method takes [pmc_ocr::W](pmc_ocr::W) writer structure"]
impl crate::Writable for PMC_OCR {}
#[doc = "Oscillator Calibration Register"]
pub mod pmc_ocr;
#[doc = "SleepWalking Enable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_er0](pmc_slpwk_er0) module"]
pub type PMC_SLPWK_ER0 = crate::Reg<u32, _PMC_SLPWK_ER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_ER0;
#[doc = "`write(|w| ..)` method takes [pmc_slpwk_er0::W](pmc_slpwk_er0::W) writer structure"]
impl crate::Writable for PMC_SLPWK_ER0 {}
#[doc = "SleepWalking Enable Register 0"]
pub mod pmc_slpwk_er0;
#[doc = "SleepWalking Disable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_dr0](pmc_slpwk_dr0) module"]
pub type PMC_SLPWK_DR0 = crate::Reg<u32, _PMC_SLPWK_DR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_DR0;
#[doc = "`write(|w| ..)` method takes [pmc_slpwk_dr0::W](pmc_slpwk_dr0::W) writer structure"]
impl crate::Writable for PMC_SLPWK_DR0 {}
#[doc = "SleepWalking Disable Register 0"]
pub mod pmc_slpwk_dr0;
#[doc = "SleepWalking Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_sr0](pmc_slpwk_sr0) module"]
pub type PMC_SLPWK_SR0 = crate::Reg<u32, _PMC_SLPWK_SR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_SR0;
#[doc = "`read()` method returns [pmc_slpwk_sr0::R](pmc_slpwk_sr0::R) reader structure"]
impl crate::Readable for PMC_SLPWK_SR0 {}
#[doc = "SleepWalking Status Register 0"]
pub mod pmc_slpwk_sr0;
#[doc = "SleepWalking Activity Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_asr0](pmc_slpwk_asr0) module"]
pub type PMC_SLPWK_ASR0 = crate::Reg<u32, _PMC_SLPWK_ASR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_ASR0;
#[doc = "`read()` method returns [pmc_slpwk_asr0::R](pmc_slpwk_asr0::R) reader structure"]
impl crate::Readable for PMC_SLPWK_ASR0 {}
#[doc = "SleepWalking Activity Status Register 0"]
pub mod pmc_slpwk_asr0;
#[doc = "PLL Maximum Multiplier Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pmmr](pmc_pmmr) module"]
pub type PMC_PMMR = crate::Reg<u32, _PMC_PMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_PMMR;
#[doc = "`read()` method returns [pmc_pmmr::R](pmc_pmmr::R) reader structure"]
impl crate::Readable for PMC_PMMR {}
#[doc = "`write(|w| ..)` method takes [pmc_pmmr::W](pmc_pmmr::W) writer structure"]
impl crate::Writable for PMC_PMMR {}
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmc_pmmr;
#[doc = "SleepWalking Enable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_er1](pmc_slpwk_er1) module"]
pub type PMC_SLPWK_ER1 = crate::Reg<u32, _PMC_SLPWK_ER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_ER1;
#[doc = "`write(|w| ..)` method takes [pmc_slpwk_er1::W](pmc_slpwk_er1::W) writer structure"]
impl crate::Writable for PMC_SLPWK_ER1 {}
#[doc = "SleepWalking Enable Register 1"]
pub mod pmc_slpwk_er1;
#[doc = "SleepWalking Disable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_dr1](pmc_slpwk_dr1) module"]
pub type PMC_SLPWK_DR1 = crate::Reg<u32, _PMC_SLPWK_DR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_DR1;
#[doc = "`write(|w| ..)` method takes [pmc_slpwk_dr1::W](pmc_slpwk_dr1::W) writer structure"]
impl crate::Writable for PMC_SLPWK_DR1 {}
#[doc = "SleepWalking Disable Register 1"]
pub mod pmc_slpwk_dr1;
#[doc = "SleepWalking Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_sr1](pmc_slpwk_sr1) module"]
pub type PMC_SLPWK_SR1 = crate::Reg<u32, _PMC_SLPWK_SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_SR1;
#[doc = "`read()` method returns [pmc_slpwk_sr1::R](pmc_slpwk_sr1::R) reader structure"]
impl crate::Readable for PMC_SLPWK_SR1 {}
#[doc = "SleepWalking Status Register 1"]
pub mod pmc_slpwk_sr1;
#[doc = "SleepWalking Activity Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_asr1](pmc_slpwk_asr1) module"]
pub type PMC_SLPWK_ASR1 = crate::Reg<u32, _PMC_SLPWK_ASR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_ASR1;
#[doc = "`read()` method returns [pmc_slpwk_asr1::R](pmc_slpwk_asr1::R) reader structure"]
impl crate::Readable for PMC_SLPWK_ASR1 {}
#[doc = "SleepWalking Activity Status Register 1"]
pub mod pmc_slpwk_asr1;
#[doc = "SleepWalking Activity In Progress Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_slpwk_aipr](pmc_slpwk_aipr) module"]
pub type PMC_SLPWK_AIPR = crate::Reg<u32, _PMC_SLPWK_AIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC_SLPWK_AIPR;
#[doc = "`read()` method returns [pmc_slpwk_aipr::R](pmc_slpwk_aipr::R) reader structure"]
impl crate::Readable for PMC_SLPWK_AIPR {}
#[doc = "SleepWalking Activity In Progress Register"]
pub mod pmc_slpwk_aipr;
