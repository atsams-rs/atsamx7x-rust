#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x08 - Receive Data Register"]
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    #[doc = "0x0c - Transmit Data Register"]
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    #[doc = "0x10 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x20 - Serial Clock Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Instruction Address Register"]
    pub iar: crate::Reg<iar::IAR_SPEC>,
    #[doc = "0x34 - Instruction Code Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x38 - Instruction Frame Register"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Scrambling Mode Register"]
    pub smr: crate::Reg<smr::SMR_SPEC>,
    #[doc = "0x44 - Scrambling Key Register"]
    pub skr: crate::Reg<skr::SKR_SPEC>,
    _reserved14: [u8; 0x9c],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "RDR register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Serial Clock Register"]
pub mod scr;
#[doc = "IAR register accessor: an alias for `Reg<IAR_SPEC>`"]
pub type IAR = crate::Reg<iar::IAR_SPEC>;
#[doc = "Instruction Address Register"]
pub mod iar;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Instruction Code Register"]
pub mod icr;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "Instruction Frame Register"]
pub mod ifr;
#[doc = "SMR register accessor: an alias for `Reg<SMR_SPEC>`"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "Scrambling Mode Register"]
pub mod smr;
#[doc = "SKR register accessor: an alias for `Reg<SKR_SPEC>`"]
pub type SKR = crate::Reg<skr::SKR_SPEC>;
#[doc = "Scrambling Key Register"]
pub mod skr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
