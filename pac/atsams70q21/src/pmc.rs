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
#[doc = "System Clock Enable Register"]
pub struct PMC_SCER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System Clock Enable Register"]
pub mod pmc_scer;
#[doc = "System Clock Disable Register"]
pub struct PMC_SCDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System Clock Disable Register"]
pub mod pmc_scdr;
#[doc = "System Clock Status Register"]
pub struct PMC_SCSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System Clock Status Register"]
pub mod pmc_scsr;
#[doc = "Peripheral Clock Enable Register 0"]
pub struct PMC_PCER0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pmc_pcer0;
#[doc = "Peripheral Clock Disable Register 0"]
pub struct PMC_PCDR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pmc_pcdr0;
#[doc = "Peripheral Clock Status Register 0"]
pub struct PMC_PCSR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Status Register 0"]
pub mod pmc_pcsr0;
#[doc = "UTMI Clock Register"]
pub struct CKGR_UCKR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UTMI Clock Register"]
pub mod ckgr_uckr;
#[doc = "Main Oscillator Register"]
pub struct CKGR_MOR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "Main Clock Frequency Register"]
pub struct CKGR_MCFR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "PLLA Register"]
pub struct CKGR_PLLAR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "Master Clock Register"]
pub struct PMC_MCKR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Master Clock Register"]
pub mod pmc_mckr;
#[doc = "USB Clock Register"]
pub struct PMC_USB {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Register"]
pub mod pmc_usb;
#[doc = "Programmable Clock Register"]
pub struct PMC_PCK {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Programmable Clock Register"]
pub mod pmc_pck;
#[doc = "Interrupt Enable Register"]
pub struct PMC_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod pmc_ier;
#[doc = "Interrupt Disable Register"]
pub struct PMC_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod pmc_idr;
#[doc = "Status Register"]
pub struct PMC_SR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod pmc_sr;
#[doc = "Interrupt Mask Register"]
pub struct PMC_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod pmc_imr;
#[doc = "Fast Startup Mode Register"]
pub struct PMC_FSMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Fast Startup Mode Register"]
pub mod pmc_fsmr;
#[doc = "Fast Startup Polarity Register"]
pub struct PMC_FSPR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Fast Startup Polarity Register"]
pub mod pmc_fspr;
#[doc = "Fault Output Clear Register"]
pub struct PMC_FOCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Fault Output Clear Register"]
pub mod pmc_focr;
#[doc = "Write Protection Mode Register"]
pub struct PMC_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod pmc_wpmr;
#[doc = "Write Protection Status Register"]
pub struct PMC_WPSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod pmc_wpsr;
#[doc = "Peripheral Clock Enable Register 1"]
pub struct PMC_PCER1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Enable Register 1"]
pub mod pmc_pcer1;
#[doc = "Peripheral Clock Disable Register 1"]
pub struct PMC_PCDR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Disable Register 1"]
pub mod pmc_pcdr1;
#[doc = "Peripheral Clock Status Register 1"]
pub struct PMC_PCSR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Status Register 1"]
pub mod pmc_pcsr1;
#[doc = "Peripheral Control Register"]
pub struct PMC_PCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Control Register"]
pub mod pmc_pcr;
#[doc = "Oscillator Calibration Register"]
pub struct PMC_OCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Calibration Register"]
pub mod pmc_ocr;
#[doc = "SleepWalking Enable Register 0"]
pub struct PMC_SLPWK_ER0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Enable Register 0"]
pub mod pmc_slpwk_er0;
#[doc = "SleepWalking Disable Register 0"]
pub struct PMC_SLPWK_DR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Disable Register 0"]
pub mod pmc_slpwk_dr0;
#[doc = "SleepWalking Status Register 0"]
pub struct PMC_SLPWK_SR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Status Register 0"]
pub mod pmc_slpwk_sr0;
#[doc = "SleepWalking Activity Status Register 0"]
pub struct PMC_SLPWK_ASR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Activity Status Register 0"]
pub mod pmc_slpwk_asr0;
#[doc = "PLL Maximum Multiplier Value Register"]
pub struct PMC_PMMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmc_pmmr;
#[doc = "SleepWalking Enable Register 1"]
pub struct PMC_SLPWK_ER1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Enable Register 1"]
pub mod pmc_slpwk_er1;
#[doc = "SleepWalking Disable Register 1"]
pub struct PMC_SLPWK_DR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Disable Register 1"]
pub mod pmc_slpwk_dr1;
#[doc = "SleepWalking Status Register 1"]
pub struct PMC_SLPWK_SR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Status Register 1"]
pub mod pmc_slpwk_sr1;
#[doc = "SleepWalking Activity Status Register 1"]
pub struct PMC_SLPWK_ASR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Activity Status Register 1"]
pub mod pmc_slpwk_asr1;
#[doc = "SleepWalking Activity In Progress Register"]
pub struct PMC_SLPWK_AIPR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Activity In Progress Register"]
pub mod pmc_slpwk_aipr;
