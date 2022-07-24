#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    pub per: crate::Reg<per::PER_SPEC>,
    #[doc = "0x04 - PIO Disable Register"]
    pub pdr: crate::Reg<pdr::PDR_SPEC>,
    #[doc = "0x08 - PIO Status Register"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Output Enable Register"]
    pub oer: crate::Reg<oer::OER_SPEC>,
    #[doc = "0x14 - Output Disable Register"]
    pub odr: crate::Reg<odr::ODR_SPEC>,
    #[doc = "0x18 - Output Status Register"]
    pub osr: crate::Reg<osr::OSR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    pub ifer: crate::Reg<ifer::IFER_SPEC>,
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    pub ifdr: crate::Reg<ifdr::IFDR_SPEC>,
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    pub ifsr: crate::Reg<ifsr::IFSR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Set Output Data Register"]
    pub sodr: crate::Reg<sodr::SODR_SPEC>,
    #[doc = "0x34 - Clear Output Data Register"]
    pub codr: crate::Reg<codr::CODR_SPEC>,
    #[doc = "0x38 - Output Data Status Register"]
    pub odsr: crate::Reg<odsr::ODSR_SPEC>,
    #[doc = "0x3c - Pin Data Status Register"]
    pub pdsr: crate::Reg<pdsr::PDSR_SPEC>,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x4c - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x50 - Multi-driver Enable Register"]
    pub mder: crate::Reg<mder::MDER_SPEC>,
    #[doc = "0x54 - Multi-driver Disable Register"]
    pub mddr: crate::Reg<mddr::MDDR_SPEC>,
    #[doc = "0x58 - Multi-driver Status Register"]
    pub mdsr: crate::Reg<mdsr::MDSR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - Pull-up Disable Register"]
    pub pudr: crate::Reg<pudr::PUDR_SPEC>,
    #[doc = "0x64 - Pull-up Enable Register"]
    pub puer: crate::Reg<puer::PUER_SPEC>,
    #[doc = "0x68 - Pad Pull-up Status Register"]
    pub pusr: crate::Reg<pusr::PUSR_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x70..0x78 - Peripheral ABCD Select Register 0"]
    pub abcdsr: [crate::Reg<abcdsr::ABCDSR_SPEC>; 2],
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - Input Filter Slow Clock Disable Register"]
    pub ifscdr: crate::Reg<ifscdr::IFSCDR_SPEC>,
    #[doc = "0x84 - Input Filter Slow Clock Enable Register"]
    pub ifscer: crate::Reg<ifscer::IFSCER_SPEC>,
    #[doc = "0x88 - Input Filter Slow Clock Status Register"]
    pub ifscsr: crate::Reg<ifscsr::IFSCSR_SPEC>,
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    pub scdr: crate::Reg<scdr::SCDR_SPEC>,
    #[doc = "0x90 - Pad Pull-down Disable Register"]
    pub ppddr: crate::Reg<ppddr::PPDDR_SPEC>,
    #[doc = "0x94 - Pad Pull-down Enable Register"]
    pub ppder: crate::Reg<ppder::PPDER_SPEC>,
    #[doc = "0x98 - Pad Pull-down Status Register"]
    pub ppdsr: crate::Reg<ppdsr::PPDSR_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0xa0 - Output Write Enable"]
    pub ower: crate::Reg<ower::OWER_SPEC>,
    #[doc = "0xa4 - Output Write Disable"]
    pub owdr: crate::Reg<owdr::OWDR_SPEC>,
    #[doc = "0xa8 - Output Write Status Register"]
    pub owsr: crate::Reg<owsr::OWSR_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    pub aimer: crate::Reg<aimer::AIMER_SPEC>,
    #[doc = "0xb4 - Additional Interrupt Modes Disable Register"]
    pub aimdr: crate::Reg<aimdr::AIMDR_SPEC>,
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    pub aimmr: crate::Reg<aimmr::AIMMR_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0xc0 - Edge Select Register"]
    pub esr: crate::Reg<esr::ESR_SPEC>,
    #[doc = "0xc4 - Level Select Register"]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    #[doc = "0xc8 - Edge/Level Status Register"]
    pub elsr: crate::Reg<elsr::ELSR_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0xd0 - Falling Edge/Low-Level Select Register"]
    pub fellsr: crate::Reg<fellsr::FELLSR_SPEC>,
    #[doc = "0xd4 - Rising Edge/High-Level Select Register"]
    pub rehlsr: crate::Reg<rehlsr::REHLSR_SPEC>,
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    pub frlhsr: crate::Reg<frlhsr::FRLHSR_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0xe0 - Lock Status"]
    pub locksr: crate::Reg<locksr::LOCKSR_SPEC>,
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved46: [u8; 0x14],
    #[doc = "0x100 - Schmitt Trigger Register"]
    pub schmitt: crate::Reg<schmitt::SCHMITT_SPEC>,
    _reserved47: [u8; 0x14],
    #[doc = "0x118 - I/O Drive Register"]
    pub driver: crate::Reg<driver::DRIVER_SPEC>,
    _reserved48: [u8; 0x34],
    #[doc = "0x150 - Parallel Capture Mode Register"]
    pub pcmr: crate::Reg<pcmr::PCMR_SPEC>,
    #[doc = "0x154 - Parallel Capture Interrupt Enable Register"]
    pub pcier: crate::Reg<pcier::PCIER_SPEC>,
    #[doc = "0x158 - Parallel Capture Interrupt Disable Register"]
    pub pcidr: crate::Reg<pcidr::PCIDR_SPEC>,
    #[doc = "0x15c - Parallel Capture Interrupt Mask Register"]
    pub pcimr: crate::Reg<pcimr::PCIMR_SPEC>,
    #[doc = "0x160 - Parallel Capture Interrupt Status Register"]
    pub pcisr: crate::Reg<pcisr::PCISR_SPEC>,
    #[doc = "0x164 - Parallel Capture Reception Holding Register"]
    pub pcrhr: crate::Reg<pcrhr::PCRHR_SPEC>,
}
#[doc = "PER register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "PIO Enable Register"]
pub mod per;
#[doc = "PDR register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "PIO Disable Register"]
pub mod pdr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "PIO Status Register"]
pub mod psr;
#[doc = "OER register accessor: an alias for `Reg<OER_SPEC>`"]
pub type OER = crate::Reg<oer::OER_SPEC>;
#[doc = "Output Enable Register"]
pub mod oer;
#[doc = "ODR register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "Output Disable Register"]
pub mod odr;
#[doc = "OSR register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Output Status Register"]
pub mod osr;
#[doc = "IFER register accessor: an alias for `Reg<IFER_SPEC>`"]
pub type IFER = crate::Reg<ifer::IFER_SPEC>;
#[doc = "Glitch Input Filter Enable Register"]
pub mod ifer;
#[doc = "IFDR register accessor: an alias for `Reg<IFDR_SPEC>`"]
pub type IFDR = crate::Reg<ifdr::IFDR_SPEC>;
#[doc = "Glitch Input Filter Disable Register"]
pub mod ifdr;
#[doc = "IFSR register accessor: an alias for `Reg<IFSR_SPEC>`"]
pub type IFSR = crate::Reg<ifsr::IFSR_SPEC>;
#[doc = "Glitch Input Filter Status Register"]
pub mod ifsr;
#[doc = "SODR register accessor: an alias for `Reg<SODR_SPEC>`"]
pub type SODR = crate::Reg<sodr::SODR_SPEC>;
#[doc = "Set Output Data Register"]
pub mod sodr;
#[doc = "CODR register accessor: an alias for `Reg<CODR_SPEC>`"]
pub type CODR = crate::Reg<codr::CODR_SPEC>;
#[doc = "Clear Output Data Register"]
pub mod codr;
#[doc = "ODSR register accessor: an alias for `Reg<ODSR_SPEC>`"]
pub type ODSR = crate::Reg<odsr::ODSR_SPEC>;
#[doc = "Output Data Status Register"]
pub mod odsr;
#[doc = "PDSR register accessor: an alias for `Reg<PDSR_SPEC>`"]
pub type PDSR = crate::Reg<pdsr::PDSR_SPEC>;
#[doc = "Pin Data Status Register"]
pub mod pdsr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "MDER register accessor: an alias for `Reg<MDER_SPEC>`"]
pub type MDER = crate::Reg<mder::MDER_SPEC>;
#[doc = "Multi-driver Enable Register"]
pub mod mder;
#[doc = "MDDR register accessor: an alias for `Reg<MDDR_SPEC>`"]
pub type MDDR = crate::Reg<mddr::MDDR_SPEC>;
#[doc = "Multi-driver Disable Register"]
pub mod mddr;
#[doc = "MDSR register accessor: an alias for `Reg<MDSR_SPEC>`"]
pub type MDSR = crate::Reg<mdsr::MDSR_SPEC>;
#[doc = "Multi-driver Status Register"]
pub mod mdsr;
#[doc = "PUDR register accessor: an alias for `Reg<PUDR_SPEC>`"]
pub type PUDR = crate::Reg<pudr::PUDR_SPEC>;
#[doc = "Pull-up Disable Register"]
pub mod pudr;
#[doc = "PUER register accessor: an alias for `Reg<PUER_SPEC>`"]
pub type PUER = crate::Reg<puer::PUER_SPEC>;
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "PUSR register accessor: an alias for `Reg<PUSR_SPEC>`"]
pub type PUSR = crate::Reg<pusr::PUSR_SPEC>;
#[doc = "Pad Pull-up Status Register"]
pub mod pusr;
#[doc = "ABCDSR register accessor: an alias for `Reg<ABCDSR_SPEC>`"]
pub type ABCDSR = crate::Reg<abcdsr::ABCDSR_SPEC>;
#[doc = "Peripheral ABCD Select Register 0"]
pub mod abcdsr;
#[doc = "IFSCDR register accessor: an alias for `Reg<IFSCDR_SPEC>`"]
pub type IFSCDR = crate::Reg<ifscdr::IFSCDR_SPEC>;
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod ifscdr;
#[doc = "IFSCER register accessor: an alias for `Reg<IFSCER_SPEC>`"]
pub type IFSCER = crate::Reg<ifscer::IFSCER_SPEC>;
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod ifscer;
#[doc = "IFSCSR register accessor: an alias for `Reg<IFSCSR_SPEC>`"]
pub type IFSCSR = crate::Reg<ifscsr::IFSCSR_SPEC>;
#[doc = "Input Filter Slow Clock Status Register"]
pub mod ifscsr;
#[doc = "SCDR register accessor: an alias for `Reg<SCDR_SPEC>`"]
pub type SCDR = crate::Reg<scdr::SCDR_SPEC>;
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "PPDDR register accessor: an alias for `Reg<PPDDR_SPEC>`"]
pub type PPDDR = crate::Reg<ppddr::PPDDR_SPEC>;
#[doc = "Pad Pull-down Disable Register"]
pub mod ppddr;
#[doc = "PPDER register accessor: an alias for `Reg<PPDER_SPEC>`"]
pub type PPDER = crate::Reg<ppder::PPDER_SPEC>;
#[doc = "Pad Pull-down Enable Register"]
pub mod ppder;
#[doc = "PPDSR register accessor: an alias for `Reg<PPDSR_SPEC>`"]
pub type PPDSR = crate::Reg<ppdsr::PPDSR_SPEC>;
#[doc = "Pad Pull-down Status Register"]
pub mod ppdsr;
#[doc = "OWER register accessor: an alias for `Reg<OWER_SPEC>`"]
pub type OWER = crate::Reg<ower::OWER_SPEC>;
#[doc = "Output Write Enable"]
pub mod ower;
#[doc = "OWDR register accessor: an alias for `Reg<OWDR_SPEC>`"]
pub type OWDR = crate::Reg<owdr::OWDR_SPEC>;
#[doc = "Output Write Disable"]
pub mod owdr;
#[doc = "OWSR register accessor: an alias for `Reg<OWSR_SPEC>`"]
pub type OWSR = crate::Reg<owsr::OWSR_SPEC>;
#[doc = "Output Write Status Register"]
pub mod owsr;
#[doc = "AIMER register accessor: an alias for `Reg<AIMER_SPEC>`"]
pub type AIMER = crate::Reg<aimer::AIMER_SPEC>;
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod aimer;
#[doc = "AIMDR register accessor: an alias for `Reg<AIMDR_SPEC>`"]
pub type AIMDR = crate::Reg<aimdr::AIMDR_SPEC>;
#[doc = "Additional Interrupt Modes Disable Register"]
pub mod aimdr;
#[doc = "AIMMR register accessor: an alias for `Reg<AIMMR_SPEC>`"]
pub type AIMMR = crate::Reg<aimmr::AIMMR_SPEC>;
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod aimmr;
#[doc = "ESR register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "Edge Select Register"]
pub mod esr;
#[doc = "LSR register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Level Select Register"]
pub mod lsr;
#[doc = "ELSR register accessor: an alias for `Reg<ELSR_SPEC>`"]
pub type ELSR = crate::Reg<elsr::ELSR_SPEC>;
#[doc = "Edge/Level Status Register"]
pub mod elsr;
#[doc = "FELLSR register accessor: an alias for `Reg<FELLSR_SPEC>`"]
pub type FELLSR = crate::Reg<fellsr::FELLSR_SPEC>;
#[doc = "Falling Edge/Low-Level Select Register"]
pub mod fellsr;
#[doc = "REHLSR register accessor: an alias for `Reg<REHLSR_SPEC>`"]
pub type REHLSR = crate::Reg<rehlsr::REHLSR_SPEC>;
#[doc = "Rising Edge/High-Level Select Register"]
pub mod rehlsr;
#[doc = "FRLHSR register accessor: an alias for `Reg<FRLHSR_SPEC>`"]
pub type FRLHSR = crate::Reg<frlhsr::FRLHSR_SPEC>;
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod frlhsr;
#[doc = "LOCKSR register accessor: an alias for `Reg<LOCKSR_SPEC>`"]
pub type LOCKSR = crate::Reg<locksr::LOCKSR_SPEC>;
#[doc = "Lock Status"]
pub mod locksr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "SCHMITT register accessor: an alias for `Reg<SCHMITT_SPEC>`"]
pub type SCHMITT = crate::Reg<schmitt::SCHMITT_SPEC>;
#[doc = "Schmitt Trigger Register"]
pub mod schmitt;
#[doc = "DRIVER register accessor: an alias for `Reg<DRIVER_SPEC>`"]
pub type DRIVER = crate::Reg<driver::DRIVER_SPEC>;
#[doc = "I/O Drive Register"]
pub mod driver;
#[doc = "PCMR register accessor: an alias for `Reg<PCMR_SPEC>`"]
pub type PCMR = crate::Reg<pcmr::PCMR_SPEC>;
#[doc = "Parallel Capture Mode Register"]
pub mod pcmr;
#[doc = "PCIER register accessor: an alias for `Reg<PCIER_SPEC>`"]
pub type PCIER = crate::Reg<pcier::PCIER_SPEC>;
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pcier;
#[doc = "PCIDR register accessor: an alias for `Reg<PCIDR_SPEC>`"]
pub type PCIDR = crate::Reg<pcidr::PCIDR_SPEC>;
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pcidr;
#[doc = "PCIMR register accessor: an alias for `Reg<PCIMR_SPEC>`"]
pub type PCIMR = crate::Reg<pcimr::PCIMR_SPEC>;
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pcimr;
#[doc = "PCISR register accessor: an alias for `Reg<PCISR_SPEC>`"]
pub type PCISR = crate::Reg<pcisr::PCISR_SPEC>;
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pcisr;
#[doc = "PCRHR register accessor: an alias for `Reg<PCRHR_SPEC>`"]
pub type PCRHR = crate::Reg<pcrhr::PCRHR_SPEC>;
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pcrhr;
