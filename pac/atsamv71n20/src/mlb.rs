#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MediaLB Control 0 Register"]
    pub mlb_mlbc0: crate::Reg<mlb_mlbc0::MLB_MLBC0_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - MediaLB Channel Status 0 Register"]
    pub mlb_ms0: crate::Reg<mlb_ms0::MLB_MS0_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x14 - MediaLB Channel Status1 Register"]
    pub mlb_ms1: crate::Reg<mlb_ms1::MLB_MS1_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x20 - MediaLB System Status Register"]
    pub mlb_mss: crate::Reg<mlb_mss::MLB_MSS_SPEC>,
    #[doc = "0x24 - MediaLB System Data Register"]
    pub mlb_msd: crate::Reg<mlb_msd::MLB_MSD_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x2c - MediaLB Interrupt Enable Register"]
    pub mlb_mien: crate::Reg<mlb_mien::MLB_MIEN_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x3c - MediaLB Control 1 Register"]
    pub mlb_mlbc1: crate::Reg<mlb_mlbc1::MLB_MLBC1_SPEC>,
    _reserved7: [u8; 0x40],
    #[doc = "0x80 - HBI Control Register"]
    pub mlb_hctl: crate::Reg<mlb_hctl::MLB_HCTL_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x88..0x90 - HBI Channel Mask 0 Register 0"]
    pub mlb_hcmr: [crate::Reg<mlb_hcmr::MLB_HCMR_SPEC>; 2],
    #[doc = "0x90..0x98 - HBI Channel Error 0 Register 0"]
    pub mlb_hcer: [crate::Reg<mlb_hcer::MLB_HCER_SPEC>; 2],
    #[doc = "0x98..0xa0 - HBI Channel Busy 0 Register 0"]
    pub mlb_hcbr: [crate::Reg<mlb_hcbr::MLB_HCBR_SPEC>; 2],
    _reserved11: [u8; 0x20],
    #[doc = "0xc0..0xd0 - MIF Data 0 Register 0"]
    pub mlb_mdat: [crate::Reg<mlb_mdat::MLB_MDAT_SPEC>; 4],
    #[doc = "0xd0..0xe0 - MIF Data Write Enable 0 Register 0"]
    pub mlb_mdwe: [crate::Reg<mlb_mdwe::MLB_MDWE_SPEC>; 4],
    #[doc = "0xe0 - MIF Control Register"]
    pub mlb_mctl: crate::Reg<mlb_mctl::MLB_MCTL_SPEC>,
    #[doc = "0xe4 - MIF Address Register"]
    pub mlb_madr: crate::Reg<mlb_madr::MLB_MADR_SPEC>,
    _reserved15: [u8; 0x02d8],
    #[doc = "0x3c0 - AHB Control Register"]
    pub mlb_actl: crate::Reg<mlb_actl::MLB_ACTL_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x3d0..0x3d8 - AHB Channel Status 0 Register 0"]
    pub mlb_acsr: [crate::Reg<mlb_acsr::MLB_ACSR_SPEC>; 2],
    #[doc = "0x3d8..0x3e0 - AHB Channel Mask 0 Register 0"]
    pub mlb_acmr: [crate::Reg<mlb_acmr::MLB_ACMR_SPEC>; 2],
}
#[doc = "MLB_MLBC0 register accessor: an alias for `Reg<MLB_MLBC0_SPEC>`"]
pub type MLB_MLBC0 = crate::Reg<mlb_mlbc0::MLB_MLBC0_SPEC>;
#[doc = "MediaLB Control 0 Register"]
pub mod mlb_mlbc0;
#[doc = "MLB_MS0 register accessor: an alias for `Reg<MLB_MS0_SPEC>`"]
pub type MLB_MS0 = crate::Reg<mlb_ms0::MLB_MS0_SPEC>;
#[doc = "MediaLB Channel Status 0 Register"]
pub mod mlb_ms0;
#[doc = "MLB_MS1 register accessor: an alias for `Reg<MLB_MS1_SPEC>`"]
pub type MLB_MS1 = crate::Reg<mlb_ms1::MLB_MS1_SPEC>;
#[doc = "MediaLB Channel Status1 Register"]
pub mod mlb_ms1;
#[doc = "MLB_MSS register accessor: an alias for `Reg<MLB_MSS_SPEC>`"]
pub type MLB_MSS = crate::Reg<mlb_mss::MLB_MSS_SPEC>;
#[doc = "MediaLB System Status Register"]
pub mod mlb_mss;
#[doc = "MLB_MSD register accessor: an alias for `Reg<MLB_MSD_SPEC>`"]
pub type MLB_MSD = crate::Reg<mlb_msd::MLB_MSD_SPEC>;
#[doc = "MediaLB System Data Register"]
pub mod mlb_msd;
#[doc = "MLB_MIEN register accessor: an alias for `Reg<MLB_MIEN_SPEC>`"]
pub type MLB_MIEN = crate::Reg<mlb_mien::MLB_MIEN_SPEC>;
#[doc = "MediaLB Interrupt Enable Register"]
pub mod mlb_mien;
#[doc = "MLB_MLBC1 register accessor: an alias for `Reg<MLB_MLBC1_SPEC>`"]
pub type MLB_MLBC1 = crate::Reg<mlb_mlbc1::MLB_MLBC1_SPEC>;
#[doc = "MediaLB Control 1 Register"]
pub mod mlb_mlbc1;
#[doc = "MLB_HCTL register accessor: an alias for `Reg<MLB_HCTL_SPEC>`"]
pub type MLB_HCTL = crate::Reg<mlb_hctl::MLB_HCTL_SPEC>;
#[doc = "HBI Control Register"]
pub mod mlb_hctl;
#[doc = "MLB_HCMR register accessor: an alias for `Reg<MLB_HCMR_SPEC>`"]
pub type MLB_HCMR = crate::Reg<mlb_hcmr::MLB_HCMR_SPEC>;
#[doc = "HBI Channel Mask 0 Register 0"]
pub mod mlb_hcmr;
#[doc = "MLB_HCER register accessor: an alias for `Reg<MLB_HCER_SPEC>`"]
pub type MLB_HCER = crate::Reg<mlb_hcer::MLB_HCER_SPEC>;
#[doc = "HBI Channel Error 0 Register 0"]
pub mod mlb_hcer;
#[doc = "MLB_HCBR register accessor: an alias for `Reg<MLB_HCBR_SPEC>`"]
pub type MLB_HCBR = crate::Reg<mlb_hcbr::MLB_HCBR_SPEC>;
#[doc = "HBI Channel Busy 0 Register 0"]
pub mod mlb_hcbr;
#[doc = "MLB_MDAT register accessor: an alias for `Reg<MLB_MDAT_SPEC>`"]
pub type MLB_MDAT = crate::Reg<mlb_mdat::MLB_MDAT_SPEC>;
#[doc = "MIF Data 0 Register 0"]
pub mod mlb_mdat;
#[doc = "MLB_MDWE register accessor: an alias for `Reg<MLB_MDWE_SPEC>`"]
pub type MLB_MDWE = crate::Reg<mlb_mdwe::MLB_MDWE_SPEC>;
#[doc = "MIF Data Write Enable 0 Register 0"]
pub mod mlb_mdwe;
#[doc = "MLB_MCTL register accessor: an alias for `Reg<MLB_MCTL_SPEC>`"]
pub type MLB_MCTL = crate::Reg<mlb_mctl::MLB_MCTL_SPEC>;
#[doc = "MIF Control Register"]
pub mod mlb_mctl;
#[doc = "MLB_MADR register accessor: an alias for `Reg<MLB_MADR_SPEC>`"]
pub type MLB_MADR = crate::Reg<mlb_madr::MLB_MADR_SPEC>;
#[doc = "MIF Address Register"]
pub mod mlb_madr;
#[doc = "MLB_ACTL register accessor: an alias for `Reg<MLB_ACTL_SPEC>`"]
pub type MLB_ACTL = crate::Reg<mlb_actl::MLB_ACTL_SPEC>;
#[doc = "AHB Control Register"]
pub mod mlb_actl;
#[doc = "MLB_ACSR register accessor: an alias for `Reg<MLB_ACSR_SPEC>`"]
pub type MLB_ACSR = crate::Reg<mlb_acsr::MLB_ACSR_SPEC>;
#[doc = "AHB Channel Status 0 Register 0"]
pub mod mlb_acsr;
#[doc = "MLB_ACMR register accessor: an alias for `Reg<MLB_ACMR_SPEC>`"]
pub type MLB_ACMR = crate::Reg<mlb_acmr::MLB_ACMR_SPEC>;
#[doc = "AHB Channel Mask 0 Register 0"]
pub mod mlb_acmr;
