#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x34 - Channel Control Register (channel = 0)"]
    pub tc_channel0: TC_CHANNEL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x40..0x74 - Channel Control Register (channel = 0)"]
    pub tc_channel1: TC_CHANNEL,
    _reserved2: [u8; 0x0c],
    #[doc = "0x80..0xb4 - Channel Control Register (channel = 0)"]
    pub tc_channel2: TC_CHANNEL,
    _reserved3: [u8; 0x0c],
    #[doc = "0xc0 - Block Control Register"]
    pub bcr: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0xc4 - Block Mode Register"]
    pub bmr: crate::Reg<bmr::BMR_SPEC>,
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    pub qier: crate::Reg<qier::QIER_SPEC>,
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    pub qidr: crate::Reg<qidr::QIDR_SPEC>,
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    pub qimr: crate::Reg<qimr::QIMR_SPEC>,
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    pub qisr: crate::Reg<qisr::QISR_SPEC>,
    #[doc = "0xd8 - Fault Mode Register"]
    pub fmr: crate::Reg<fmr::FMR_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TC_CHANNEL {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr: crate::Reg<self::tc_channel::ccr::CCR_SPEC>,
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    pub cmr: crate::Reg<self::tc_channel::cmr::CMR_SPEC>,
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub smmr: crate::Reg<self::tc_channel::smmr::SMMR_SPEC>,
    #[doc = "0x0c - Register AB (channel = 0)"]
    pub rab: crate::Reg<self::tc_channel::rab::RAB_SPEC>,
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub cv: crate::Reg<self::tc_channel::cv::CV_SPEC>,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub ra: crate::Reg<self::tc_channel::ra::RA_SPEC>,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub rb: crate::Reg<self::tc_channel::rb::RB_SPEC>,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub rc: crate::Reg<self::tc_channel::rc::RC_SPEC>,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub sr: crate::Reg<self::tc_channel::sr::SR_SPEC>,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub ier: crate::Reg<self::tc_channel::ier::IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub idr: crate::Reg<self::tc_channel::idr::IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub imr: crate::Reg<self::tc_channel::imr::IMR_SPEC>,
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    pub emr: crate::Reg<self::tc_channel::emr::EMR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel Control Register (channel = 0)"]
pub mod tc_channel;
#[doc = "BCR register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "BMR register accessor: an alias for `Reg<BMR_SPEC>`"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QIER register accessor: an alias for `Reg<QIER_SPEC>`"]
pub type QIER = crate::Reg<qier::QIER_SPEC>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QIDR register accessor: an alias for `Reg<QIDR_SPEC>`"]
pub type QIDR = crate::Reg<qidr::QIDR_SPEC>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QIMR register accessor: an alias for `Reg<QIMR_SPEC>`"]
pub type QIMR = crate::Reg<qimr::QIMR_SPEC>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QISR register accessor: an alias for `Reg<QISR_SPEC>`"]
pub type QISR = crate::Reg<qisr::QISR_SPEC>;
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "FMR register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "Fault Mode Register"]
pub mod fmr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
