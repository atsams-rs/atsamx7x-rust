#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub gmac_ncr: GMAC_NCR,
    #[doc = "0x04 - Network Configuration Register"]
    pub gmac_ncfgr: GMAC_NCFGR,
    #[doc = "0x08 - Network Status Register"]
    pub gmac_nsr: GMAC_NSR,
    #[doc = "0x0c - User Register"]
    pub gmac_ur: GMAC_UR,
    #[doc = "0x10 - DMA Configuration Register"]
    pub gmac_dcfgr: GMAC_DCFGR,
    #[doc = "0x14 - Transmit Status Register"]
    pub gmac_tsr: GMAC_TSR,
    #[doc = "0x18 - Receive Buffer Queue Base Address Register"]
    pub gmac_rbqb: GMAC_RBQB,
    #[doc = "0x1c - Transmit Buffer Queue Base Address Register"]
    pub gmac_tbqb: GMAC_TBQB,
    #[doc = "0x20 - Receive Status Register"]
    pub gmac_rsr: GMAC_RSR,
    #[doc = "0x24 - Interrupt Status Register"]
    pub gmac_isr: GMAC_ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub gmac_ier: GMAC_IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub gmac_idr: GMAC_IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub gmac_imr: GMAC_IMR,
    #[doc = "0x34 - PHY Maintenance Register"]
    pub gmac_man: GMAC_MAN,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub gmac_rpq: GMAC_RPQ,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub gmac_tpq: GMAC_TPQ,
    #[doc = "0x40 - TX Partial Store and Forward Register"]
    pub gmac_tpsf: GMAC_TPSF,
    #[doc = "0x44 - RX Partial Store and Forward Register"]
    pub gmac_rpsf: GMAC_RPSF,
    #[doc = "0x48 - RX Jumbo Frame Max Length Register"]
    pub gmac_rjfml: GMAC_RJFML,
    _reserved19: [u8; 52usize],
    #[doc = "0x80 - Hash Register Bottom"]
    pub gmac_hrb: GMAC_HRB,
    #[doc = "0x84 - Hash Register Top"]
    pub gmac_hrt: GMAC_HRT,
    #[doc = "0x88 - Specific Address 1 Bottom Register"]
    pub gmac_sa1: GMAC_SA,
    #[doc = "0x90 - Specific Address 1 Bottom Register"]
    pub gmac_sa2: GMAC_SA,
    #[doc = "0x98 - Specific Address 1 Bottom Register"]
    pub gmac_sa3: GMAC_SA,
    #[doc = "0xa0 - Specific Address 1 Bottom Register"]
    pub gmac_sa4: GMAC_SA,
    #[doc = "0xa8 - Type ID Match 1 Register"]
    pub gmac_tidm1: GMAC_TIDM1,
    #[doc = "0xac - Type ID Match 2 Register"]
    pub gmac_tidm2: GMAC_TIDM2,
    #[doc = "0xb0 - Type ID Match 3 Register"]
    pub gmac_tidm3: GMAC_TIDM3,
    #[doc = "0xb4 - Type ID Match 4 Register"]
    pub gmac_tidm4: GMAC_TIDM4,
    #[doc = "0xb8 - Wake on LAN Register"]
    pub gmac_wol: GMAC_WOL,
    #[doc = "0xbc - IPG Stretch Register"]
    pub gmac_ipgs: GMAC_IPGS,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub gmac_svlan: GMAC_SVLAN,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub gmac_tpfcp: GMAC_TPFCP,
    #[doc = "0xc8 - Specific Address 1 Mask Bottom Register"]
    pub gmac_samb1: GMAC_SAMB1,
    #[doc = "0xcc - Specific Address 1 Mask Top Register"]
    pub gmac_samt1: GMAC_SAMT1,
    _reserved35: [u8; 12usize],
    #[doc = "0xdc - 1588 Timer Nanosecond Comparison Register"]
    pub gmac_nsc: GMAC_NSC,
    #[doc = "0xe0 - 1588 Timer Second Comparison Low Register"]
    pub gmac_scl: GMAC_SCL,
    #[doc = "0xe4 - 1588 Timer Second Comparison High Register"]
    pub gmac_sch: GMAC_SCH,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds High Register"]
    pub gmac_eftsh: GMAC_EFTSH,
    #[doc = "0xec - PTP Event Frame Received Seconds High Register"]
    pub gmac_efrsh: GMAC_EFRSH,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds High Register"]
    pub gmac_peftsh: GMAC_PEFTSH,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds High Register"]
    pub gmac_pefrsh: GMAC_PEFRSH,
    _reserved42: [u8; 8usize],
    #[doc = "0x100 - Octets Transmitted Low Register"]
    pub gmac_otlo: GMAC_OTLO,
    #[doc = "0x104 - Octets Transmitted High Register"]
    pub gmac_othi: GMAC_OTHI,
    #[doc = "0x108 - Frames Transmitted Register"]
    pub gmac_ft: GMAC_FT,
    #[doc = "0x10c - Broadcast Frames Transmitted Register"]
    pub gmac_bcft: GMAC_BCFT,
    #[doc = "0x110 - Multicast Frames Transmitted Register"]
    pub gmac_mft: GMAC_MFT,
    #[doc = "0x114 - Pause Frames Transmitted Register"]
    pub gmac_pft: GMAC_PFT,
    #[doc = "0x118 - 64 Byte Frames Transmitted Register"]
    pub gmac_bft64: GMAC_BFT64,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted Register"]
    pub gmac_tbft127: GMAC_TBFT127,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted Register"]
    pub gmac_tbft255: GMAC_TBFT255,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted Register"]
    pub gmac_tbft511: GMAC_TBFT511,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted Register"]
    pub gmac_tbft1023: GMAC_TBFT1023,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted Register"]
    pub gmac_tbft1518: GMAC_TBFT1518,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted Register"]
    pub gmac_gtbft1518: GMAC_GTBFT1518,
    #[doc = "0x134 - Transmit Underruns Register"]
    pub gmac_tur: GMAC_TUR,
    #[doc = "0x138 - Single Collision Frames Register"]
    pub gmac_scf: GMAC_SCF,
    #[doc = "0x13c - Multiple Collision Frames Register"]
    pub gmac_mcf: GMAC_MCF,
    #[doc = "0x140 - Excessive Collisions Register"]
    pub gmac_ec: GMAC_EC,
    #[doc = "0x144 - Late Collisions Register"]
    pub gmac_lc: GMAC_LC,
    #[doc = "0x148 - Deferred Transmission Frames Register"]
    pub gmac_dtf: GMAC_DTF,
    #[doc = "0x14c - Carrier Sense Errors Register"]
    pub gmac_cse: GMAC_CSE,
    #[doc = "0x150 - Octets Received Low Received Register"]
    pub gmac_orlo: GMAC_ORLO,
    #[doc = "0x154 - Octets Received High Received Register"]
    pub gmac_orhi: GMAC_ORHI,
    #[doc = "0x158 - Frames Received Register"]
    pub gmac_fr: GMAC_FR,
    #[doc = "0x15c - Broadcast Frames Received Register"]
    pub gmac_bcfr: GMAC_BCFR,
    #[doc = "0x160 - Multicast Frames Received Register"]
    pub gmac_mfr: GMAC_MFR,
    #[doc = "0x164 - Pause Frames Received Register"]
    pub gmac_pfr: GMAC_PFR,
    #[doc = "0x168 - 64 Byte Frames Received Register"]
    pub gmac_bfr64: GMAC_BFR64,
    #[doc = "0x16c - 65 to 127 Byte Frames Received Register"]
    pub gmac_tbfr127: GMAC_TBFR127,
    #[doc = "0x170 - 128 to 255 Byte Frames Received Register"]
    pub gmac_tbfr255: GMAC_TBFR255,
    #[doc = "0x174 - 256 to 511 Byte Frames Received Register"]
    pub gmac_tbfr511: GMAC_TBFR511,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received Register"]
    pub gmac_tbfr1023: GMAC_TBFR1023,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received Register"]
    pub gmac_tbfr1518: GMAC_TBFR1518,
    #[doc = "0x180 - 1519 to Maximum Byte Frames Received Register"]
    pub gmac_tmxbfr: GMAC_TMXBFR,
    #[doc = "0x184 - Undersize Frames Received Register"]
    pub gmac_ufr: GMAC_UFR,
    #[doc = "0x188 - Oversize Frames Received Register"]
    pub gmac_ofr: GMAC_OFR,
    #[doc = "0x18c - Jabbers Received Register"]
    pub gmac_jr: GMAC_JR,
    #[doc = "0x190 - Frame Check Sequence Errors Register"]
    pub gmac_fcse: GMAC_FCSE,
    #[doc = "0x194 - Length Field Frame Errors Register"]
    pub gmac_lffe: GMAC_LFFE,
    #[doc = "0x198 - Receive Symbol Errors Register"]
    pub gmac_rse: GMAC_RSE,
    #[doc = "0x19c - Alignment Errors Register"]
    pub gmac_ae: GMAC_AE,
    #[doc = "0x1a0 - Receive Resource Errors Register"]
    pub gmac_rre: GMAC_RRE,
    #[doc = "0x1a4 - Receive Overrun Register"]
    pub gmac_roe: GMAC_ROE,
    #[doc = "0x1a8 - IP Header Checksum Errors Register"]
    pub gmac_ihce: GMAC_IHCE,
    #[doc = "0x1ac - TCP Checksum Errors Register"]
    pub gmac_tce: GMAC_TCE,
    #[doc = "0x1b0 - UDP Checksum Errors Register"]
    pub gmac_uce: GMAC_UCE,
    _reserved87: [u8; 8usize],
    #[doc = "0x1bc - 1588 Timer Increment Sub-nanoseconds Register"]
    pub gmac_tisubn: GMAC_TISUBN,
    #[doc = "0x1c0 - 1588 Timer Seconds High Register"]
    pub gmac_tsh: GMAC_TSH,
    _reserved89: [u8; 12usize],
    #[doc = "0x1d0 - 1588 Timer Seconds Low Register"]
    pub gmac_tsl: GMAC_TSL,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub gmac_tn: GMAC_TN,
    #[doc = "0x1d8 - 1588 Timer Adjust Register"]
    pub gmac_ta: GMAC_TA,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub gmac_ti: GMAC_TI,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Low Register"]
    pub gmac_eftsl: GMAC_EFTSL,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub gmac_eftn: GMAC_EFTN,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Low Register"]
    pub gmac_efrsl: GMAC_EFRSL,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    pub gmac_efrn: GMAC_EFRN,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Low Register"]
    pub gmac_peftsl: GMAC_PEFTSL,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub gmac_peftn: GMAC_PEFTN,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Low Register"]
    pub gmac_pefrsl: GMAC_PEFRSL,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub gmac_pefrn: GMAC_PEFRN,
    _reserved101: [u8; 112usize],
    #[doc = "0x270 - Received LPI Transitions"]
    pub gmac_rxlpi: GMAC_RXLPI,
    #[doc = "0x274 - Received LPI Time"]
    pub gmac_rxlpitime: GMAC_RXLPITIME,
    #[doc = "0x278 - Transmit LPI Transitions"]
    pub gmac_txlpi: GMAC_TXLPI,
    #[doc = "0x27c - Transmit LPI Time"]
    pub gmac_txlpitime: GMAC_TXLPITIME,
    _reserved105: [u8; 384usize],
    #[doc = "0x400 - Interrupt Status Register Priority Queue (1..5)"]
    pub gmac_isrpq: [GMAC_ISRPQ; 5],
    _reserved106: [u8; 44usize],
    #[doc = "0x440 - Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
    pub gmac_tbqbapq: [GMAC_TBQBAPQ; 5],
    _reserved107: [u8; 44usize],
    #[doc = "0x480 - Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
    pub gmac_rbqbapq: [GMAC_RBQBAPQ; 5],
    _reserved108: [u8; 12usize],
    #[doc = "0x4a0 - Receive Buffer Size Register Priority Queue (1..5)"]
    pub gmac_rbsrpq: [GMAC_RBSRPQ; 5],
    _reserved109: [u8; 8usize],
    #[doc = "0x4bc - Credit-Based Shaping Control Register"]
    pub gmac_cbscr: GMAC_CBSCR,
    #[doc = "0x4c0 - Credit-Based Shaping IdleSlope Register for Queue A"]
    pub gmac_cbsisqa: GMAC_CBSISQA,
    #[doc = "0x4c4 - Credit-Based Shaping IdleSlope Register for Queue B"]
    pub gmac_cbsisqb: GMAC_CBSISQB,
    _reserved112: [u8; 56usize],
    #[doc = "0x500 - Screening Type 1 Register Priority Queue"]
    pub gmac_st1rpq: [GMAC_ST1RPQ; 4],
    _reserved113: [u8; 48usize],
    #[doc = "0x540 - Screening Type 2 Register Priority Queue"]
    pub gmac_st2rpq: [GMAC_ST2RPQ; 8],
    _reserved114: [u8; 160usize],
    #[doc = "0x600 - Interrupt Enable Register Priority Queue (1..5)"]
    pub gmac_ierpq: [GMAC_IERPQ; 5],
    _reserved115: [u8; 12usize],
    #[doc = "0x620 - Interrupt Disable Register Priority Queue (1..5)"]
    pub gmac_idrpq: [GMAC_IDRPQ; 5],
    _reserved116: [u8; 12usize],
    #[doc = "0x640 - Interrupt Mask Register Priority Queue (1..5)"]
    pub gmac_imrpq: [GMAC_IMRPQ; 5],
    _reserved117: [u8; 140usize],
    #[doc = "0x6e0 - Screening Type 2 Ethertype Register"]
    pub gmac_st2er: [GMAC_ST2ER; 4],
    _reserved118: [u8; 16usize],
    #[doc = "0x700 - Screening Type 2 Compare Word 0 Register"]
    pub gmac_st2cw: [GMAC_ST2CW; 24],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_SA {
    #[doc = "0x00 - Specific Address 1 Bottom Register"]
    pub gmac_sab: self::gmac_sa::GMAC_SAB,
    #[doc = "0x04 - Specific Address 1 Top Register"]
    pub gmac_sat: self::gmac_sa::GMAC_SAT,
}
#[doc = r"Register block"]
#[doc = "Specific Address 1 Bottom Register"]
pub mod gmac_sa;
#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_ST2CW {
    #[doc = "0x00 - Screening Type 2 Compare Word 0 Register"]
    pub gmac_st2cw0: self::gmac_st2cw::GMAC_ST2CW0,
    #[doc = "0x04 - Screening Type 2 Compare Word 1 Register"]
    pub gmac_st2cw1: self::gmac_st2cw::GMAC_ST2CW1,
}
#[doc = r"Register block"]
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub mod gmac_st2cw;
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ncr](gmac_ncr) module"]
pub type GMAC_NCR = crate::Reg<u32, _GMAC_NCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_NCR;
#[doc = "`read()` method returns [gmac_ncr::R](gmac_ncr::R) reader structure"]
impl crate::Readable for GMAC_NCR {}
#[doc = "`write(|w| ..)` method takes [gmac_ncr::W](gmac_ncr::W) writer structure"]
impl crate::Writable for GMAC_NCR {}
#[doc = "Network Control Register"]
pub mod gmac_ncr;
#[doc = "Network Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ncfgr](gmac_ncfgr) module"]
pub type GMAC_NCFGR = crate::Reg<u32, _GMAC_NCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_NCFGR;
#[doc = "`read()` method returns [gmac_ncfgr::R](gmac_ncfgr::R) reader structure"]
impl crate::Readable for GMAC_NCFGR {}
#[doc = "`write(|w| ..)` method takes [gmac_ncfgr::W](gmac_ncfgr::W) writer structure"]
impl crate::Writable for GMAC_NCFGR {}
#[doc = "Network Configuration Register"]
pub mod gmac_ncfgr;
#[doc = "Network Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_nsr](gmac_nsr) module"]
pub type GMAC_NSR = crate::Reg<u32, _GMAC_NSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_NSR;
#[doc = "`read()` method returns [gmac_nsr::R](gmac_nsr::R) reader structure"]
impl crate::Readable for GMAC_NSR {}
#[doc = "Network Status Register"]
pub mod gmac_nsr;
#[doc = "User Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ur](gmac_ur) module"]
pub type GMAC_UR = crate::Reg<u32, _GMAC_UR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_UR;
#[doc = "`read()` method returns [gmac_ur::R](gmac_ur::R) reader structure"]
impl crate::Readable for GMAC_UR {}
#[doc = "`write(|w| ..)` method takes [gmac_ur::W](gmac_ur::W) writer structure"]
impl crate::Writable for GMAC_UR {}
#[doc = "User Register"]
pub mod gmac_ur;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_dcfgr](gmac_dcfgr) module"]
pub type GMAC_DCFGR = crate::Reg<u32, _GMAC_DCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_DCFGR;
#[doc = "`read()` method returns [gmac_dcfgr::R](gmac_dcfgr::R) reader structure"]
impl crate::Readable for GMAC_DCFGR {}
#[doc = "`write(|w| ..)` method takes [gmac_dcfgr::W](gmac_dcfgr::W) writer structure"]
impl crate::Writable for GMAC_DCFGR {}
#[doc = "DMA Configuration Register"]
pub mod gmac_dcfgr;
#[doc = "Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tsr](gmac_tsr) module"]
pub type GMAC_TSR = crate::Reg<u32, _GMAC_TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TSR;
#[doc = "`read()` method returns [gmac_tsr::R](gmac_tsr::R) reader structure"]
impl crate::Readable for GMAC_TSR {}
#[doc = "`write(|w| ..)` method takes [gmac_tsr::W](gmac_tsr::W) writer structure"]
impl crate::Writable for GMAC_TSR {}
#[doc = "Transmit Status Register"]
pub mod gmac_tsr;
#[doc = "Receive Buffer Queue Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rbqb](gmac_rbqb) module"]
pub type GMAC_RBQB = crate::Reg<u32, _GMAC_RBQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RBQB;
#[doc = "`read()` method returns [gmac_rbqb::R](gmac_rbqb::R) reader structure"]
impl crate::Readable for GMAC_RBQB {}
#[doc = "`write(|w| ..)` method takes [gmac_rbqb::W](gmac_rbqb::W) writer structure"]
impl crate::Writable for GMAC_RBQB {}
#[doc = "Receive Buffer Queue Base Address Register"]
pub mod gmac_rbqb;
#[doc = "Transmit Buffer Queue Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbqb](gmac_tbqb) module"]
pub type GMAC_TBQB = crate::Reg<u32, _GMAC_TBQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBQB;
#[doc = "`read()` method returns [gmac_tbqb::R](gmac_tbqb::R) reader structure"]
impl crate::Readable for GMAC_TBQB {}
#[doc = "`write(|w| ..)` method takes [gmac_tbqb::W](gmac_tbqb::W) writer structure"]
impl crate::Writable for GMAC_TBQB {}
#[doc = "Transmit Buffer Queue Base Address Register"]
pub mod gmac_tbqb;
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rsr](gmac_rsr) module"]
pub type GMAC_RSR = crate::Reg<u32, _GMAC_RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RSR;
#[doc = "`read()` method returns [gmac_rsr::R](gmac_rsr::R) reader structure"]
impl crate::Readable for GMAC_RSR {}
#[doc = "`write(|w| ..)` method takes [gmac_rsr::W](gmac_rsr::W) writer structure"]
impl crate::Writable for GMAC_RSR {}
#[doc = "Receive Status Register"]
pub mod gmac_rsr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_isr](gmac_isr) module"]
pub type GMAC_ISR = crate::Reg<u32, _GMAC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ISR;
#[doc = "`read()` method returns [gmac_isr::R](gmac_isr::R) reader structure"]
impl crate::Readable for GMAC_ISR {}
#[doc = "Interrupt Status Register"]
pub mod gmac_isr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ier](gmac_ier) module"]
pub type GMAC_IER = crate::Reg<u32, _GMAC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IER;
#[doc = "`write(|w| ..)` method takes [gmac_ier::W](gmac_ier::W) writer structure"]
impl crate::Writable for GMAC_IER {}
#[doc = "Interrupt Enable Register"]
pub mod gmac_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_idr](gmac_idr) module"]
pub type GMAC_IDR = crate::Reg<u32, _GMAC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IDR;
#[doc = "`write(|w| ..)` method takes [gmac_idr::W](gmac_idr::W) writer structure"]
impl crate::Writable for GMAC_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod gmac_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_imr](gmac_imr) module"]
pub type GMAC_IMR = crate::Reg<u32, _GMAC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IMR;
#[doc = "`read()` method returns [gmac_imr::R](gmac_imr::R) reader structure"]
impl crate::Readable for GMAC_IMR {}
#[doc = "`write(|w| ..)` method takes [gmac_imr::W](gmac_imr::W) writer structure"]
impl crate::Writable for GMAC_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod gmac_imr;
#[doc = "PHY Maintenance Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_man](gmac_man) module"]
pub type GMAC_MAN = crate::Reg<u32, _GMAC_MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_MAN;
#[doc = "`read()` method returns [gmac_man::R](gmac_man::R) reader structure"]
impl crate::Readable for GMAC_MAN {}
#[doc = "`write(|w| ..)` method takes [gmac_man::W](gmac_man::W) writer structure"]
impl crate::Writable for GMAC_MAN {}
#[doc = "PHY Maintenance Register"]
pub mod gmac_man;
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rpq](gmac_rpq) module"]
pub type GMAC_RPQ = crate::Reg<u32, _GMAC_RPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RPQ;
#[doc = "`read()` method returns [gmac_rpq::R](gmac_rpq::R) reader structure"]
impl crate::Readable for GMAC_RPQ {}
#[doc = "Received Pause Quantum Register"]
pub mod gmac_rpq;
#[doc = "Transmit Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tpq](gmac_tpq) module"]
pub type GMAC_TPQ = crate::Reg<u32, _GMAC_TPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TPQ;
#[doc = "`read()` method returns [gmac_tpq::R](gmac_tpq::R) reader structure"]
impl crate::Readable for GMAC_TPQ {}
#[doc = "`write(|w| ..)` method takes [gmac_tpq::W](gmac_tpq::W) writer structure"]
impl crate::Writable for GMAC_TPQ {}
#[doc = "Transmit Pause Quantum Register"]
pub mod gmac_tpq;
#[doc = "TX Partial Store and Forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tpsf](gmac_tpsf) module"]
pub type GMAC_TPSF = crate::Reg<u32, _GMAC_TPSF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TPSF;
#[doc = "`read()` method returns [gmac_tpsf::R](gmac_tpsf::R) reader structure"]
impl crate::Readable for GMAC_TPSF {}
#[doc = "`write(|w| ..)` method takes [gmac_tpsf::W](gmac_tpsf::W) writer structure"]
impl crate::Writable for GMAC_TPSF {}
#[doc = "TX Partial Store and Forward Register"]
pub mod gmac_tpsf;
#[doc = "RX Partial Store and Forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rpsf](gmac_rpsf) module"]
pub type GMAC_RPSF = crate::Reg<u32, _GMAC_RPSF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RPSF;
#[doc = "`read()` method returns [gmac_rpsf::R](gmac_rpsf::R) reader structure"]
impl crate::Readable for GMAC_RPSF {}
#[doc = "`write(|w| ..)` method takes [gmac_rpsf::W](gmac_rpsf::W) writer structure"]
impl crate::Writable for GMAC_RPSF {}
#[doc = "RX Partial Store and Forward Register"]
pub mod gmac_rpsf;
#[doc = "RX Jumbo Frame Max Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rjfml](gmac_rjfml) module"]
pub type GMAC_RJFML = crate::Reg<u32, _GMAC_RJFML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RJFML;
#[doc = "`read()` method returns [gmac_rjfml::R](gmac_rjfml::R) reader structure"]
impl crate::Readable for GMAC_RJFML {}
#[doc = "`write(|w| ..)` method takes [gmac_rjfml::W](gmac_rjfml::W) writer structure"]
impl crate::Writable for GMAC_RJFML {}
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod gmac_rjfml;
#[doc = "Hash Register Bottom\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_hrb](gmac_hrb) module"]
pub type GMAC_HRB = crate::Reg<u32, _GMAC_HRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_HRB;
#[doc = "`read()` method returns [gmac_hrb::R](gmac_hrb::R) reader structure"]
impl crate::Readable for GMAC_HRB {}
#[doc = "`write(|w| ..)` method takes [gmac_hrb::W](gmac_hrb::W) writer structure"]
impl crate::Writable for GMAC_HRB {}
#[doc = "Hash Register Bottom"]
pub mod gmac_hrb;
#[doc = "Hash Register Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_hrt](gmac_hrt) module"]
pub type GMAC_HRT = crate::Reg<u32, _GMAC_HRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_HRT;
#[doc = "`read()` method returns [gmac_hrt::R](gmac_hrt::R) reader structure"]
impl crate::Readable for GMAC_HRT {}
#[doc = "`write(|w| ..)` method takes [gmac_hrt::W](gmac_hrt::W) writer structure"]
impl crate::Writable for GMAC_HRT {}
#[doc = "Hash Register Top"]
pub mod gmac_hrt;
#[doc = "Type ID Match 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tidm1](gmac_tidm1) module"]
pub type GMAC_TIDM1 = crate::Reg<u32, _GMAC_TIDM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TIDM1;
#[doc = "`read()` method returns [gmac_tidm1::R](gmac_tidm1::R) reader structure"]
impl crate::Readable for GMAC_TIDM1 {}
#[doc = "`write(|w| ..)` method takes [gmac_tidm1::W](gmac_tidm1::W) writer structure"]
impl crate::Writable for GMAC_TIDM1 {}
#[doc = "Type ID Match 1 Register"]
pub mod gmac_tidm1;
#[doc = "Type ID Match 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tidm2](gmac_tidm2) module"]
pub type GMAC_TIDM2 = crate::Reg<u32, _GMAC_TIDM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TIDM2;
#[doc = "`read()` method returns [gmac_tidm2::R](gmac_tidm2::R) reader structure"]
impl crate::Readable for GMAC_TIDM2 {}
#[doc = "`write(|w| ..)` method takes [gmac_tidm2::W](gmac_tidm2::W) writer structure"]
impl crate::Writable for GMAC_TIDM2 {}
#[doc = "Type ID Match 2 Register"]
pub mod gmac_tidm2;
#[doc = "Type ID Match 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tidm3](gmac_tidm3) module"]
pub type GMAC_TIDM3 = crate::Reg<u32, _GMAC_TIDM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TIDM3;
#[doc = "`read()` method returns [gmac_tidm3::R](gmac_tidm3::R) reader structure"]
impl crate::Readable for GMAC_TIDM3 {}
#[doc = "`write(|w| ..)` method takes [gmac_tidm3::W](gmac_tidm3::W) writer structure"]
impl crate::Writable for GMAC_TIDM3 {}
#[doc = "Type ID Match 3 Register"]
pub mod gmac_tidm3;
#[doc = "Type ID Match 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tidm4](gmac_tidm4) module"]
pub type GMAC_TIDM4 = crate::Reg<u32, _GMAC_TIDM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TIDM4;
#[doc = "`read()` method returns [gmac_tidm4::R](gmac_tidm4::R) reader structure"]
impl crate::Readable for GMAC_TIDM4 {}
#[doc = "`write(|w| ..)` method takes [gmac_tidm4::W](gmac_tidm4::W) writer structure"]
impl crate::Writable for GMAC_TIDM4 {}
#[doc = "Type ID Match 4 Register"]
pub mod gmac_tidm4;
#[doc = "Wake on LAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_wol](gmac_wol) module"]
pub type GMAC_WOL = crate::Reg<u32, _GMAC_WOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_WOL;
#[doc = "`read()` method returns [gmac_wol::R](gmac_wol::R) reader structure"]
impl crate::Readable for GMAC_WOL {}
#[doc = "`write(|w| ..)` method takes [gmac_wol::W](gmac_wol::W) writer structure"]
impl crate::Writable for GMAC_WOL {}
#[doc = "Wake on LAN Register"]
pub mod gmac_wol;
#[doc = "IPG Stretch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ipgs](gmac_ipgs) module"]
pub type GMAC_IPGS = crate::Reg<u32, _GMAC_IPGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IPGS;
#[doc = "`read()` method returns [gmac_ipgs::R](gmac_ipgs::R) reader structure"]
impl crate::Readable for GMAC_IPGS {}
#[doc = "`write(|w| ..)` method takes [gmac_ipgs::W](gmac_ipgs::W) writer structure"]
impl crate::Writable for GMAC_IPGS {}
#[doc = "IPG Stretch Register"]
pub mod gmac_ipgs;
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_svlan](gmac_svlan) module"]
pub type GMAC_SVLAN = crate::Reg<u32, _GMAC_SVLAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SVLAN;
#[doc = "`read()` method returns [gmac_svlan::R](gmac_svlan::R) reader structure"]
impl crate::Readable for GMAC_SVLAN {}
#[doc = "`write(|w| ..)` method takes [gmac_svlan::W](gmac_svlan::W) writer structure"]
impl crate::Writable for GMAC_SVLAN {}
#[doc = "Stacked VLAN Register"]
pub mod gmac_svlan;
#[doc = "Transmit PFC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tpfcp](gmac_tpfcp) module"]
pub type GMAC_TPFCP = crate::Reg<u32, _GMAC_TPFCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TPFCP;
#[doc = "`read()` method returns [gmac_tpfcp::R](gmac_tpfcp::R) reader structure"]
impl crate::Readable for GMAC_TPFCP {}
#[doc = "`write(|w| ..)` method takes [gmac_tpfcp::W](gmac_tpfcp::W) writer structure"]
impl crate::Writable for GMAC_TPFCP {}
#[doc = "Transmit PFC Pause Register"]
pub mod gmac_tpfcp;
#[doc = "Specific Address 1 Mask Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_samb1](gmac_samb1) module"]
pub type GMAC_SAMB1 = crate::Reg<u32, _GMAC_SAMB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SAMB1;
#[doc = "`read()` method returns [gmac_samb1::R](gmac_samb1::R) reader structure"]
impl crate::Readable for GMAC_SAMB1 {}
#[doc = "`write(|w| ..)` method takes [gmac_samb1::W](gmac_samb1::W) writer structure"]
impl crate::Writable for GMAC_SAMB1 {}
#[doc = "Specific Address 1 Mask Bottom Register"]
pub mod gmac_samb1;
#[doc = "Specific Address 1 Mask Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_samt1](gmac_samt1) module"]
pub type GMAC_SAMT1 = crate::Reg<u32, _GMAC_SAMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SAMT1;
#[doc = "`read()` method returns [gmac_samt1::R](gmac_samt1::R) reader structure"]
impl crate::Readable for GMAC_SAMT1 {}
#[doc = "`write(|w| ..)` method takes [gmac_samt1::W](gmac_samt1::W) writer structure"]
impl crate::Writable for GMAC_SAMT1 {}
#[doc = "Specific Address 1 Mask Top Register"]
pub mod gmac_samt1;
#[doc = "1588 Timer Nanosecond Comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_nsc](gmac_nsc) module"]
pub type GMAC_NSC = crate::Reg<u32, _GMAC_NSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_NSC;
#[doc = "`read()` method returns [gmac_nsc::R](gmac_nsc::R) reader structure"]
impl crate::Readable for GMAC_NSC {}
#[doc = "`write(|w| ..)` method takes [gmac_nsc::W](gmac_nsc::W) writer structure"]
impl crate::Writable for GMAC_NSC {}
#[doc = "1588 Timer Nanosecond Comparison Register"]
pub mod gmac_nsc;
#[doc = "1588 Timer Second Comparison Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_scl](gmac_scl) module"]
pub type GMAC_SCL = crate::Reg<u32, _GMAC_SCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SCL;
#[doc = "`read()` method returns [gmac_scl::R](gmac_scl::R) reader structure"]
impl crate::Readable for GMAC_SCL {}
#[doc = "`write(|w| ..)` method takes [gmac_scl::W](gmac_scl::W) writer structure"]
impl crate::Writable for GMAC_SCL {}
#[doc = "1588 Timer Second Comparison Low Register"]
pub mod gmac_scl;
#[doc = "1588 Timer Second Comparison High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_sch](gmac_sch) module"]
pub type GMAC_SCH = crate::Reg<u32, _GMAC_SCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SCH;
#[doc = "`read()` method returns [gmac_sch::R](gmac_sch::R) reader structure"]
impl crate::Readable for GMAC_SCH {}
#[doc = "`write(|w| ..)` method takes [gmac_sch::W](gmac_sch::W) writer structure"]
impl crate::Writable for GMAC_SCH {}
#[doc = "1588 Timer Second Comparison High Register"]
pub mod gmac_sch;
#[doc = "PTP Event Frame Transmitted Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_eftsh](gmac_eftsh) module"]
pub type GMAC_EFTSH = crate::Reg<u32, _GMAC_EFTSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_EFTSH;
#[doc = "`read()` method returns [gmac_eftsh::R](gmac_eftsh::R) reader structure"]
impl crate::Readable for GMAC_EFTSH {}
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod gmac_eftsh;
#[doc = "PTP Event Frame Received Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_efrsh](gmac_efrsh) module"]
pub type GMAC_EFRSH = crate::Reg<u32, _GMAC_EFRSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_EFRSH;
#[doc = "`read()` method returns [gmac_efrsh::R](gmac_efrsh::R) reader structure"]
impl crate::Readable for GMAC_EFRSH {}
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod gmac_efrsh;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_peftsh](gmac_peftsh) module"]
pub type GMAC_PEFTSH = crate::Reg<u32, _GMAC_PEFTSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PEFTSH;
#[doc = "`read()` method returns [gmac_peftsh::R](gmac_peftsh::R) reader structure"]
impl crate::Readable for GMAC_PEFTSH {}
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod gmac_peftsh;
#[doc = "PTP Peer Event Frame Received Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_pefrsh](gmac_pefrsh) module"]
pub type GMAC_PEFRSH = crate::Reg<u32, _GMAC_PEFRSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PEFRSH;
#[doc = "`read()` method returns [gmac_pefrsh::R](gmac_pefrsh::R) reader structure"]
impl crate::Readable for GMAC_PEFRSH {}
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod gmac_pefrsh;
#[doc = "Octets Transmitted Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_otlo](gmac_otlo) module"]
pub type GMAC_OTLO = crate::Reg<u32, _GMAC_OTLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_OTLO;
#[doc = "`read()` method returns [gmac_otlo::R](gmac_otlo::R) reader structure"]
impl crate::Readable for GMAC_OTLO {}
#[doc = "Octets Transmitted Low Register"]
pub mod gmac_otlo;
#[doc = "Octets Transmitted High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_othi](gmac_othi) module"]
pub type GMAC_OTHI = crate::Reg<u32, _GMAC_OTHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_OTHI;
#[doc = "`read()` method returns [gmac_othi::R](gmac_othi::R) reader structure"]
impl crate::Readable for GMAC_OTHI {}
#[doc = "Octets Transmitted High Register"]
pub mod gmac_othi;
#[doc = "Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ft](gmac_ft) module"]
pub type GMAC_FT = crate::Reg<u32, _GMAC_FT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_FT;
#[doc = "`read()` method returns [gmac_ft::R](gmac_ft::R) reader structure"]
impl crate::Readable for GMAC_FT {}
#[doc = "Frames Transmitted Register"]
pub mod gmac_ft;
#[doc = "Broadcast Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_bcft](gmac_bcft) module"]
pub type GMAC_BCFT = crate::Reg<u32, _GMAC_BCFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_BCFT;
#[doc = "`read()` method returns [gmac_bcft::R](gmac_bcft::R) reader structure"]
impl crate::Readable for GMAC_BCFT {}
#[doc = "Broadcast Frames Transmitted Register"]
pub mod gmac_bcft;
#[doc = "Multicast Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_mft](gmac_mft) module"]
pub type GMAC_MFT = crate::Reg<u32, _GMAC_MFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_MFT;
#[doc = "`read()` method returns [gmac_mft::R](gmac_mft::R) reader structure"]
impl crate::Readable for GMAC_MFT {}
#[doc = "Multicast Frames Transmitted Register"]
pub mod gmac_mft;
#[doc = "Pause Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_pft](gmac_pft) module"]
pub type GMAC_PFT = crate::Reg<u32, _GMAC_PFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PFT;
#[doc = "`read()` method returns [gmac_pft::R](gmac_pft::R) reader structure"]
impl crate::Readable for GMAC_PFT {}
#[doc = "Pause Frames Transmitted Register"]
pub mod gmac_pft;
#[doc = "64 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_bft64](gmac_bft64) module"]
pub type GMAC_BFT64 = crate::Reg<u32, _GMAC_BFT64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_BFT64;
#[doc = "`read()` method returns [gmac_bft64::R](gmac_bft64::R) reader structure"]
impl crate::Readable for GMAC_BFT64 {}
#[doc = "64 Byte Frames Transmitted Register"]
pub mod gmac_bft64;
#[doc = "65 to 127 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbft127](gmac_tbft127) module"]
pub type GMAC_TBFT127 = crate::Reg<u32, _GMAC_TBFT127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFT127;
#[doc = "`read()` method returns [gmac_tbft127::R](gmac_tbft127::R) reader structure"]
impl crate::Readable for GMAC_TBFT127 {}
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod gmac_tbft127;
#[doc = "128 to 255 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbft255](gmac_tbft255) module"]
pub type GMAC_TBFT255 = crate::Reg<u32, _GMAC_TBFT255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFT255;
#[doc = "`read()` method returns [gmac_tbft255::R](gmac_tbft255::R) reader structure"]
impl crate::Readable for GMAC_TBFT255 {}
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod gmac_tbft255;
#[doc = "256 to 511 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbft511](gmac_tbft511) module"]
pub type GMAC_TBFT511 = crate::Reg<u32, _GMAC_TBFT511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFT511;
#[doc = "`read()` method returns [gmac_tbft511::R](gmac_tbft511::R) reader structure"]
impl crate::Readable for GMAC_TBFT511 {}
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod gmac_tbft511;
#[doc = "512 to 1023 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbft1023](gmac_tbft1023) module"]
pub type GMAC_TBFT1023 = crate::Reg<u32, _GMAC_TBFT1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFT1023;
#[doc = "`read()` method returns [gmac_tbft1023::R](gmac_tbft1023::R) reader structure"]
impl crate::Readable for GMAC_TBFT1023 {}
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod gmac_tbft1023;
#[doc = "1024 to 1518 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbft1518](gmac_tbft1518) module"]
pub type GMAC_TBFT1518 = crate::Reg<u32, _GMAC_TBFT1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFT1518;
#[doc = "`read()` method returns [gmac_tbft1518::R](gmac_tbft1518::R) reader structure"]
impl crate::Readable for GMAC_TBFT1518 {}
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod gmac_tbft1518;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_gtbft1518](gmac_gtbft1518) module"]
pub type GMAC_GTBFT1518 = crate::Reg<u32, _GMAC_GTBFT1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_GTBFT1518;
#[doc = "`read()` method returns [gmac_gtbft1518::R](gmac_gtbft1518::R) reader structure"]
impl crate::Readable for GMAC_GTBFT1518 {}
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gmac_gtbft1518;
#[doc = "Transmit Underruns Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tur](gmac_tur) module"]
pub type GMAC_TUR = crate::Reg<u32, _GMAC_TUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TUR;
#[doc = "`read()` method returns [gmac_tur::R](gmac_tur::R) reader structure"]
impl crate::Readable for GMAC_TUR {}
#[doc = "Transmit Underruns Register"]
pub mod gmac_tur;
#[doc = "Single Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_scf](gmac_scf) module"]
pub type GMAC_SCF = crate::Reg<u32, _GMAC_SCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SCF;
#[doc = "`read()` method returns [gmac_scf::R](gmac_scf::R) reader structure"]
impl crate::Readable for GMAC_SCF {}
#[doc = "Single Collision Frames Register"]
pub mod gmac_scf;
#[doc = "Multiple Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_mcf](gmac_mcf) module"]
pub type GMAC_MCF = crate::Reg<u32, _GMAC_MCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_MCF;
#[doc = "`read()` method returns [gmac_mcf::R](gmac_mcf::R) reader structure"]
impl crate::Readable for GMAC_MCF {}
#[doc = "Multiple Collision Frames Register"]
pub mod gmac_mcf;
#[doc = "Excessive Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ec](gmac_ec) module"]
pub type GMAC_EC = crate::Reg<u32, _GMAC_EC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_EC;
#[doc = "`read()` method returns [gmac_ec::R](gmac_ec::R) reader structure"]
impl crate::Readable for GMAC_EC {}
#[doc = "Excessive Collisions Register"]
pub mod gmac_ec;
#[doc = "Late Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_lc](gmac_lc) module"]
pub type GMAC_LC = crate::Reg<u32, _GMAC_LC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_LC;
#[doc = "`read()` method returns [gmac_lc::R](gmac_lc::R) reader structure"]
impl crate::Readable for GMAC_LC {}
#[doc = "Late Collisions Register"]
pub mod gmac_lc;
#[doc = "Deferred Transmission Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_dtf](gmac_dtf) module"]
pub type GMAC_DTF = crate::Reg<u32, _GMAC_DTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_DTF;
#[doc = "`read()` method returns [gmac_dtf::R](gmac_dtf::R) reader structure"]
impl crate::Readable for GMAC_DTF {}
#[doc = "Deferred Transmission Frames Register"]
pub mod gmac_dtf;
#[doc = "Carrier Sense Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_cse](gmac_cse) module"]
pub type GMAC_CSE = crate::Reg<u32, _GMAC_CSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_CSE;
#[doc = "`read()` method returns [gmac_cse::R](gmac_cse::R) reader structure"]
impl crate::Readable for GMAC_CSE {}
#[doc = "Carrier Sense Errors Register"]
pub mod gmac_cse;
#[doc = "Octets Received Low Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_orlo](gmac_orlo) module"]
pub type GMAC_ORLO = crate::Reg<u32, _GMAC_ORLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ORLO;
#[doc = "`read()` method returns [gmac_orlo::R](gmac_orlo::R) reader structure"]
impl crate::Readable for GMAC_ORLO {}
#[doc = "Octets Received Low Received Register"]
pub mod gmac_orlo;
#[doc = "Octets Received High Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_orhi](gmac_orhi) module"]
pub type GMAC_ORHI = crate::Reg<u32, _GMAC_ORHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ORHI;
#[doc = "`read()` method returns [gmac_orhi::R](gmac_orhi::R) reader structure"]
impl crate::Readable for GMAC_ORHI {}
#[doc = "Octets Received High Received Register"]
pub mod gmac_orhi;
#[doc = "Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_fr](gmac_fr) module"]
pub type GMAC_FR = crate::Reg<u32, _GMAC_FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_FR;
#[doc = "`read()` method returns [gmac_fr::R](gmac_fr::R) reader structure"]
impl crate::Readable for GMAC_FR {}
#[doc = "Frames Received Register"]
pub mod gmac_fr;
#[doc = "Broadcast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_bcfr](gmac_bcfr) module"]
pub type GMAC_BCFR = crate::Reg<u32, _GMAC_BCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_BCFR;
#[doc = "`read()` method returns [gmac_bcfr::R](gmac_bcfr::R) reader structure"]
impl crate::Readable for GMAC_BCFR {}
#[doc = "Broadcast Frames Received Register"]
pub mod gmac_bcfr;
#[doc = "Multicast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_mfr](gmac_mfr) module"]
pub type GMAC_MFR = crate::Reg<u32, _GMAC_MFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_MFR;
#[doc = "`read()` method returns [gmac_mfr::R](gmac_mfr::R) reader structure"]
impl crate::Readable for GMAC_MFR {}
#[doc = "Multicast Frames Received Register"]
pub mod gmac_mfr;
#[doc = "Pause Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_pfr](gmac_pfr) module"]
pub type GMAC_PFR = crate::Reg<u32, _GMAC_PFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PFR;
#[doc = "`read()` method returns [gmac_pfr::R](gmac_pfr::R) reader structure"]
impl crate::Readable for GMAC_PFR {}
#[doc = "Pause Frames Received Register"]
pub mod gmac_pfr;
#[doc = "64 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_bfr64](gmac_bfr64) module"]
pub type GMAC_BFR64 = crate::Reg<u32, _GMAC_BFR64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_BFR64;
#[doc = "`read()` method returns [gmac_bfr64::R](gmac_bfr64::R) reader structure"]
impl crate::Readable for GMAC_BFR64 {}
#[doc = "64 Byte Frames Received Register"]
pub mod gmac_bfr64;
#[doc = "65 to 127 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbfr127](gmac_tbfr127) module"]
pub type GMAC_TBFR127 = crate::Reg<u32, _GMAC_TBFR127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFR127;
#[doc = "`read()` method returns [gmac_tbfr127::R](gmac_tbfr127::R) reader structure"]
impl crate::Readable for GMAC_TBFR127 {}
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod gmac_tbfr127;
#[doc = "128 to 255 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbfr255](gmac_tbfr255) module"]
pub type GMAC_TBFR255 = crate::Reg<u32, _GMAC_TBFR255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFR255;
#[doc = "`read()` method returns [gmac_tbfr255::R](gmac_tbfr255::R) reader structure"]
impl crate::Readable for GMAC_TBFR255 {}
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod gmac_tbfr255;
#[doc = "256 to 511 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbfr511](gmac_tbfr511) module"]
pub type GMAC_TBFR511 = crate::Reg<u32, _GMAC_TBFR511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFR511;
#[doc = "`read()` method returns [gmac_tbfr511::R](gmac_tbfr511::R) reader structure"]
impl crate::Readable for GMAC_TBFR511 {}
#[doc = "256 to 511 Byte Frames Received Register"]
pub mod gmac_tbfr511;
#[doc = "512 to 1023 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbfr1023](gmac_tbfr1023) module"]
pub type GMAC_TBFR1023 = crate::Reg<u32, _GMAC_TBFR1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFR1023;
#[doc = "`read()` method returns [gmac_tbfr1023::R](gmac_tbfr1023::R) reader structure"]
impl crate::Readable for GMAC_TBFR1023 {}
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod gmac_tbfr1023;
#[doc = "1024 to 1518 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbfr1518](gmac_tbfr1518) module"]
pub type GMAC_TBFR1518 = crate::Reg<u32, _GMAC_TBFR1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBFR1518;
#[doc = "`read()` method returns [gmac_tbfr1518::R](gmac_tbfr1518::R) reader structure"]
impl crate::Readable for GMAC_TBFR1518 {}
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod gmac_tbfr1518;
#[doc = "1519 to Maximum Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tmxbfr](gmac_tmxbfr) module"]
pub type GMAC_TMXBFR = crate::Reg<u32, _GMAC_TMXBFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TMXBFR;
#[doc = "`read()` method returns [gmac_tmxbfr::R](gmac_tmxbfr::R) reader structure"]
impl crate::Readable for GMAC_TMXBFR {}
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod gmac_tmxbfr;
#[doc = "Undersize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ufr](gmac_ufr) module"]
pub type GMAC_UFR = crate::Reg<u32, _GMAC_UFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_UFR;
#[doc = "`read()` method returns [gmac_ufr::R](gmac_ufr::R) reader structure"]
impl crate::Readable for GMAC_UFR {}
#[doc = "Undersize Frames Received Register"]
pub mod gmac_ufr;
#[doc = "Oversize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ofr](gmac_ofr) module"]
pub type GMAC_OFR = crate::Reg<u32, _GMAC_OFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_OFR;
#[doc = "`read()` method returns [gmac_ofr::R](gmac_ofr::R) reader structure"]
impl crate::Readable for GMAC_OFR {}
#[doc = "Oversize Frames Received Register"]
pub mod gmac_ofr;
#[doc = "Jabbers Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_jr](gmac_jr) module"]
pub type GMAC_JR = crate::Reg<u32, _GMAC_JR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_JR;
#[doc = "`read()` method returns [gmac_jr::R](gmac_jr::R) reader structure"]
impl crate::Readable for GMAC_JR {}
#[doc = "Jabbers Received Register"]
pub mod gmac_jr;
#[doc = "Frame Check Sequence Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_fcse](gmac_fcse) module"]
pub type GMAC_FCSE = crate::Reg<u32, _GMAC_FCSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_FCSE;
#[doc = "`read()` method returns [gmac_fcse::R](gmac_fcse::R) reader structure"]
impl crate::Readable for GMAC_FCSE {}
#[doc = "Frame Check Sequence Errors Register"]
pub mod gmac_fcse;
#[doc = "Length Field Frame Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_lffe](gmac_lffe) module"]
pub type GMAC_LFFE = crate::Reg<u32, _GMAC_LFFE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_LFFE;
#[doc = "`read()` method returns [gmac_lffe::R](gmac_lffe::R) reader structure"]
impl crate::Readable for GMAC_LFFE {}
#[doc = "Length Field Frame Errors Register"]
pub mod gmac_lffe;
#[doc = "Receive Symbol Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rse](gmac_rse) module"]
pub type GMAC_RSE = crate::Reg<u32, _GMAC_RSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RSE;
#[doc = "`read()` method returns [gmac_rse::R](gmac_rse::R) reader structure"]
impl crate::Readable for GMAC_RSE {}
#[doc = "Receive Symbol Errors Register"]
pub mod gmac_rse;
#[doc = "Alignment Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ae](gmac_ae) module"]
pub type GMAC_AE = crate::Reg<u32, _GMAC_AE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_AE;
#[doc = "`read()` method returns [gmac_ae::R](gmac_ae::R) reader structure"]
impl crate::Readable for GMAC_AE {}
#[doc = "Alignment Errors Register"]
pub mod gmac_ae;
#[doc = "Receive Resource Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rre](gmac_rre) module"]
pub type GMAC_RRE = crate::Reg<u32, _GMAC_RRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RRE;
#[doc = "`read()` method returns [gmac_rre::R](gmac_rre::R) reader structure"]
impl crate::Readable for GMAC_RRE {}
#[doc = "Receive Resource Errors Register"]
pub mod gmac_rre;
#[doc = "Receive Overrun Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_roe](gmac_roe) module"]
pub type GMAC_ROE = crate::Reg<u32, _GMAC_ROE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ROE;
#[doc = "`read()` method returns [gmac_roe::R](gmac_roe::R) reader structure"]
impl crate::Readable for GMAC_ROE {}
#[doc = "Receive Overrun Register"]
pub mod gmac_roe;
#[doc = "IP Header Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ihce](gmac_ihce) module"]
pub type GMAC_IHCE = crate::Reg<u32, _GMAC_IHCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IHCE;
#[doc = "`read()` method returns [gmac_ihce::R](gmac_ihce::R) reader structure"]
impl crate::Readable for GMAC_IHCE {}
#[doc = "IP Header Checksum Errors Register"]
pub mod gmac_ihce;
#[doc = "TCP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tce](gmac_tce) module"]
pub type GMAC_TCE = crate::Reg<u32, _GMAC_TCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TCE;
#[doc = "`read()` method returns [gmac_tce::R](gmac_tce::R) reader structure"]
impl crate::Readable for GMAC_TCE {}
#[doc = "TCP Checksum Errors Register"]
pub mod gmac_tce;
#[doc = "UDP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_uce](gmac_uce) module"]
pub type GMAC_UCE = crate::Reg<u32, _GMAC_UCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_UCE;
#[doc = "`read()` method returns [gmac_uce::R](gmac_uce::R) reader structure"]
impl crate::Readable for GMAC_UCE {}
#[doc = "UDP Checksum Errors Register"]
pub mod gmac_uce;
#[doc = "1588 Timer Increment Sub-nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tisubn](gmac_tisubn) module"]
pub type GMAC_TISUBN = crate::Reg<u32, _GMAC_TISUBN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TISUBN;
#[doc = "`read()` method returns [gmac_tisubn::R](gmac_tisubn::R) reader structure"]
impl crate::Readable for GMAC_TISUBN {}
#[doc = "`write(|w| ..)` method takes [gmac_tisubn::W](gmac_tisubn::W) writer structure"]
impl crate::Writable for GMAC_TISUBN {}
#[doc = "1588 Timer Increment Sub-nanoseconds Register"]
pub mod gmac_tisubn;
#[doc = "1588 Timer Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tsh](gmac_tsh) module"]
pub type GMAC_TSH = crate::Reg<u32, _GMAC_TSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TSH;
#[doc = "`read()` method returns [gmac_tsh::R](gmac_tsh::R) reader structure"]
impl crate::Readable for GMAC_TSH {}
#[doc = "`write(|w| ..)` method takes [gmac_tsh::W](gmac_tsh::W) writer structure"]
impl crate::Writable for GMAC_TSH {}
#[doc = "1588 Timer Seconds High Register"]
pub mod gmac_tsh;
#[doc = "1588 Timer Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tsl](gmac_tsl) module"]
pub type GMAC_TSL = crate::Reg<u32, _GMAC_TSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TSL;
#[doc = "`read()` method returns [gmac_tsl::R](gmac_tsl::R) reader structure"]
impl crate::Readable for GMAC_TSL {}
#[doc = "`write(|w| ..)` method takes [gmac_tsl::W](gmac_tsl::W) writer structure"]
impl crate::Writable for GMAC_TSL {}
#[doc = "1588 Timer Seconds Low Register"]
pub mod gmac_tsl;
#[doc = "1588 Timer Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tn](gmac_tn) module"]
pub type GMAC_TN = crate::Reg<u32, _GMAC_TN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TN;
#[doc = "`read()` method returns [gmac_tn::R](gmac_tn::R) reader structure"]
impl crate::Readable for GMAC_TN {}
#[doc = "`write(|w| ..)` method takes [gmac_tn::W](gmac_tn::W) writer structure"]
impl crate::Writable for GMAC_TN {}
#[doc = "1588 Timer Nanoseconds Register"]
pub mod gmac_tn;
#[doc = "1588 Timer Adjust Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ta](gmac_ta) module"]
pub type GMAC_TA = crate::Reg<u32, _GMAC_TA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TA;
#[doc = "`write(|w| ..)` method takes [gmac_ta::W](gmac_ta::W) writer structure"]
impl crate::Writable for GMAC_TA {}
#[doc = "1588 Timer Adjust Register"]
pub mod gmac_ta;
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ti](gmac_ti) module"]
pub type GMAC_TI = crate::Reg<u32, _GMAC_TI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TI;
#[doc = "`read()` method returns [gmac_ti::R](gmac_ti::R) reader structure"]
impl crate::Readable for GMAC_TI {}
#[doc = "`write(|w| ..)` method takes [gmac_ti::W](gmac_ti::W) writer structure"]
impl crate::Writable for GMAC_TI {}
#[doc = "1588 Timer Increment Register"]
pub mod gmac_ti;
#[doc = "PTP Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_eftsl](gmac_eftsl) module"]
pub type GMAC_EFTSL = crate::Reg<u32, _GMAC_EFTSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_EFTSL;
#[doc = "`read()` method returns [gmac_eftsl::R](gmac_eftsl::R) reader structure"]
impl crate::Readable for GMAC_EFTSL {}
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod gmac_eftsl;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_eftn](gmac_eftn) module"]
pub type GMAC_EFTN = crate::Reg<u32, _GMAC_EFTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_EFTN;
#[doc = "`read()` method returns [gmac_eftn::R](gmac_eftn::R) reader structure"]
impl crate::Readable for GMAC_EFTN {}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod gmac_eftn;
#[doc = "PTP Event Frame Received Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_efrsl](gmac_efrsl) module"]
pub type GMAC_EFRSL = crate::Reg<u32, _GMAC_EFRSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_EFRSL;
#[doc = "`read()` method returns [gmac_efrsl::R](gmac_efrsl::R) reader structure"]
impl crate::Readable for GMAC_EFRSL {}
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod gmac_efrsl;
#[doc = "PTP Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_efrn](gmac_efrn) module"]
pub type GMAC_EFRN = crate::Reg<u32, _GMAC_EFRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_EFRN;
#[doc = "`read()` method returns [gmac_efrn::R](gmac_efrn::R) reader structure"]
impl crate::Readable for GMAC_EFRN {}
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod gmac_efrn;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_peftsl](gmac_peftsl) module"]
pub type GMAC_PEFTSL = crate::Reg<u32, _GMAC_PEFTSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PEFTSL;
#[doc = "`read()` method returns [gmac_peftsl::R](gmac_peftsl::R) reader structure"]
impl crate::Readable for GMAC_PEFTSL {}
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod gmac_peftsl;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_peftn](gmac_peftn) module"]
pub type GMAC_PEFTN = crate::Reg<u32, _GMAC_PEFTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PEFTN;
#[doc = "`read()` method returns [gmac_peftn::R](gmac_peftn::R) reader structure"]
impl crate::Readable for GMAC_PEFTN {}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod gmac_peftn;
#[doc = "PTP Peer Event Frame Received Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_pefrsl](gmac_pefrsl) module"]
pub type GMAC_PEFRSL = crate::Reg<u32, _GMAC_PEFRSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PEFRSL;
#[doc = "`read()` method returns [gmac_pefrsl::R](gmac_pefrsl::R) reader structure"]
impl crate::Readable for GMAC_PEFRSL {}
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod gmac_pefrsl;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_pefrn](gmac_pefrn) module"]
pub type GMAC_PEFRN = crate::Reg<u32, _GMAC_PEFRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_PEFRN;
#[doc = "`read()` method returns [gmac_pefrn::R](gmac_pefrn::R) reader structure"]
impl crate::Readable for GMAC_PEFRN {}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod gmac_pefrn;
#[doc = "Received LPI Transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rxlpi](gmac_rxlpi) module"]
pub type GMAC_RXLPI = crate::Reg<u32, _GMAC_RXLPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RXLPI;
#[doc = "`read()` method returns [gmac_rxlpi::R](gmac_rxlpi::R) reader structure"]
impl crate::Readable for GMAC_RXLPI {}
#[doc = "Received LPI Transitions"]
pub mod gmac_rxlpi;
#[doc = "Received LPI Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rxlpitime](gmac_rxlpitime) module"]
pub type GMAC_RXLPITIME = crate::Reg<u32, _GMAC_RXLPITIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RXLPITIME;
#[doc = "`read()` method returns [gmac_rxlpitime::R](gmac_rxlpitime::R) reader structure"]
impl crate::Readable for GMAC_RXLPITIME {}
#[doc = "Received LPI Time"]
pub mod gmac_rxlpitime;
#[doc = "Transmit LPI Transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_txlpi](gmac_txlpi) module"]
pub type GMAC_TXLPI = crate::Reg<u32, _GMAC_TXLPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TXLPI;
#[doc = "`read()` method returns [gmac_txlpi::R](gmac_txlpi::R) reader structure"]
impl crate::Readable for GMAC_TXLPI {}
#[doc = "Transmit LPI Transitions"]
pub mod gmac_txlpi;
#[doc = "Transmit LPI Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_txlpitime](gmac_txlpitime) module"]
pub type GMAC_TXLPITIME = crate::Reg<u32, _GMAC_TXLPITIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TXLPITIME;
#[doc = "`read()` method returns [gmac_txlpitime::R](gmac_txlpitime::R) reader structure"]
impl crate::Readable for GMAC_TXLPITIME {}
#[doc = "Transmit LPI Time"]
pub mod gmac_txlpitime;
#[doc = "Interrupt Status Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_isrpq](gmac_isrpq) module"]
pub type GMAC_ISRPQ = crate::Reg<u32, _GMAC_ISRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ISRPQ;
#[doc = "`read()` method returns [gmac_isrpq::R](gmac_isrpq::R) reader structure"]
impl crate::Readable for GMAC_ISRPQ {}
#[doc = "Interrupt Status Register Priority Queue (1..5)"]
pub mod gmac_isrpq;
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tbqbapq](gmac_tbqbapq) module"]
pub type GMAC_TBQBAPQ = crate::Reg<u32, _GMAC_TBQBAPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_TBQBAPQ;
#[doc = "`read()` method returns [gmac_tbqbapq::R](gmac_tbqbapq::R) reader structure"]
impl crate::Readable for GMAC_TBQBAPQ {}
#[doc = "`write(|w| ..)` method takes [gmac_tbqbapq::W](gmac_tbqbapq::W) writer structure"]
impl crate::Writable for GMAC_TBQBAPQ {}
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod gmac_tbqbapq;
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rbqbapq](gmac_rbqbapq) module"]
pub type GMAC_RBQBAPQ = crate::Reg<u32, _GMAC_RBQBAPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RBQBAPQ;
#[doc = "`read()` method returns [gmac_rbqbapq::R](gmac_rbqbapq::R) reader structure"]
impl crate::Readable for GMAC_RBQBAPQ {}
#[doc = "`write(|w| ..)` method takes [gmac_rbqbapq::W](gmac_rbqbapq::W) writer structure"]
impl crate::Writable for GMAC_RBQBAPQ {}
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod gmac_rbqbapq;
#[doc = "Receive Buffer Size Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rbsrpq](gmac_rbsrpq) module"]
pub type GMAC_RBSRPQ = crate::Reg<u32, _GMAC_RBSRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_RBSRPQ;
#[doc = "`read()` method returns [gmac_rbsrpq::R](gmac_rbsrpq::R) reader structure"]
impl crate::Readable for GMAC_RBSRPQ {}
#[doc = "`write(|w| ..)` method takes [gmac_rbsrpq::W](gmac_rbsrpq::W) writer structure"]
impl crate::Writable for GMAC_RBSRPQ {}
#[doc = "Receive Buffer Size Register Priority Queue (1..5)"]
pub mod gmac_rbsrpq;
#[doc = "Credit-Based Shaping Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_cbscr](gmac_cbscr) module"]
pub type GMAC_CBSCR = crate::Reg<u32, _GMAC_CBSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_CBSCR;
#[doc = "`read()` method returns [gmac_cbscr::R](gmac_cbscr::R) reader structure"]
impl crate::Readable for GMAC_CBSCR {}
#[doc = "`write(|w| ..)` method takes [gmac_cbscr::W](gmac_cbscr::W) writer structure"]
impl crate::Writable for GMAC_CBSCR {}
#[doc = "Credit-Based Shaping Control Register"]
pub mod gmac_cbscr;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_cbsisqa](gmac_cbsisqa) module"]
pub type GMAC_CBSISQA = crate::Reg<u32, _GMAC_CBSISQA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_CBSISQA;
#[doc = "`read()` method returns [gmac_cbsisqa::R](gmac_cbsisqa::R) reader structure"]
impl crate::Readable for GMAC_CBSISQA {}
#[doc = "`write(|w| ..)` method takes [gmac_cbsisqa::W](gmac_cbsisqa::W) writer structure"]
impl crate::Writable for GMAC_CBSISQA {}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A"]
pub mod gmac_cbsisqa;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_cbsisqb](gmac_cbsisqb) module"]
pub type GMAC_CBSISQB = crate::Reg<u32, _GMAC_CBSISQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_CBSISQB;
#[doc = "`read()` method returns [gmac_cbsisqb::R](gmac_cbsisqb::R) reader structure"]
impl crate::Readable for GMAC_CBSISQB {}
#[doc = "`write(|w| ..)` method takes [gmac_cbsisqb::W](gmac_cbsisqb::W) writer structure"]
impl crate::Writable for GMAC_CBSISQB {}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B"]
pub mod gmac_cbsisqb;
#[doc = "Screening Type 1 Register Priority Queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_st1rpq](gmac_st1rpq) module"]
pub type GMAC_ST1RPQ = crate::Reg<u32, _GMAC_ST1RPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ST1RPQ;
#[doc = "`read()` method returns [gmac_st1rpq::R](gmac_st1rpq::R) reader structure"]
impl crate::Readable for GMAC_ST1RPQ {}
#[doc = "`write(|w| ..)` method takes [gmac_st1rpq::W](gmac_st1rpq::W) writer structure"]
impl crate::Writable for GMAC_ST1RPQ {}
#[doc = "Screening Type 1 Register Priority Queue"]
pub mod gmac_st1rpq;
#[doc = "Screening Type 2 Register Priority Queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_st2rpq](gmac_st2rpq) module"]
pub type GMAC_ST2RPQ = crate::Reg<u32, _GMAC_ST2RPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ST2RPQ;
#[doc = "`read()` method returns [gmac_st2rpq::R](gmac_st2rpq::R) reader structure"]
impl crate::Readable for GMAC_ST2RPQ {}
#[doc = "`write(|w| ..)` method takes [gmac_st2rpq::W](gmac_st2rpq::W) writer structure"]
impl crate::Writable for GMAC_ST2RPQ {}
#[doc = "Screening Type 2 Register Priority Queue"]
pub mod gmac_st2rpq;
#[doc = "Interrupt Enable Register Priority Queue (1..5)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ierpq](gmac_ierpq) module"]
pub type GMAC_IERPQ = crate::Reg<u32, _GMAC_IERPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IERPQ;
#[doc = "`write(|w| ..)` method takes [gmac_ierpq::W](gmac_ierpq::W) writer structure"]
impl crate::Writable for GMAC_IERPQ {}
#[doc = "Interrupt Enable Register Priority Queue (1..5)"]
pub mod gmac_ierpq;
#[doc = "Interrupt Disable Register Priority Queue (1..5)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_idrpq](gmac_idrpq) module"]
pub type GMAC_IDRPQ = crate::Reg<u32, _GMAC_IDRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IDRPQ;
#[doc = "`write(|w| ..)` method takes [gmac_idrpq::W](gmac_idrpq::W) writer structure"]
impl crate::Writable for GMAC_IDRPQ {}
#[doc = "Interrupt Disable Register Priority Queue (1..5)"]
pub mod gmac_idrpq;
#[doc = "Interrupt Mask Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_imrpq](gmac_imrpq) module"]
pub type GMAC_IMRPQ = crate::Reg<u32, _GMAC_IMRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_IMRPQ;
#[doc = "`read()` method returns [gmac_imrpq::R](gmac_imrpq::R) reader structure"]
impl crate::Readable for GMAC_IMRPQ {}
#[doc = "`write(|w| ..)` method takes [gmac_imrpq::W](gmac_imrpq::W) writer structure"]
impl crate::Writable for GMAC_IMRPQ {}
#[doc = "Interrupt Mask Register Priority Queue (1..5)"]
pub mod gmac_imrpq;
#[doc = "Screening Type 2 Ethertype Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_st2er](gmac_st2er) module"]
pub type GMAC_ST2ER = crate::Reg<u32, _GMAC_ST2ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_ST2ER;
#[doc = "`read()` method returns [gmac_st2er::R](gmac_st2er::R) reader structure"]
impl crate::Readable for GMAC_ST2ER {}
#[doc = "`write(|w| ..)` method takes [gmac_st2er::W](gmac_st2er::W) writer structure"]
impl crate::Writable for GMAC_ST2ER {}
#[doc = "Screening Type 2 Ethertype Register"]
pub mod gmac_st2er;
