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
#[doc = "Block Control Register"]
pub struct TC_BCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Block Control Register"]
pub mod tc_bcr;
#[doc = "Block Mode Register"]
pub struct TC_BMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Block Mode Register"]
pub mod tc_bmr;
#[doc = "QDEC Interrupt Enable Register"]
pub struct TC_QIER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Enable Register"]
pub mod tc_qier;
#[doc = "QDEC Interrupt Disable Register"]
pub struct TC_QIDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Disable Register"]
pub mod tc_qidr;
#[doc = "QDEC Interrupt Mask Register"]
pub struct TC_QIMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Mask Register"]
pub mod tc_qimr;
#[doc = "QDEC Interrupt Status Register"]
pub struct TC_QISR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Status Register"]
pub mod tc_qisr;
#[doc = "Fault Mode Register"]
pub struct TC_FMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Fault Mode Register"]
pub mod tc_fmr;
#[doc = "Write Protection Mode Register"]
pub struct TC_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod tc_wpmr;
