#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    pub pio_per: PIO_PER,
    #[doc = "0x04 - PIO Disable Register"]
    pub pio_pdr: PIO_PDR,
    #[doc = "0x08 - PIO Status Register"]
    pub pio_psr: PIO_PSR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Output Enable Register"]
    pub pio_oer: PIO_OER,
    #[doc = "0x14 - Output Disable Register"]
    pub pio_odr: PIO_ODR,
    #[doc = "0x18 - Output Status Register"]
    pub pio_osr: PIO_OSR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    pub pio_ifer: PIO_IFER,
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    pub pio_ifdr: PIO_IFDR,
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    pub pio_ifsr: PIO_IFSR,
    _reserved9: [u8; 4usize],
    #[doc = "0x30 - Set Output Data Register"]
    pub pio_sodr: PIO_SODR,
    #[doc = "0x34 - Clear Output Data Register"]
    pub pio_codr: PIO_CODR,
    #[doc = "0x38 - Output Data Status Register"]
    pub pio_odsr: PIO_ODSR,
    #[doc = "0x3c - Pin Data Status Register"]
    pub pio_pdsr: PIO_PDSR,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub pio_ier: PIO_IER,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub pio_idr: PIO_IDR,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub pio_imr: PIO_IMR,
    #[doc = "0x4c - Interrupt Status Register"]
    pub pio_isr: PIO_ISR,
    #[doc = "0x50 - Multi-driver Enable Register"]
    pub pio_mder: PIO_MDER,
    #[doc = "0x54 - Multi-driver Disable Register"]
    pub pio_mddr: PIO_MDDR,
    #[doc = "0x58 - Multi-driver Status Register"]
    pub pio_mdsr: PIO_MDSR,
    _reserved20: [u8; 4usize],
    #[doc = "0x60 - Pull-up Disable Register"]
    pub pio_pudr: PIO_PUDR,
    #[doc = "0x64 - Pull-up Enable Register"]
    pub pio_puer: PIO_PUER,
    #[doc = "0x68 - Pad Pull-up Status Register"]
    pub pio_pusr: PIO_PUSR,
    _reserved23: [u8; 4usize],
    #[doc = "0x70 - Peripheral ABCD Select Register 0"]
    pub pio_abcdsr: [PIO_ABCDSR; 2],
    _reserved24: [u8; 8usize],
    #[doc = "0x80 - Input Filter Slow Clock Disable Register"]
    pub pio_ifscdr: PIO_IFSCDR,
    #[doc = "0x84 - Input Filter Slow Clock Enable Register"]
    pub pio_ifscer: PIO_IFSCER,
    #[doc = "0x88 - Input Filter Slow Clock Status Register"]
    pub pio_ifscsr: PIO_IFSCSR,
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    pub pio_scdr: PIO_SCDR,
    #[doc = "0x90 - Pad Pull-down Disable Register"]
    pub pio_ppddr: PIO_PPDDR,
    #[doc = "0x94 - Pad Pull-down Enable Register"]
    pub pio_ppder: PIO_PPDER,
    #[doc = "0x98 - Pad Pull-down Status Register"]
    pub pio_ppdsr: PIO_PPDSR,
    _reserved31: [u8; 4usize],
    #[doc = "0xa0 - Output Write Enable"]
    pub pio_ower: PIO_OWER,
    #[doc = "0xa4 - Output Write Disable"]
    pub pio_owdr: PIO_OWDR,
    #[doc = "0xa8 - Output Write Status Register"]
    pub pio_owsr: PIO_OWSR,
    _reserved34: [u8; 4usize],
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    pub pio_aimer: PIO_AIMER,
    #[doc = "0xb4 - Additional Interrupt Modes Disable Register"]
    pub pio_aimdr: PIO_AIMDR,
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    pub pio_aimmr: PIO_AIMMR,
    _reserved37: [u8; 4usize],
    #[doc = "0xc0 - Edge Select Register"]
    pub pio_esr: PIO_ESR,
    #[doc = "0xc4 - Level Select Register"]
    pub pio_lsr: PIO_LSR,
    #[doc = "0xc8 - Edge/Level Status Register"]
    pub pio_elsr: PIO_ELSR,
    _reserved40: [u8; 4usize],
    #[doc = "0xd0 - Falling Edge/Low-Level Select Register"]
    pub pio_fellsr: PIO_FELLSR,
    #[doc = "0xd4 - Rising Edge/High-Level Select Register"]
    pub pio_rehlsr: PIO_REHLSR,
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    pub pio_frlhsr: PIO_FRLHSR,
    _reserved43: [u8; 4usize],
    #[doc = "0xe0 - Lock Status"]
    pub pio_locksr: PIO_LOCKSR,
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub pio_wpmr: PIO_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub pio_wpsr: PIO_WPSR,
    _reserved46: [u8; 20usize],
    #[doc = "0x100 - Schmitt Trigger Register"]
    pub pio_schmitt: PIO_SCHMITT,
    _reserved47: [u8; 20usize],
    #[doc = "0x118 - I/O Drive Register"]
    pub pio_driver: PIO_DRIVER,
    _reserved48: [u8; 52usize],
    #[doc = "0x150 - Parallel Capture Mode Register"]
    pub pio_pcmr: PIO_PCMR,
    #[doc = "0x154 - Parallel Capture Interrupt Enable Register"]
    pub pio_pcier: PIO_PCIER,
    #[doc = "0x158 - Parallel Capture Interrupt Disable Register"]
    pub pio_pcidr: PIO_PCIDR,
    #[doc = "0x15c - Parallel Capture Interrupt Mask Register"]
    pub pio_pcimr: PIO_PCIMR,
    #[doc = "0x160 - Parallel Capture Interrupt Status Register"]
    pub pio_pcisr: PIO_PCISR,
    #[doc = "0x164 - Parallel Capture Reception Holding Register"]
    pub pio_pcrhr: PIO_PCRHR,
}
#[doc = "PIO Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_per](pio_per) module"]
pub type PIO_PER = crate::Reg<u32, _PIO_PER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PER;
#[doc = "`write(|w| ..)` method takes [pio_per::W](pio_per::W) writer structure"]
impl crate::Writable for PIO_PER {}
#[doc = "PIO Enable Register"]
pub mod pio_per;
#[doc = "PIO Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pdr](pio_pdr) module"]
pub type PIO_PDR = crate::Reg<u32, _PIO_PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PDR;
#[doc = "`write(|w| ..)` method takes [pio_pdr::W](pio_pdr::W) writer structure"]
impl crate::Writable for PIO_PDR {}
#[doc = "PIO Disable Register"]
pub mod pio_pdr;
#[doc = "PIO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_psr](pio_psr) module"]
pub type PIO_PSR = crate::Reg<u32, _PIO_PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PSR;
#[doc = "`read()` method returns [pio_psr::R](pio_psr::R) reader structure"]
impl crate::Readable for PIO_PSR {}
#[doc = "PIO Status Register"]
pub mod pio_psr;
#[doc = "Output Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_oer](pio_oer) module"]
pub type PIO_OER = crate::Reg<u32, _PIO_OER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_OER;
#[doc = "`write(|w| ..)` method takes [pio_oer::W](pio_oer::W) writer structure"]
impl crate::Writable for PIO_OER {}
#[doc = "Output Enable Register"]
pub mod pio_oer;
#[doc = "Output Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_odr](pio_odr) module"]
pub type PIO_ODR = crate::Reg<u32, _PIO_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_ODR;
#[doc = "`write(|w| ..)` method takes [pio_odr::W](pio_odr::W) writer structure"]
impl crate::Writable for PIO_ODR {}
#[doc = "Output Disable Register"]
pub mod pio_odr;
#[doc = "Output Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_osr](pio_osr) module"]
pub type PIO_OSR = crate::Reg<u32, _PIO_OSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_OSR;
#[doc = "`read()` method returns [pio_osr::R](pio_osr::R) reader structure"]
impl crate::Readable for PIO_OSR {}
#[doc = "Output Status Register"]
pub mod pio_osr;
#[doc = "Glitch Input Filter Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ifer](pio_ifer) module"]
pub type PIO_IFER = crate::Reg<u32, _PIO_IFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IFER;
#[doc = "`write(|w| ..)` method takes [pio_ifer::W](pio_ifer::W) writer structure"]
impl crate::Writable for PIO_IFER {}
#[doc = "Glitch Input Filter Enable Register"]
pub mod pio_ifer;
#[doc = "Glitch Input Filter Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ifdr](pio_ifdr) module"]
pub type PIO_IFDR = crate::Reg<u32, _PIO_IFDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IFDR;
#[doc = "`write(|w| ..)` method takes [pio_ifdr::W](pio_ifdr::W) writer structure"]
impl crate::Writable for PIO_IFDR {}
#[doc = "Glitch Input Filter Disable Register"]
pub mod pio_ifdr;
#[doc = "Glitch Input Filter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ifsr](pio_ifsr) module"]
pub type PIO_IFSR = crate::Reg<u32, _PIO_IFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IFSR;
#[doc = "`read()` method returns [pio_ifsr::R](pio_ifsr::R) reader structure"]
impl crate::Readable for PIO_IFSR {}
#[doc = "Glitch Input Filter Status Register"]
pub mod pio_ifsr;
#[doc = "Set Output Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_sodr](pio_sodr) module"]
pub type PIO_SODR = crate::Reg<u32, _PIO_SODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_SODR;
#[doc = "`write(|w| ..)` method takes [pio_sodr::W](pio_sodr::W) writer structure"]
impl crate::Writable for PIO_SODR {}
#[doc = "Set Output Data Register"]
pub mod pio_sodr;
#[doc = "Clear Output Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_codr](pio_codr) module"]
pub type PIO_CODR = crate::Reg<u32, _PIO_CODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_CODR;
#[doc = "`write(|w| ..)` method takes [pio_codr::W](pio_codr::W) writer structure"]
impl crate::Writable for PIO_CODR {}
#[doc = "Clear Output Data Register"]
pub mod pio_codr;
#[doc = "Output Data Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_odsr](pio_odsr) module"]
pub type PIO_ODSR = crate::Reg<u32, _PIO_ODSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_ODSR;
#[doc = "`read()` method returns [pio_odsr::R](pio_odsr::R) reader structure"]
impl crate::Readable for PIO_ODSR {}
#[doc = "`write(|w| ..)` method takes [pio_odsr::W](pio_odsr::W) writer structure"]
impl crate::Writable for PIO_ODSR {}
#[doc = "Output Data Status Register"]
pub mod pio_odsr;
#[doc = "Pin Data Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pdsr](pio_pdsr) module"]
pub type PIO_PDSR = crate::Reg<u32, _PIO_PDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PDSR;
#[doc = "`read()` method returns [pio_pdsr::R](pio_pdsr::R) reader structure"]
impl crate::Readable for PIO_PDSR {}
#[doc = "Pin Data Status Register"]
pub mod pio_pdsr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ier](pio_ier) module"]
pub type PIO_IER = crate::Reg<u32, _PIO_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IER;
#[doc = "`write(|w| ..)` method takes [pio_ier::W](pio_ier::W) writer structure"]
impl crate::Writable for PIO_IER {}
#[doc = "Interrupt Enable Register"]
pub mod pio_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_idr](pio_idr) module"]
pub type PIO_IDR = crate::Reg<u32, _PIO_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IDR;
#[doc = "`write(|w| ..)` method takes [pio_idr::W](pio_idr::W) writer structure"]
impl crate::Writable for PIO_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod pio_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_imr](pio_imr) module"]
pub type PIO_IMR = crate::Reg<u32, _PIO_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IMR;
#[doc = "`read()` method returns [pio_imr::R](pio_imr::R) reader structure"]
impl crate::Readable for PIO_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod pio_imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_isr](pio_isr) module"]
pub type PIO_ISR = crate::Reg<u32, _PIO_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_ISR;
#[doc = "`read()` method returns [pio_isr::R](pio_isr::R) reader structure"]
impl crate::Readable for PIO_ISR {}
#[doc = "Interrupt Status Register"]
pub mod pio_isr;
#[doc = "Multi-driver Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_mder](pio_mder) module"]
pub type PIO_MDER = crate::Reg<u32, _PIO_MDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_MDER;
#[doc = "`write(|w| ..)` method takes [pio_mder::W](pio_mder::W) writer structure"]
impl crate::Writable for PIO_MDER {}
#[doc = "Multi-driver Enable Register"]
pub mod pio_mder;
#[doc = "Multi-driver Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_mddr](pio_mddr) module"]
pub type PIO_MDDR = crate::Reg<u32, _PIO_MDDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_MDDR;
#[doc = "`write(|w| ..)` method takes [pio_mddr::W](pio_mddr::W) writer structure"]
impl crate::Writable for PIO_MDDR {}
#[doc = "Multi-driver Disable Register"]
pub mod pio_mddr;
#[doc = "Multi-driver Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_mdsr](pio_mdsr) module"]
pub type PIO_MDSR = crate::Reg<u32, _PIO_MDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_MDSR;
#[doc = "`read()` method returns [pio_mdsr::R](pio_mdsr::R) reader structure"]
impl crate::Readable for PIO_MDSR {}
#[doc = "Multi-driver Status Register"]
pub mod pio_mdsr;
#[doc = "Pull-up Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pudr](pio_pudr) module"]
pub type PIO_PUDR = crate::Reg<u32, _PIO_PUDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PUDR;
#[doc = "`write(|w| ..)` method takes [pio_pudr::W](pio_pudr::W) writer structure"]
impl crate::Writable for PIO_PUDR {}
#[doc = "Pull-up Disable Register"]
pub mod pio_pudr;
#[doc = "Pull-up Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_puer](pio_puer) module"]
pub type PIO_PUER = crate::Reg<u32, _PIO_PUER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PUER;
#[doc = "`write(|w| ..)` method takes [pio_puer::W](pio_puer::W) writer structure"]
impl crate::Writable for PIO_PUER {}
#[doc = "Pull-up Enable Register"]
pub mod pio_puer;
#[doc = "Pad Pull-up Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pusr](pio_pusr) module"]
pub type PIO_PUSR = crate::Reg<u32, _PIO_PUSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PUSR;
#[doc = "`read()` method returns [pio_pusr::R](pio_pusr::R) reader structure"]
impl crate::Readable for PIO_PUSR {}
#[doc = "Pad Pull-up Status Register"]
pub mod pio_pusr;
#[doc = "Peripheral ABCD Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_abcdsr](pio_abcdsr) module"]
pub type PIO_ABCDSR = crate::Reg<u32, _PIO_ABCDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_ABCDSR;
#[doc = "`read()` method returns [pio_abcdsr::R](pio_abcdsr::R) reader structure"]
impl crate::Readable for PIO_ABCDSR {}
#[doc = "`write(|w| ..)` method takes [pio_abcdsr::W](pio_abcdsr::W) writer structure"]
impl crate::Writable for PIO_ABCDSR {}
#[doc = "Peripheral ABCD Select Register 0"]
pub mod pio_abcdsr;
#[doc = "Input Filter Slow Clock Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ifscdr](pio_ifscdr) module"]
pub type PIO_IFSCDR = crate::Reg<u32, _PIO_IFSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IFSCDR;
#[doc = "`write(|w| ..)` method takes [pio_ifscdr::W](pio_ifscdr::W) writer structure"]
impl crate::Writable for PIO_IFSCDR {}
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod pio_ifscdr;
#[doc = "Input Filter Slow Clock Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ifscer](pio_ifscer) module"]
pub type PIO_IFSCER = crate::Reg<u32, _PIO_IFSCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IFSCER;
#[doc = "`write(|w| ..)` method takes [pio_ifscer::W](pio_ifscer::W) writer structure"]
impl crate::Writable for PIO_IFSCER {}
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod pio_ifscer;
#[doc = "Input Filter Slow Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ifscsr](pio_ifscsr) module"]
pub type PIO_IFSCSR = crate::Reg<u32, _PIO_IFSCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_IFSCSR;
#[doc = "`read()` method returns [pio_ifscsr::R](pio_ifscsr::R) reader structure"]
impl crate::Readable for PIO_IFSCSR {}
#[doc = "Input Filter Slow Clock Status Register"]
pub mod pio_ifscsr;
#[doc = "Slow Clock Divider Debouncing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_scdr](pio_scdr) module"]
pub type PIO_SCDR = crate::Reg<u32, _PIO_SCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_SCDR;
#[doc = "`read()` method returns [pio_scdr::R](pio_scdr::R) reader structure"]
impl crate::Readable for PIO_SCDR {}
#[doc = "`write(|w| ..)` method takes [pio_scdr::W](pio_scdr::W) writer structure"]
impl crate::Writable for PIO_SCDR {}
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod pio_scdr;
#[doc = "Pad Pull-down Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ppddr](pio_ppddr) module"]
pub type PIO_PPDDR = crate::Reg<u32, _PIO_PPDDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PPDDR;
#[doc = "`write(|w| ..)` method takes [pio_ppddr::W](pio_ppddr::W) writer structure"]
impl crate::Writable for PIO_PPDDR {}
#[doc = "Pad Pull-down Disable Register"]
pub mod pio_ppddr;
#[doc = "Pad Pull-down Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ppder](pio_ppder) module"]
pub type PIO_PPDER = crate::Reg<u32, _PIO_PPDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PPDER;
#[doc = "`write(|w| ..)` method takes [pio_ppder::W](pio_ppder::W) writer structure"]
impl crate::Writable for PIO_PPDER {}
#[doc = "Pad Pull-down Enable Register"]
pub mod pio_ppder;
#[doc = "Pad Pull-down Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ppdsr](pio_ppdsr) module"]
pub type PIO_PPDSR = crate::Reg<u32, _PIO_PPDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PPDSR;
#[doc = "`read()` method returns [pio_ppdsr::R](pio_ppdsr::R) reader structure"]
impl crate::Readable for PIO_PPDSR {}
#[doc = "Pad Pull-down Status Register"]
pub mod pio_ppdsr;
#[doc = "Output Write Enable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_ower](pio_ower) module"]
pub type PIO_OWER = crate::Reg<u32, _PIO_OWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_OWER;
#[doc = "`write(|w| ..)` method takes [pio_ower::W](pio_ower::W) writer structure"]
impl crate::Writable for PIO_OWER {}
#[doc = "Output Write Enable"]
pub mod pio_ower;
#[doc = "Output Write Disable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_owdr](pio_owdr) module"]
pub type PIO_OWDR = crate::Reg<u32, _PIO_OWDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_OWDR;
#[doc = "`write(|w| ..)` method takes [pio_owdr::W](pio_owdr::W) writer structure"]
impl crate::Writable for PIO_OWDR {}
#[doc = "Output Write Disable"]
pub mod pio_owdr;
#[doc = "Output Write Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_owsr](pio_owsr) module"]
pub type PIO_OWSR = crate::Reg<u32, _PIO_OWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_OWSR;
#[doc = "`read()` method returns [pio_owsr::R](pio_owsr::R) reader structure"]
impl crate::Readable for PIO_OWSR {}
#[doc = "Output Write Status Register"]
pub mod pio_owsr;
#[doc = "Additional Interrupt Modes Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_aimer](pio_aimer) module"]
pub type PIO_AIMER = crate::Reg<u32, _PIO_AIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_AIMER;
#[doc = "`write(|w| ..)` method takes [pio_aimer::W](pio_aimer::W) writer structure"]
impl crate::Writable for PIO_AIMER {}
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod pio_aimer;
#[doc = "Additional Interrupt Modes Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_aimdr](pio_aimdr) module"]
pub type PIO_AIMDR = crate::Reg<u32, _PIO_AIMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_AIMDR;
#[doc = "`write(|w| ..)` method takes [pio_aimdr::W](pio_aimdr::W) writer structure"]
impl crate::Writable for PIO_AIMDR {}
#[doc = "Additional Interrupt Modes Disable Register"]
pub mod pio_aimdr;
#[doc = "Additional Interrupt Modes Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_aimmr](pio_aimmr) module"]
pub type PIO_AIMMR = crate::Reg<u32, _PIO_AIMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_AIMMR;
#[doc = "`read()` method returns [pio_aimmr::R](pio_aimmr::R) reader structure"]
impl crate::Readable for PIO_AIMMR {}
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod pio_aimmr;
#[doc = "Edge Select Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_esr](pio_esr) module"]
pub type PIO_ESR = crate::Reg<u32, _PIO_ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_ESR;
#[doc = "`write(|w| ..)` method takes [pio_esr::W](pio_esr::W) writer structure"]
impl crate::Writable for PIO_ESR {}
#[doc = "Edge Select Register"]
pub mod pio_esr;
#[doc = "Level Select Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_lsr](pio_lsr) module"]
pub type PIO_LSR = crate::Reg<u32, _PIO_LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_LSR;
#[doc = "`write(|w| ..)` method takes [pio_lsr::W](pio_lsr::W) writer structure"]
impl crate::Writable for PIO_LSR {}
#[doc = "Level Select Register"]
pub mod pio_lsr;
#[doc = "Edge/Level Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_elsr](pio_elsr) module"]
pub type PIO_ELSR = crate::Reg<u32, _PIO_ELSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_ELSR;
#[doc = "`read()` method returns [pio_elsr::R](pio_elsr::R) reader structure"]
impl crate::Readable for PIO_ELSR {}
#[doc = "Edge/Level Status Register"]
pub mod pio_elsr;
#[doc = "Falling Edge/Low-Level Select Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_fellsr](pio_fellsr) module"]
pub type PIO_FELLSR = crate::Reg<u32, _PIO_FELLSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_FELLSR;
#[doc = "`write(|w| ..)` method takes [pio_fellsr::W](pio_fellsr::W) writer structure"]
impl crate::Writable for PIO_FELLSR {}
#[doc = "Falling Edge/Low-Level Select Register"]
pub mod pio_fellsr;
#[doc = "Rising Edge/High-Level Select Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_rehlsr](pio_rehlsr) module"]
pub type PIO_REHLSR = crate::Reg<u32, _PIO_REHLSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_REHLSR;
#[doc = "`write(|w| ..)` method takes [pio_rehlsr::W](pio_rehlsr::W) writer structure"]
impl crate::Writable for PIO_REHLSR {}
#[doc = "Rising Edge/High-Level Select Register"]
pub mod pio_rehlsr;
#[doc = "Fall/Rise - Low/High Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_frlhsr](pio_frlhsr) module"]
pub type PIO_FRLHSR = crate::Reg<u32, _PIO_FRLHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_FRLHSR;
#[doc = "`read()` method returns [pio_frlhsr::R](pio_frlhsr::R) reader structure"]
impl crate::Readable for PIO_FRLHSR {}
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod pio_frlhsr;
#[doc = "Lock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_locksr](pio_locksr) module"]
pub type PIO_LOCKSR = crate::Reg<u32, _PIO_LOCKSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_LOCKSR;
#[doc = "`read()` method returns [pio_locksr::R](pio_locksr::R) reader structure"]
impl crate::Readable for PIO_LOCKSR {}
#[doc = "Lock Status"]
pub mod pio_locksr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_wpmr](pio_wpmr) module"]
pub type PIO_WPMR = crate::Reg<u32, _PIO_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_WPMR;
#[doc = "`read()` method returns [pio_wpmr::R](pio_wpmr::R) reader structure"]
impl crate::Readable for PIO_WPMR {}
#[doc = "`write(|w| ..)` method takes [pio_wpmr::W](pio_wpmr::W) writer structure"]
impl crate::Writable for PIO_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod pio_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_wpsr](pio_wpsr) module"]
pub type PIO_WPSR = crate::Reg<u32, _PIO_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_WPSR;
#[doc = "`read()` method returns [pio_wpsr::R](pio_wpsr::R) reader structure"]
impl crate::Readable for PIO_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod pio_wpsr;
#[doc = "Schmitt Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_schmitt](pio_schmitt) module"]
pub type PIO_SCHMITT = crate::Reg<u32, _PIO_SCHMITT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_SCHMITT;
#[doc = "`read()` method returns [pio_schmitt::R](pio_schmitt::R) reader structure"]
impl crate::Readable for PIO_SCHMITT {}
#[doc = "`write(|w| ..)` method takes [pio_schmitt::W](pio_schmitt::W) writer structure"]
impl crate::Writable for PIO_SCHMITT {}
#[doc = "Schmitt Trigger Register"]
pub mod pio_schmitt;
#[doc = "I/O Drive Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_driver](pio_driver) module"]
pub type PIO_DRIVER = crate::Reg<u32, _PIO_DRIVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_DRIVER;
#[doc = "`read()` method returns [pio_driver::R](pio_driver::R) reader structure"]
impl crate::Readable for PIO_DRIVER {}
#[doc = "`write(|w| ..)` method takes [pio_driver::W](pio_driver::W) writer structure"]
impl crate::Writable for PIO_DRIVER {}
#[doc = "I/O Drive Register"]
pub mod pio_driver;
#[doc = "Parallel Capture Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcmr](pio_pcmr) module"]
pub type PIO_PCMR = crate::Reg<u32, _PIO_PCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PCMR;
#[doc = "`read()` method returns [pio_pcmr::R](pio_pcmr::R) reader structure"]
impl crate::Readable for PIO_PCMR {}
#[doc = "`write(|w| ..)` method takes [pio_pcmr::W](pio_pcmr::W) writer structure"]
impl crate::Writable for PIO_PCMR {}
#[doc = "Parallel Capture Mode Register"]
pub mod pio_pcmr;
#[doc = "Parallel Capture Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcier](pio_pcier) module"]
pub type PIO_PCIER = crate::Reg<u32, _PIO_PCIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PCIER;
#[doc = "`write(|w| ..)` method takes [pio_pcier::W](pio_pcier::W) writer structure"]
impl crate::Writable for PIO_PCIER {}
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pio_pcier;
#[doc = "Parallel Capture Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcidr](pio_pcidr) module"]
pub type PIO_PCIDR = crate::Reg<u32, _PIO_PCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PCIDR;
#[doc = "`write(|w| ..)` method takes [pio_pcidr::W](pio_pcidr::W) writer structure"]
impl crate::Writable for PIO_PCIDR {}
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pio_pcidr;
#[doc = "Parallel Capture Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcimr](pio_pcimr) module"]
pub type PIO_PCIMR = crate::Reg<u32, _PIO_PCIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PCIMR;
#[doc = "`read()` method returns [pio_pcimr::R](pio_pcimr::R) reader structure"]
impl crate::Readable for PIO_PCIMR {}
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pio_pcimr;
#[doc = "Parallel Capture Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcisr](pio_pcisr) module"]
pub type PIO_PCISR = crate::Reg<u32, _PIO_PCISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PCISR;
#[doc = "`read()` method returns [pio_pcisr::R](pio_pcisr::R) reader structure"]
impl crate::Readable for PIO_PCISR {}
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pio_pcisr;
#[doc = "Parallel Capture Reception Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcrhr](pio_pcrhr) module"]
pub type PIO_PCRHR = crate::Reg<u32, _PIO_PCRHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO_PCRHR;
#[doc = "`read()` method returns [pio_pcrhr::R](pio_pcrhr::R) reader structure"]
impl crate::Readable for PIO_PCRHR {}
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pio_pcrhr;
