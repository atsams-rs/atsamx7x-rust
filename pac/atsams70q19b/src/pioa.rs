#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    pub pio_per: PIO_PER,
    #[doc = "0x04 - PIO Disable Register"]
    pub pio_pdr: PIO_PDR,
    #[doc = "0x08 - PIO Status Register"]
    pub pio_psr: PIO_PSR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Output Enable Register"]
    pub pio_oer: PIO_OER,
    #[doc = "0x14 - Output Disable Register"]
    pub pio_odr: PIO_ODR,
    #[doc = "0x18 - Output Status Register"]
    pub pio_osr: PIO_OSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    pub pio_ifer: PIO_IFER,
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    pub pio_ifdr: PIO_IFDR,
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    pub pio_ifsr: PIO_IFSR,
    _reserved2: [u8; 4usize],
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
    _reserved3: [u8; 4usize],
    #[doc = "0x60 - Pull-up Disable Register"]
    pub pio_pudr: PIO_PUDR,
    #[doc = "0x64 - Pull-up Enable Register"]
    pub pio_puer: PIO_PUER,
    #[doc = "0x68 - Pad Pull-up Status Register"]
    pub pio_pusr: PIO_PUSR,
    _reserved4: [u8; 4usize],
    #[doc = "0x70 - Peripheral ABCD Select Register 0"]
    pub pio_abcdsr: [PIO_ABCDSR; 2],
    _reserved5: [u8; 8usize],
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
    _reserved6: [u8; 4usize],
    #[doc = "0xa0 - Output Write Enable"]
    pub pio_ower: PIO_OWER,
    #[doc = "0xa4 - Output Write Disable"]
    pub pio_owdr: PIO_OWDR,
    #[doc = "0xa8 - Output Write Status Register"]
    pub pio_owsr: PIO_OWSR,
    _reserved7: [u8; 4usize],
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    pub pio_aimer: PIO_AIMER,
    #[doc = "0xb4 - Additional Interrupt Modes Disable Register"]
    pub pio_aimdr: PIO_AIMDR,
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    pub pio_aimmr: PIO_AIMMR,
    _reserved8: [u8; 4usize],
    #[doc = "0xc0 - Edge Select Register"]
    pub pio_esr: PIO_ESR,
    #[doc = "0xc4 - Level Select Register"]
    pub pio_lsr: PIO_LSR,
    #[doc = "0xc8 - Edge/Level Status Register"]
    pub pio_elsr: PIO_ELSR,
    _reserved9: [u8; 4usize],
    #[doc = "0xd0 - Falling Edge/Low-Level Select Register"]
    pub pio_fellsr: PIO_FELLSR,
    #[doc = "0xd4 - Rising Edge/High-Level Select Register"]
    pub pio_rehlsr: PIO_REHLSR,
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    pub pio_frlhsr: PIO_FRLHSR,
    _reserved10: [u8; 4usize],
    #[doc = "0xe0 - Lock Status"]
    pub pio_locksr: PIO_LOCKSR,
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub pio_wpmr: PIO_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub pio_wpsr: PIO_WPSR,
    _reserved11: [u8; 20usize],
    #[doc = "0x100 - Schmitt Trigger Register"]
    pub pio_schmitt: PIO_SCHMITT,
    _reserved12: [u8; 20usize],
    #[doc = "0x118 - I/O Drive Register"]
    pub pio_driver: PIO_DRIVER,
    _reserved13: [u8; 52usize],
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
#[doc = "PIO Enable Register"]
pub struct PIO_PER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Enable Register"]
pub mod pio_per;
#[doc = "PIO Disable Register"]
pub struct PIO_PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Disable Register"]
pub mod pio_pdr;
#[doc = "PIO Status Register"]
pub struct PIO_PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Status Register"]
pub mod pio_psr;
#[doc = "Output Enable Register"]
pub struct PIO_OER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Enable Register"]
pub mod pio_oer;
#[doc = "Output Disable Register"]
pub struct PIO_ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Disable Register"]
pub mod pio_odr;
#[doc = "Output Status Register"]
pub struct PIO_OSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Status Register"]
pub mod pio_osr;
#[doc = "Glitch Input Filter Enable Register"]
pub struct PIO_IFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Input Filter Enable Register"]
pub mod pio_ifer;
#[doc = "Glitch Input Filter Disable Register"]
pub struct PIO_IFDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Input Filter Disable Register"]
pub mod pio_ifdr;
#[doc = "Glitch Input Filter Status Register"]
pub struct PIO_IFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Input Filter Status Register"]
pub mod pio_ifsr;
#[doc = "Set Output Data Register"]
pub struct PIO_SODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Output Data Register"]
pub mod pio_sodr;
#[doc = "Clear Output Data Register"]
pub struct PIO_CODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Output Data Register"]
pub mod pio_codr;
#[doc = "Output Data Status Register"]
pub struct PIO_ODSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Data Status Register"]
pub mod pio_odsr;
#[doc = "Pin Data Status Register"]
pub struct PIO_PDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Data Status Register"]
pub mod pio_pdsr;
#[doc = "Interrupt Enable Register"]
pub struct PIO_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod pio_ier;
#[doc = "Interrupt Disable Register"]
pub struct PIO_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod pio_idr;
#[doc = "Interrupt Mask Register"]
pub struct PIO_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod pio_imr;
#[doc = "Interrupt Status Register"]
pub struct PIO_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod pio_isr;
#[doc = "Multi-driver Enable Register"]
pub struct PIO_MDER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-driver Enable Register"]
pub mod pio_mder;
#[doc = "Multi-driver Disable Register"]
pub struct PIO_MDDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-driver Disable Register"]
pub mod pio_mddr;
#[doc = "Multi-driver Status Register"]
pub struct PIO_MDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-driver Status Register"]
pub mod pio_mdsr;
#[doc = "Pull-up Disable Register"]
pub struct PIO_PUDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Disable Register"]
pub mod pio_pudr;
#[doc = "Pull-up Enable Register"]
pub struct PIO_PUER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Enable Register"]
pub mod pio_puer;
#[doc = "Pad Pull-up Status Register"]
pub struct PIO_PUSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-up Status Register"]
pub mod pio_pusr;
#[doc = "Peripheral ABCD Select Register 0"]
pub struct PIO_ABCDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral ABCD Select Register 0"]
pub mod pio_abcdsr;
#[doc = "Input Filter Slow Clock Disable Register"]
pub struct PIO_IFSCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod pio_ifscdr;
#[doc = "Input Filter Slow Clock Enable Register"]
pub struct PIO_IFSCER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod pio_ifscer;
#[doc = "Input Filter Slow Clock Status Register"]
pub struct PIO_IFSCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Filter Slow Clock Status Register"]
pub mod pio_ifscsr;
#[doc = "Slow Clock Divider Debouncing Register"]
pub struct PIO_SCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod pio_scdr;
#[doc = "Pad Pull-down Disable Register"]
pub struct PIO_PPDDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-down Disable Register"]
pub mod pio_ppddr;
#[doc = "Pad Pull-down Enable Register"]
pub struct PIO_PPDER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-down Enable Register"]
pub mod pio_ppder;
#[doc = "Pad Pull-down Status Register"]
pub struct PIO_PPDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-down Status Register"]
pub mod pio_ppdsr;
#[doc = "Output Write Enable"]
pub struct PIO_OWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Write Enable"]
pub mod pio_ower;
#[doc = "Output Write Disable"]
pub struct PIO_OWDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Write Disable"]
pub mod pio_owdr;
#[doc = "Output Write Status Register"]
pub struct PIO_OWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Write Status Register"]
pub mod pio_owsr;
#[doc = "Additional Interrupt Modes Enable Register"]
pub struct PIO_AIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod pio_aimer;
#[doc = "Additional Interrupt Modes Disable Register"]
pub struct PIO_AIMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional Interrupt Modes Disable Register"]
pub mod pio_aimdr;
#[doc = "Additional Interrupt Modes Mask Register"]
pub struct PIO_AIMMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod pio_aimmr;
#[doc = "Edge Select Register"]
pub struct PIO_ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Edge Select Register"]
pub mod pio_esr;
#[doc = "Level Select Register"]
pub struct PIO_LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Level Select Register"]
pub mod pio_lsr;
#[doc = "Edge/Level Status Register"]
pub struct PIO_ELSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Edge/Level Status Register"]
pub mod pio_elsr;
#[doc = "Falling Edge/Low-Level Select Register"]
pub struct PIO_FELLSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Falling Edge/Low-Level Select Register"]
pub mod pio_fellsr;
#[doc = "Rising Edge/High-Level Select Register"]
pub struct PIO_REHLSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rising Edge/High-Level Select Register"]
pub mod pio_rehlsr;
#[doc = "Fall/Rise - Low/High Status Register"]
pub struct PIO_FRLHSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod pio_frlhsr;
#[doc = "Lock Status"]
pub struct PIO_LOCKSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Status"]
pub mod pio_locksr;
#[doc = "Write Protection Mode Register"]
pub struct PIO_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod pio_wpmr;
#[doc = "Write Protection Status Register"]
pub struct PIO_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod pio_wpsr;
#[doc = "Schmitt Trigger Register"]
pub struct PIO_SCHMITT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Schmitt Trigger Register"]
pub mod pio_schmitt;
#[doc = "I/O Drive Register"]
pub struct PIO_DRIVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Drive Register"]
pub mod pio_driver;
#[doc = "Parallel Capture Mode Register"]
pub struct PIO_PCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Mode Register"]
pub mod pio_pcmr;
#[doc = "Parallel Capture Interrupt Enable Register"]
pub struct PIO_PCIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pio_pcier;
#[doc = "Parallel Capture Interrupt Disable Register"]
pub struct PIO_PCIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pio_pcidr;
#[doc = "Parallel Capture Interrupt Mask Register"]
pub struct PIO_PCIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pio_pcimr;
#[doc = "Parallel Capture Interrupt Status Register"]
pub struct PIO_PCISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pio_pcisr;
#[doc = "Parallel Capture Reception Holding Register"]
pub struct PIO_PCRHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pio_pcrhr;
