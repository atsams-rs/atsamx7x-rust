#[doc = r" Register block"]
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
    _reserved0: [u8; 12usize],
    #[doc = "0x7c - PWM Event Line 0 Mode Register 0"]
    pub pwm_elmr: [PWM_ELMR; 2],
    _reserved1: [u8; 28usize],
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    pub pwm_sspr: PWM_SSPR,
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    pub pwm_sspup: PWM_SSPUP,
    _reserved2: [u8; 8usize],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub pwm_smmr: PWM_SMMR,
    _reserved3: [u8; 12usize],
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    pub pwm_fpv2: PWM_FPV2,
    _reserved4: [u8; 32usize],
    #[doc = "0xe4 - PWM Write Protection Control Register"]
    pub pwm_wpcr: PWM_WPCR,
    #[doc = "0xe8 - PWM Write Protection Status Register"]
    pub pwm_wpsr: PWM_WPSR,
    _reserved5: [u8; 68usize],
    #[doc = "0x130 - PWM Comparison 0 Value Register"]
    pub pwm_cmp: [PWM_CMP; 8],
    _reserved6: [u8; 80usize],
    #[doc = "0x200 - PWM Channel Mode Register"]
    pub pwm_ch_num: [PWM_CH_NUM; 4],
    _reserved7: [u8; 384usize],
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    pub pwm_cmupd0: PWM_CMUPD0,
    _reserved8: [u8; 28usize],
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    pub pwm_cmupd1: PWM_CMUPD1,
    _reserved9: [u8; 8usize],
    #[doc = "0x42c - PWM External Trigger Register (trg_num = 1)"]
    pub pwm_etrg1: PWM_ETRG1,
    #[doc = "0x430 - PWM Leading-Edge Blanking Register (trg_num = 1)"]
    pub pwm_lebr1: PWM_LEBR1,
    _reserved10: [u8; 12usize],
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    pub pwm_cmupd2: PWM_CMUPD2,
    _reserved11: [u8; 8usize],
    #[doc = "0x44c - PWM External Trigger Register (trg_num = 2)"]
    pub pwm_etrg2: PWM_ETRG2,
    #[doc = "0x450 - PWM Leading-Edge Blanking Register (trg_num = 2)"]
    pub pwm_lebr2: PWM_LEBR2,
    _reserved12: [u8; 12usize],
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    pub pwm_cmupd3: PWM_CMUPD3,
}
#[doc = r" Register block"]
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
#[doc = r" Register block"]
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmp;
#[doc = r" Register block"]
#[repr(C)]
pub struct PWM_CH_NUM {
    #[doc = "0x00 - PWM Channel Mode Register"]
    pub pwm_cmr: self::pwm_ch_num::PWM_CMR,
    #[doc = "0x04 - PWM Channel Duty Cycle Register"]
    pub pwm_cdty: self::pwm_ch_num::PWM_CDTY,
    #[doc = "0x08 - PWM Channel Duty Cycle Update Register"]
    pub pwm_cdtyupd: self::pwm_ch_num::PWM_CDTYUPD,
    #[doc = "0x0c - PWM Channel Period Register"]
    pub pwm_cprd: self::pwm_ch_num::PWM_CPRD,
    #[doc = "0x10 - PWM Channel Period Update Register"]
    pub pwm_cprdupd: self::pwm_ch_num::PWM_CPRDUPD,
    #[doc = "0x14 - PWM Channel Counter Register"]
    pub pwm_ccnt: self::pwm_ch_num::PWM_CCNT,
    #[doc = "0x18 - PWM Channel Dead Time Register"]
    pub pwm_dt: self::pwm_ch_num::PWM_DT,
    #[doc = "0x1c - PWM Channel Dead Time Update Register"]
    pub pwm_dtupd: self::pwm_ch_num::PWM_DTUPD,
}
#[doc = r" Register block"]
#[doc = "PWM Channel Mode Register"]
pub mod pwm_ch_num;
#[doc = "PWM Clock Register"]
pub struct PWM_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Clock Register"]
pub mod pwm_clk;
#[doc = "PWM Enable Register"]
pub struct PWM_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Enable Register"]
pub mod pwm_ena;
#[doc = "PWM Disable Register"]
pub struct PWM_DIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Disable Register"]
pub mod pwm_dis;
#[doc = "PWM Status Register"]
pub struct PWM_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Status Register"]
pub mod pwm_sr;
#[doc = "PWM Interrupt Enable Register 1"]
pub struct PWM_IER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Enable Register 1"]
pub mod pwm_ier1;
#[doc = "PWM Interrupt Disable Register 1"]
pub struct PWM_IDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Disable Register 1"]
pub mod pwm_idr1;
#[doc = "PWM Interrupt Mask Register 1"]
pub struct PWM_IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Mask Register 1"]
pub mod pwm_imr1;
#[doc = "PWM Interrupt Status Register 1"]
pub struct PWM_ISR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Status Register 1"]
pub mod pwm_isr1;
#[doc = "PWM Sync Channels Mode Register"]
pub struct PWM_SCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Mode Register"]
pub mod pwm_scm;
#[doc = "PWM DMA Register"]
pub struct PWM_DMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM DMA Register"]
pub mod pwm_dmar;
#[doc = "PWM Sync Channels Update Control Register"]
pub struct PWM_SCUC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Update Control Register"]
pub mod pwm_scuc;
#[doc = "PWM Sync Channels Update Period Register"]
pub struct PWM_SCUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Update Period Register"]
pub mod pwm_scup;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub struct PWM_SCUPUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod pwm_scupupd;
#[doc = "PWM Interrupt Enable Register 2"]
pub struct PWM_IER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Enable Register 2"]
pub mod pwm_ier2;
#[doc = "PWM Interrupt Disable Register 2"]
pub struct PWM_IDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Disable Register 2"]
pub mod pwm_idr2;
#[doc = "PWM Interrupt Mask Register 2"]
pub struct PWM_IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Mask Register 2"]
pub mod pwm_imr2;
#[doc = "PWM Interrupt Status Register 2"]
pub struct PWM_ISR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Status Register 2"]
pub mod pwm_isr2;
#[doc = "PWM Output Override Value Register"]
pub struct PWM_OOV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Override Value Register"]
pub mod pwm_oov;
#[doc = "PWM Output Selection Register"]
pub struct PWM_OS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Register"]
pub mod pwm_os;
#[doc = "PWM Output Selection Set Register"]
pub struct PWM_OSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Set Register"]
pub mod pwm_oss;
#[doc = "PWM Output Selection Clear Register"]
pub struct PWM_OSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Clear Register"]
pub mod pwm_osc;
#[doc = "PWM Output Selection Set Update Register"]
pub struct PWM_OSSUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Set Update Register"]
pub mod pwm_ossupd;
#[doc = "PWM Output Selection Clear Update Register"]
pub struct PWM_OSCUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Clear Update Register"]
pub mod pwm_oscupd;
#[doc = "PWM Fault Mode Register"]
pub struct PWM_FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Mode Register"]
pub mod pwm_fmr;
#[doc = "PWM Fault Status Register"]
pub struct PWM_FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Status Register"]
pub mod pwm_fsr;
#[doc = "PWM Fault Clear Register"]
pub struct PWM_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Clear Register"]
pub mod pwm_fcr;
#[doc = "PWM Fault Protection Value Register 1"]
pub struct PWM_FPV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Protection Value Register 1"]
pub mod pwm_fpv1;
#[doc = "PWM Fault Protection Enable Register"]
pub struct PWM_FPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Protection Enable Register"]
pub mod pwm_fpe;
#[doc = "PWM Event Line 0 Mode Register 0"]
pub struct PWM_ELMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod pwm_elmr;
#[doc = "PWM Spread Spectrum Register"]
pub struct PWM_SSPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Spread Spectrum Register"]
pub mod pwm_sspr;
#[doc = "PWM Spread Spectrum Update Register"]
pub struct PWM_SSPUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Spread Spectrum Update Register"]
pub mod pwm_sspup;
#[doc = "PWM Stepper Motor Mode Register"]
pub struct PWM_SMMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Stepper Motor Mode Register"]
pub mod pwm_smmr;
#[doc = "PWM Fault Protection Value 2 Register"]
pub struct PWM_FPV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Protection Value 2 Register"]
pub mod pwm_fpv2;
#[doc = "PWM Write Protection Control Register"]
pub struct PWM_WPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Write Protection Control Register"]
pub mod pwm_wpcr;
#[doc = "PWM Write Protection Status Register"]
pub struct PWM_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Write Protection Status Register"]
pub mod pwm_wpsr;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub struct PWM_CMUPD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod pwm_cmupd0;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub struct PWM_CMUPD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod pwm_cmupd1;
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub struct PWM_ETRG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub mod pwm_etrg1;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub struct PWM_LEBR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub mod pwm_lebr1;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub struct PWM_CMUPD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod pwm_cmupd2;
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub struct PWM_ETRG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub mod pwm_etrg2;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub struct PWM_LEBR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub mod pwm_lebr2;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub struct PWM_CMUPD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod pwm_cmupd3;
