#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    pub clk: crate::Reg<clk::CLK_SPEC>,
    #[doc = "0x04 - PWM Enable Register"]
    pub ena: crate::Reg<ena::ENA_SPEC>,
    #[doc = "0x08 - PWM Disable Register"]
    pub dis: crate::Reg<dis::DIS_SPEC>,
    #[doc = "0x0c - PWM Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    pub ier1: crate::Reg<ier1::IER1_SPEC>,
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    pub idr1: crate::Reg<idr1::IDR1_SPEC>,
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    pub isr1: crate::Reg<isr1::ISR1_SPEC>,
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    pub scm: crate::Reg<scm::SCM_SPEC>,
    #[doc = "0x24 - PWM DMA Register"]
    pub dmar: crate::Reg<dmar::DMAR_SPEC>,
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    pub scuc: crate::Reg<scuc::SCUC_SPEC>,
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    pub scup: crate::Reg<scup::SCUP_SPEC>,
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    pub scupupd: crate::Reg<scupupd::SCUPUPD_SPEC>,
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    pub ier2: crate::Reg<ier2::IER2_SPEC>,
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    pub idr2: crate::Reg<idr2::IDR2_SPEC>,
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    pub isr2: crate::Reg<isr2::ISR2_SPEC>,
    #[doc = "0x44 - PWM Output Override Value Register"]
    pub oov: crate::Reg<oov::OOV_SPEC>,
    #[doc = "0x48 - PWM Output Selection Register"]
    pub os: crate::Reg<os::OS_SPEC>,
    #[doc = "0x4c - PWM Output Selection Set Register"]
    pub oss: crate::Reg<oss::OSS_SPEC>,
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    pub osc: crate::Reg<osc::OSC_SPEC>,
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    pub ossupd: crate::Reg<ossupd::OSSUPD_SPEC>,
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    pub oscupd: crate::Reg<oscupd::OSCUPD_SPEC>,
    #[doc = "0x5c - PWM Fault Mode Register"]
    pub fmr: crate::Reg<fmr::FMR_SPEC>,
    #[doc = "0x60 - PWM Fault Status Register"]
    pub fsr: crate::Reg<fsr::FSR_SPEC>,
    #[doc = "0x64 - PWM Fault Clear Register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    #[doc = "0x68 - PWM Fault Protection Value Register 1"]
    pub fpv1: crate::Reg<fpv1::FPV1_SPEC>,
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    pub fpe: crate::Reg<fpe::FPE_SPEC>,
    _reserved28: [u8; 0x0c],
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register 0"]
    pub elmr: [crate::Reg<elmr::ELMR_SPEC>; 2],
    _reserved29: [u8; 0x1c],
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    pub sspr: crate::Reg<sspr::SSPR_SPEC>,
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    pub sspup: crate::Reg<sspup::SSPUP_SPEC>,
    _reserved31: [u8; 0x08],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub smmr: crate::Reg<smmr::SMMR_SPEC>,
    _reserved32: [u8; 0x0c],
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    pub fpv2: crate::Reg<fpv2::FPV2_SPEC>,
    _reserved33: [u8; 0x20],
    #[doc = "0xe4 - PWM Write Protection Control Register"]
    pub wpcr: crate::Reg<wpcr::WPCR_SPEC>,
    #[doc = "0xe8 - PWM Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved35: [u8; 0x44],
    #[doc = "0x130..0x1b0 - PWM Comparison 0 Value Register"]
    pub pwm_cmp: [PWM_CMP; 8],
    _reserved36: [u8; 0x50],
    #[doc = "0x200..0x280 - PWM Channel Mode Register (ch_num = 0)"]
    pub pwm_ch_num: [PWM_CH_NUM; 4],
    _reserved37: [u8; 0x0180],
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    pub cmupd0: crate::Reg<cmupd0::CMUPD0_SPEC>,
    _reserved38: [u8; 0x1c],
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    pub cmupd1: crate::Reg<cmupd1::CMUPD1_SPEC>,
    _reserved39: [u8; 0x08],
    #[doc = "0x42c - PWM External Trigger Register (trg_num = 1)"]
    pub etrg1: crate::Reg<etrg1::ETRG1_SPEC>,
    #[doc = "0x430 - PWM Leading-Edge Blanking Register (trg_num = 1)"]
    pub lebr1: crate::Reg<lebr1::LEBR1_SPEC>,
    _reserved41: [u8; 0x0c],
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    pub cmupd2: crate::Reg<cmupd2::CMUPD2_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0x44c - PWM External Trigger Register (trg_num = 2)"]
    pub etrg2: crate::Reg<etrg2::ETRG2_SPEC>,
    #[doc = "0x450 - PWM Leading-Edge Blanking Register (trg_num = 2)"]
    pub lebr2: crate::Reg<lebr2::LEBR2_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    pub cmupd3: crate::Reg<cmupd3::CMUPD3_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CMP {
    #[doc = "0x00 - PWM Comparison 0 Value Register"]
    pub cmpv: crate::Reg<self::pwm_cmp::cmpv::CMPV_SPEC>,
    #[doc = "0x04 - PWM Comparison 0 Value Update Register"]
    pub cmpvupd: crate::Reg<self::pwm_cmp::cmpvupd::CMPVUPD_SPEC>,
    #[doc = "0x08 - PWM Comparison 0 Mode Register"]
    pub cmpm: crate::Reg<self::pwm_cmp::cmpm::CMPM_SPEC>,
    #[doc = "0x0c - PWM Comparison 0 Mode Update Register"]
    pub cmpmupd: crate::Reg<self::pwm_cmp::cmpmupd::CMPMUPD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmp;
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CH_NUM {
    #[doc = "0x00 - PWM Channel Mode Register (ch_num = 0)"]
    pub cmr: crate::Reg<self::pwm_ch_num::cmr::CMR_SPEC>,
    #[doc = "0x04 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub cdty: crate::Reg<self::pwm_ch_num::cdty::CDTY_SPEC>,
    #[doc = "0x08 - PWM Channel Duty Cycle Update Register (ch_num = 0)"]
    pub cdtyupd: crate::Reg<self::pwm_ch_num::cdtyupd::CDTYUPD_SPEC>,
    #[doc = "0x0c - PWM Channel Period Register (ch_num = 0)"]
    pub cprd: crate::Reg<self::pwm_ch_num::cprd::CPRD_SPEC>,
    #[doc = "0x10 - PWM Channel Period Update Register (ch_num = 0)"]
    pub cprdupd: crate::Reg<self::pwm_ch_num::cprdupd::CPRDUPD_SPEC>,
    #[doc = "0x14 - PWM Channel Counter Register (ch_num = 0)"]
    pub ccnt: crate::Reg<self::pwm_ch_num::ccnt::CCNT_SPEC>,
    #[doc = "0x18 - PWM Channel Dead Time Register (ch_num = 0)"]
    pub dt: crate::Reg<self::pwm_ch_num::dt::DT_SPEC>,
    #[doc = "0x1c - PWM Channel Dead Time Update Register (ch_num = 0)"]
    pub dtupd: crate::Reg<self::pwm_ch_num::dtupd::DTUPD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod pwm_ch_num;
#[doc = "CLK register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "PWM Clock Register"]
pub mod clk;
#[doc = "ENA register accessor: an alias for `Reg<ENA_SPEC>`"]
pub type ENA = crate::Reg<ena::ENA_SPEC>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS register accessor: an alias for `Reg<DIS_SPEC>`"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER1 register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "IDR1 register accessor: an alias for `Reg<IDR1_SPEC>`"]
pub type IDR1 = crate::Reg<idr1::IDR1_SPEC>;
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "ISR1 register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "SCM register accessor: an alias for `Reg<SCM_SPEC>`"]
pub type SCM = crate::Reg<scm::SCM_SPEC>;
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "DMAR register accessor: an alias for `Reg<DMAR_SPEC>`"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "PWM DMA Register"]
pub mod dmar;
#[doc = "SCUC register accessor: an alias for `Reg<SCUC_SPEC>`"]
pub type SCUC = crate::Reg<scuc::SCUC_SPEC>;
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "SCUP register accessor: an alias for `Reg<SCUP_SPEC>`"]
pub type SCUP = crate::Reg<scup::SCUP_SPEC>;
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "SCUPUPD register accessor: an alias for `Reg<SCUPUPD_SPEC>`"]
pub type SCUPUPD = crate::Reg<scupupd::SCUPUPD_SPEC>;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "IER2 register accessor: an alias for `Reg<IER2_SPEC>`"]
pub type IER2 = crate::Reg<ier2::IER2_SPEC>;
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "IDR2 register accessor: an alias for `Reg<IDR2_SPEC>`"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "IMR2 register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "ISR2 register accessor: an alias for `Reg<ISR2_SPEC>`"]
pub type ISR2 = crate::Reg<isr2::ISR2_SPEC>;
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "OOV register accessor: an alias for `Reg<OOV_SPEC>`"]
pub type OOV = crate::Reg<oov::OOV_SPEC>;
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "OS register accessor: an alias for `Reg<OS_SPEC>`"]
pub type OS = crate::Reg<os::OS_SPEC>;
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "OSS register accessor: an alias for `Reg<OSS_SPEC>`"]
pub type OSS = crate::Reg<oss::OSS_SPEC>;
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "OSC register accessor: an alias for `Reg<OSC_SPEC>`"]
pub type OSC = crate::Reg<osc::OSC_SPEC>;
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "OSSUPD register accessor: an alias for `Reg<OSSUPD_SPEC>`"]
pub type OSSUPD = crate::Reg<ossupd::OSSUPD_SPEC>;
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "OSCUPD register accessor: an alias for `Reg<OSCUPD_SPEC>`"]
pub type OSCUPD = crate::Reg<oscupd::OSCUPD_SPEC>;
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "FMR register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "FSR register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "FPV1 register accessor: an alias for `Reg<FPV1_SPEC>`"]
pub type FPV1 = crate::Reg<fpv1::FPV1_SPEC>;
#[doc = "PWM Fault Protection Value Register 1"]
pub mod fpv1;
#[doc = "FPE register accessor: an alias for `Reg<FPE_SPEC>`"]
pub type FPE = crate::Reg<fpe::FPE_SPEC>;
#[doc = "PWM Fault Protection Enable Register"]
pub mod fpe;
#[doc = "ELMR register accessor: an alias for `Reg<ELMR_SPEC>`"]
pub type ELMR = crate::Reg<elmr::ELMR_SPEC>;
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod elmr;
#[doc = "SSPR register accessor: an alias for `Reg<SSPR_SPEC>`"]
pub type SSPR = crate::Reg<sspr::SSPR_SPEC>;
#[doc = "PWM Spread Spectrum Register"]
pub mod sspr;
#[doc = "SSPUP register accessor: an alias for `Reg<SSPUP_SPEC>`"]
pub type SSPUP = crate::Reg<sspup::SSPUP_SPEC>;
#[doc = "PWM Spread Spectrum Update Register"]
pub mod sspup;
#[doc = "SMMR register accessor: an alias for `Reg<SMMR_SPEC>`"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "FPV2 register accessor: an alias for `Reg<FPV2_SPEC>`"]
pub type FPV2 = crate::Reg<fpv2::FPV2_SPEC>;
#[doc = "PWM Fault Protection Value 2 Register"]
pub mod fpv2;
#[doc = "WPCR register accessor: an alias for `Reg<WPCR_SPEC>`"]
pub type WPCR = crate::Reg<wpcr::WPCR_SPEC>;
#[doc = "PWM Write Protection Control Register"]
pub mod wpcr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "PWM Write Protection Status Register"]
pub mod wpsr;
#[doc = "CMUPD0 register accessor: an alias for `Reg<CMUPD0_SPEC>`"]
pub type CMUPD0 = crate::Reg<cmupd0::CMUPD0_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod cmupd0;
#[doc = "CMUPD1 register accessor: an alias for `Reg<CMUPD1_SPEC>`"]
pub type CMUPD1 = crate::Reg<cmupd1::CMUPD1_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod cmupd1;
#[doc = "ETRG1 register accessor: an alias for `Reg<ETRG1_SPEC>`"]
pub type ETRG1 = crate::Reg<etrg1::ETRG1_SPEC>;
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub mod etrg1;
#[doc = "LEBR1 register accessor: an alias for `Reg<LEBR1_SPEC>`"]
pub type LEBR1 = crate::Reg<lebr1::LEBR1_SPEC>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub mod lebr1;
#[doc = "CMUPD2 register accessor: an alias for `Reg<CMUPD2_SPEC>`"]
pub type CMUPD2 = crate::Reg<cmupd2::CMUPD2_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod cmupd2;
#[doc = "ETRG2 register accessor: an alias for `Reg<ETRG2_SPEC>`"]
pub type ETRG2 = crate::Reg<etrg2::ETRG2_SPEC>;
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub mod etrg2;
#[doc = "LEBR2 register accessor: an alias for `Reg<LEBR2_SPEC>`"]
pub type LEBR2 = crate::Reg<lebr2::LEBR2_SPEC>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub mod lebr2;
#[doc = "CMUPD3 register accessor: an alias for `Reg<CMUPD3_SPEC>`"]
pub type CMUPD3 = crate::Reg<cmupd3::CMUPD3_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod cmupd3;
