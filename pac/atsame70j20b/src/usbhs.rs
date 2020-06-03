#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub usbhs_devctrl: USBHS_DEVCTRL,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub usbhs_devisr: USBHS_DEVISR,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub usbhs_devicr: USBHS_DEVICR,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub usbhs_devifr: USBHS_DEVIFR,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub usbhs_devimr: USBHS_DEVIMR,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub usbhs_devidr: USBHS_DEVIDR,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub usbhs_devier: USBHS_DEVIER,
    #[doc = "0x1c - Device Endpoint Register"]
    pub usbhs_devept: USBHS_DEVEPT,
    #[doc = "0x20 - Device Frame Number Register"]
    pub usbhs_devfnum: USBHS_DEVFNUM,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - Device Endpoint Configuration Register"]
    pub usbhs_deveptcfg: [USBHS_DEVEPTCFG; 10],
    _reserved10: [u8; 8usize],
    _reserved_10_usbhs_deveptisr: [u8; 40usize],
    _reserved11: [u8; 8usize],
    _reserved_11_usbhs_devepticr: [u8; 40usize],
    _reserved12: [u8; 8usize],
    _reserved_12_usbhs_deveptifr: [u8; 40usize],
    _reserved13: [u8; 8usize],
    _reserved_13_usbhs_deveptimr: [u8; 40usize],
    _reserved14: [u8; 8usize],
    _reserved_14_usbhs_deveptier: [u8; 40usize],
    _reserved15: [u8; 8usize],
    _reserved_15_usbhs_deveptidr: [u8; 40usize],
    _reserved16: [u8; 200usize],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved17: [u8; 128usize],
    #[doc = "0x400 - Host General Control Register"]
    pub usbhs_hstctrl: USBHS_HSTCTRL,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub usbhs_hstisr: USBHS_HSTISR,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub usbhs_hsticr: USBHS_HSTICR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub usbhs_hstifr: USBHS_HSTIFR,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub usbhs_hstimr: USBHS_HSTIMR,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub usbhs_hstidr: USBHS_HSTIDR,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub usbhs_hstier: USBHS_HSTIER,
    #[doc = "0x41c - Host Pipe Register"]
    pub usbhs_hstpip: USBHS_HSTPIP,
    #[doc = "0x420 - Host Frame Number Register"]
    pub usbhs_hstfnum: USBHS_HSTFNUM,
    #[doc = "0x424 - Host Address 1 Register"]
    pub usbhs_hstaddr1: USBHS_HSTADDR1,
    #[doc = "0x428 - Host Address 2 Register"]
    pub usbhs_hstaddr2: USBHS_HSTADDR2,
    #[doc = "0x42c - Host Address 3 Register"]
    pub usbhs_hstaddr3: USBHS_HSTADDR3,
    _reserved29: [u8; 208usize],
    _reserved_29_usbhs_: [u8; 40usize],
    _reserved30: [u8; 8usize],
    _reserved_30_usbhs_hstpipisr: [u8; 40usize],
    _reserved31: [u8; 8usize],
    _reserved_31_usbhs_hstpipicr: [u8; 40usize],
    _reserved32: [u8; 8usize],
    _reserved_32_usbhs_hstpipifr: [u8; 40usize],
    _reserved33: [u8; 8usize],
    _reserved_33_usbhs_hstpipimr: [u8; 40usize],
    _reserved34: [u8; 8usize],
    _reserved_34_usbhs_hstpipier: [u8; 40usize],
    _reserved35: [u8; 8usize],
    _reserved_35_usbhs_hstpipidr: [u8; 40usize],
    _reserved36: [u8; 8usize],
    #[doc = "0x650 - Host Pipe IN Request Register"]
    pub usbhs_hstpipinrq: [USBHS_HSTPIPINRQ; 10],
    _reserved37: [u8; 8usize],
    #[doc = "0x680 - Host Pipe Error Register"]
    pub usbhs_hstpiperr: [USBHS_HSTPIPERR; 10],
    _reserved38: [u8; 104usize],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved39: [u8; 128usize],
    #[doc = "0x800 - General Control Register"]
    pub usbhs_ctrl: USBHS_CTRL,
    #[doc = "0x804 - General Status Register"]
    pub usbhs_sr: USBHS_SR,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbhs_scr: USBHS_SCR,
    #[doc = "0x80c - General Status Set Register"]
    pub usbhs_sfr: USBHS_SFR,
}
impl RegisterBlock {
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_intrpt_mode(&self) -> &[USBHS_DEVEPTISR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [USBHS_DEVEPTISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_intrpt_mode_mut(&self) -> &mut [USBHS_DEVEPTISR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [USBHS_DEVEPTISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_blk_mode(&self) -> &[USBHS_DEVEPTISR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [USBHS_DEVEPTISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_blk_mode_mut(&self) -> &mut [USBHS_DEVEPTISR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [USBHS_DEVEPTISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_iso_mode(&self) -> &[USBHS_DEVEPTISR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [USBHS_DEVEPTISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_iso_mode_mut(&self) -> &mut [USBHS_DEVEPTISR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [USBHS_DEVEPTISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_ctrl_mode(&self) -> &[USBHS_DEVEPTISR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [USBHS_DEVEPTISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_ctrl_mode_mut(&self) -> &mut [USBHS_DEVEPTISR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [USBHS_DEVEPTISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_intrpt_mode(&self) -> &[USBHS_DEVEPTICR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [USBHS_DEVEPTICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_intrpt_mode_mut(&self) -> &mut [USBHS_DEVEPTICR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [USBHS_DEVEPTICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_blk_mode(&self) -> &[USBHS_DEVEPTICR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [USBHS_DEVEPTICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_blk_mode_mut(&self) -> &mut [USBHS_DEVEPTICR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [USBHS_DEVEPTICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_iso_mode(&self) -> &[USBHS_DEVEPTICR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [USBHS_DEVEPTICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_iso_mode_mut(&self) -> &mut [USBHS_DEVEPTICR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [USBHS_DEVEPTICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_ctrl_mode(&self) -> &[USBHS_DEVEPTICR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [USBHS_DEVEPTICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_ctrl_mode_mut(&self) -> &mut [USBHS_DEVEPTICR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [USBHS_DEVEPTICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_intrpt_mode(&self) -> &[USBHS_DEVEPTIFR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [USBHS_DEVEPTIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_intrpt_mode_mut(&self) -> &mut [USBHS_DEVEPTIFR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [USBHS_DEVEPTIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_blk_mode(&self) -> &[USBHS_DEVEPTIFR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [USBHS_DEVEPTIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_blk_mode_mut(&self) -> &mut [USBHS_DEVEPTIFR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [USBHS_DEVEPTIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_iso_mode(&self) -> &[USBHS_DEVEPTIFR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [USBHS_DEVEPTIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_iso_mode_mut(&self) -> &mut [USBHS_DEVEPTIFR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [USBHS_DEVEPTIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_ctrl_mode(&self) -> &[USBHS_DEVEPTIFR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [USBHS_DEVEPTIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_ctrl_mode_mut(&self) -> &mut [USBHS_DEVEPTIFR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [USBHS_DEVEPTIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_intrpt_mode(&self) -> &[USBHS_DEVEPTIMR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [USBHS_DEVEPTIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_intrpt_mode_mut(&self) -> &mut [USBHS_DEVEPTIMR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [USBHS_DEVEPTIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_blk_mode(&self) -> &[USBHS_DEVEPTIMR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [USBHS_DEVEPTIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_blk_mode_mut(&self) -> &mut [USBHS_DEVEPTIMR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [USBHS_DEVEPTIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_iso_mode(&self) -> &[USBHS_DEVEPTIMR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [USBHS_DEVEPTIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_iso_mode_mut(&self) -> &mut [USBHS_DEVEPTIMR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [USBHS_DEVEPTIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_ctrl_mode(&self) -> &[USBHS_DEVEPTIMR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [USBHS_DEVEPTIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_ctrl_mode_mut(&self) -> &mut [USBHS_DEVEPTIMR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [USBHS_DEVEPTIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_intrpt_mode(&self) -> &[USBHS_DEVEPTIER_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [USBHS_DEVEPTIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_intrpt_mode_mut(&self) -> &mut [USBHS_DEVEPTIER_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [USBHS_DEVEPTIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_blk_mode(&self) -> &[USBHS_DEVEPTIER_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [USBHS_DEVEPTIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_blk_mode_mut(&self) -> &mut [USBHS_DEVEPTIER_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [USBHS_DEVEPTIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_iso_mode(&self) -> &[USBHS_DEVEPTIER_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [USBHS_DEVEPTIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_iso_mode_mut(&self) -> &mut [USBHS_DEVEPTIER_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [USBHS_DEVEPTIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_ctrl_mode(&self) -> &[USBHS_DEVEPTIER_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [USBHS_DEVEPTIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_ctrl_mode_mut(&self) -> &mut [USBHS_DEVEPTIER_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [USBHS_DEVEPTIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_intrpt_mode(&self) -> &[USBHS_DEVEPTIDR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [USBHS_DEVEPTIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_intrpt_mode_mut(&self) -> &mut [USBHS_DEVEPTIDR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [USBHS_DEVEPTIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_blk_mode(&self) -> &[USBHS_DEVEPTIDR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [USBHS_DEVEPTIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_blk_mode_mut(&self) -> &mut [USBHS_DEVEPTIDR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [USBHS_DEVEPTIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_iso_mode(&self) -> &[USBHS_DEVEPTIDR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [USBHS_DEVEPTIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_iso_mode_mut(&self) -> &mut [USBHS_DEVEPTIDR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [USBHS_DEVEPTIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_ctrl_mode(&self) -> &[USBHS_DEVEPTIDR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [USBHS_DEVEPTIDR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_ctrl_mode_mut(&self) -> &mut [USBHS_DEVEPTIDR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [USBHS_DEVEPTIDR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn usbhs_hstpipcfg_ctrl_bulk_mode(&self) -> &[USBHS_HSTPIPCFG_CTRL_BULK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const [USBHS_HSTPIPCFG_CTRL_BULK_MODE; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn usbhs_hstpipcfg_ctrl_bulk_mode_mut(&self) -> &mut [USBHS_HSTPIPCFG_CTRL_BULK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1280usize)
                as *mut [USBHS_HSTPIPCFG_CTRL_BULK_MODE; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn usbhs_hstpipcfg(&self) -> &[USBHS_HSTPIPCFG; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize) as *const [USBHS_HSTPIPCFG; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn usbhs_hstpipcfg_mut(&self) -> &mut [USBHS_HSTPIPCFG; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1280usize) as *mut [USBHS_HSTPIPCFG; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_intrpt_mode(&self) -> &[USBHS_HSTPIPISR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [USBHS_HSTPIPISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_intrpt_mode_mut(&self) -> &mut [USBHS_HSTPIPISR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [USBHS_HSTPIPISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_blk_mode(&self) -> &[USBHS_HSTPIPISR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [USBHS_HSTPIPISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_blk_mode_mut(&self) -> &mut [USBHS_HSTPIPISR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [USBHS_HSTPIPISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_iso_mode(&self) -> &[USBHS_HSTPIPISR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [USBHS_HSTPIPISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_iso_mode_mut(&self) -> &mut [USBHS_HSTPIPISR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [USBHS_HSTPIPISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_ctrl_mode(&self) -> &[USBHS_HSTPIPISR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [USBHS_HSTPIPISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_ctrl_mode_mut(&self) -> &mut [USBHS_HSTPIPISR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [USBHS_HSTPIPISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_intrpt_mode(&self) -> &[USBHS_HSTPIPICR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [USBHS_HSTPIPICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_intrpt_mode_mut(&self) -> &mut [USBHS_HSTPIPICR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [USBHS_HSTPIPICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_blk_mode(&self) -> &[USBHS_HSTPIPICR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [USBHS_HSTPIPICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_blk_mode_mut(&self) -> &mut [USBHS_HSTPIPICR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [USBHS_HSTPIPICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_iso_mode(&self) -> &[USBHS_HSTPIPICR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [USBHS_HSTPIPICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_iso_mode_mut(&self) -> &mut [USBHS_HSTPIPICR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [USBHS_HSTPIPICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_ctrl_mode(&self) -> &[USBHS_HSTPIPICR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [USBHS_HSTPIPICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_ctrl_mode_mut(&self) -> &mut [USBHS_HSTPIPICR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [USBHS_HSTPIPICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_intrpt_mode(&self) -> &[USBHS_HSTPIPIFR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [USBHS_HSTPIPIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_intrpt_mode_mut(&self) -> &mut [USBHS_HSTPIPIFR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [USBHS_HSTPIPIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_blk_mode(&self) -> &[USBHS_HSTPIPIFR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [USBHS_HSTPIPIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_blk_mode_mut(&self) -> &mut [USBHS_HSTPIPIFR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [USBHS_HSTPIPIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_iso_mode(&self) -> &[USBHS_HSTPIPIFR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [USBHS_HSTPIPIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_iso_mode_mut(&self) -> &mut [USBHS_HSTPIPIFR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [USBHS_HSTPIPIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_ctrl_mode(&self) -> &[USBHS_HSTPIPIFR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [USBHS_HSTPIPIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_ctrl_mode_mut(&self) -> &mut [USBHS_HSTPIPIFR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [USBHS_HSTPIPIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_intrpt_mode(&self) -> &[USBHS_HSTPIPIMR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [USBHS_HSTPIPIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_intrpt_mode_mut(&self) -> &mut [USBHS_HSTPIPIMR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [USBHS_HSTPIPIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_blk_mode(&self) -> &[USBHS_HSTPIPIMR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [USBHS_HSTPIPIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_blk_mode_mut(&self) -> &mut [USBHS_HSTPIPIMR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [USBHS_HSTPIPIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_iso_mode(&self) -> &[USBHS_HSTPIPIMR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [USBHS_HSTPIPIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_iso_mode_mut(&self) -> &mut [USBHS_HSTPIPIMR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [USBHS_HSTPIPIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_ctrl_mode(&self) -> &[USBHS_HSTPIPIMR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [USBHS_HSTPIPIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_ctrl_mode_mut(&self) -> &mut [USBHS_HSTPIPIMR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [USBHS_HSTPIPIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_intrpt_mode(&self) -> &[USBHS_HSTPIPIER_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [USBHS_HSTPIPIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_intrpt_mode_mut(&self) -> &mut [USBHS_HSTPIPIER_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [USBHS_HSTPIPIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_blk_mode(&self) -> &[USBHS_HSTPIPIER_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [USBHS_HSTPIPIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_blk_mode_mut(&self) -> &mut [USBHS_HSTPIPIER_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [USBHS_HSTPIPIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_iso_mode(&self) -> &[USBHS_HSTPIPIER_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [USBHS_HSTPIPIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_iso_mode_mut(&self) -> &mut [USBHS_HSTPIPIER_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [USBHS_HSTPIPIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_ctrl_mode(&self) -> &[USBHS_HSTPIPIER_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [USBHS_HSTPIPIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_ctrl_mode_mut(&self) -> &mut [USBHS_HSTPIPIER_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [USBHS_HSTPIPIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_intrpt_mode(&self) -> &[USBHS_HSTPIPIDR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [USBHS_HSTPIPIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_intrpt_mode_mut(&self) -> &mut [USBHS_HSTPIPIDR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [USBHS_HSTPIPIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_blk_mode(&self) -> &[USBHS_HSTPIPIDR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [USBHS_HSTPIPIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_blk_mode_mut(&self) -> &mut [USBHS_HSTPIPIDR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [USBHS_HSTPIPIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_iso_mode(&self) -> &[USBHS_HSTPIPIDR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [USBHS_HSTPIPIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_iso_mode_mut(&self) -> &mut [USBHS_HSTPIPIDR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [USBHS_HSTPIPIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_ctrl_mode(&self) -> &[USBHS_HSTPIPIDR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [USBHS_HSTPIPIDR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_ctrl_mode_mut(&self) -> &mut [USBHS_HSTPIPIDR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [USBHS_HSTPIPIDR_CTRL_MODE; 10])
        }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register"]
    pub usbhs_devdmanxtdsc: self::usbhs_devdma::USBHS_DEVDMANXTDSC,
    #[doc = "0x04 - Device DMA Channel Address Register"]
    pub usbhs_devdmaaddress: self::usbhs_devdma::USBHS_DEVDMAADDRESS,
    #[doc = "0x08 - Device DMA Channel Control Register"]
    pub usbhs_devdmacontrol: self::usbhs_devdma::USBHS_DEVDMACONTROL,
    #[doc = "0x0c - Device DMA Channel Status Register"]
    pub usbhs_devdmastatus: self::usbhs_devdma::USBHS_DEVDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod usbhs_devdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register"]
    pub usbhs_hstdmanxtdsc: self::usbhs_hstdma::USBHS_HSTDMANXTDSC,
    #[doc = "0x04 - Host DMA Channel Address Register"]
    pub usbhs_hstdmaaddress: self::usbhs_hstdma::USBHS_HSTDMAADDRESS,
    #[doc = "0x08 - Host DMA Channel Control Register"]
    pub usbhs_hstdmacontrol: self::usbhs_hstdma::USBHS_HSTDMACONTROL,
    #[doc = "0x0c - Host DMA Channel Status Register"]
    pub usbhs_hstdmastatus: self::usbhs_hstdma::USBHS_HSTDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub mod usbhs_hstdma;
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devctrl](usbhs_devctrl) module"]
pub type USBHS_DEVCTRL = crate::Reg<u32, _USBHS_DEVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVCTRL;
#[doc = "`read()` method returns [usbhs_devctrl::R](usbhs_devctrl::R) reader structure"]
impl crate::Readable for USBHS_DEVCTRL {}
#[doc = "`write(|w| ..)` method takes [usbhs_devctrl::W](usbhs_devctrl::W) writer structure"]
impl crate::Writable for USBHS_DEVCTRL {}
#[doc = "Device General Control Register"]
pub mod usbhs_devctrl;
#[doc = "Device Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devisr](usbhs_devisr) module"]
pub type USBHS_DEVISR = crate::Reg<u32, _USBHS_DEVISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVISR;
#[doc = "`read()` method returns [usbhs_devisr::R](usbhs_devisr::R) reader structure"]
impl crate::Readable for USBHS_DEVISR {}
#[doc = "Device Global Interrupt Status Register"]
pub mod usbhs_devisr;
#[doc = "Device Global Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devicr](usbhs_devicr) module"]
pub type USBHS_DEVICR = crate::Reg<u32, _USBHS_DEVICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVICR;
#[doc = "`write(|w| ..)` method takes [usbhs_devicr::W](usbhs_devicr::W) writer structure"]
impl crate::Writable for USBHS_DEVICR {}
#[doc = "Device Global Interrupt Clear Register"]
pub mod usbhs_devicr;
#[doc = "Device Global Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devifr](usbhs_devifr) module"]
pub type USBHS_DEVIFR = crate::Reg<u32, _USBHS_DEVIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIFR;
#[doc = "`write(|w| ..)` method takes [usbhs_devifr::W](usbhs_devifr::W) writer structure"]
impl crate::Writable for USBHS_DEVIFR {}
#[doc = "Device Global Interrupt Set Register"]
pub mod usbhs_devifr;
#[doc = "Device Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devimr](usbhs_devimr) module"]
pub type USBHS_DEVIMR = crate::Reg<u32, _USBHS_DEVIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIMR;
#[doc = "`read()` method returns [usbhs_devimr::R](usbhs_devimr::R) reader structure"]
impl crate::Readable for USBHS_DEVIMR {}
#[doc = "Device Global Interrupt Mask Register"]
pub mod usbhs_devimr;
#[doc = "Device Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devidr](usbhs_devidr) module"]
pub type USBHS_DEVIDR = crate::Reg<u32, _USBHS_DEVIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIDR;
#[doc = "`write(|w| ..)` method takes [usbhs_devidr::W](usbhs_devidr::W) writer structure"]
impl crate::Writable for USBHS_DEVIDR {}
#[doc = "Device Global Interrupt Disable Register"]
pub mod usbhs_devidr;
#[doc = "Device Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devier](usbhs_devier) module"]
pub type USBHS_DEVIER = crate::Reg<u32, _USBHS_DEVIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIER;
#[doc = "`write(|w| ..)` method takes [usbhs_devier::W](usbhs_devier::W) writer structure"]
impl crate::Writable for USBHS_DEVIER {}
#[doc = "Device Global Interrupt Enable Register"]
pub mod usbhs_devier;
#[doc = "Device Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devept](usbhs_devept) module"]
pub type USBHS_DEVEPT = crate::Reg<u32, _USBHS_DEVEPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPT;
#[doc = "`read()` method returns [usbhs_devept::R](usbhs_devept::R) reader structure"]
impl crate::Readable for USBHS_DEVEPT {}
#[doc = "`write(|w| ..)` method takes [usbhs_devept::W](usbhs_devept::W) writer structure"]
impl crate::Writable for USBHS_DEVEPT {}
#[doc = "Device Endpoint Register"]
pub mod usbhs_devept;
#[doc = "Device Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devfnum](usbhs_devfnum) module"]
pub type USBHS_DEVFNUM = crate::Reg<u32, _USBHS_DEVFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVFNUM;
#[doc = "`read()` method returns [usbhs_devfnum::R](usbhs_devfnum::R) reader structure"]
impl crate::Readable for USBHS_DEVFNUM {}
#[doc = "Device Frame Number Register"]
pub mod usbhs_devfnum;
#[doc = "Device Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptcfg](usbhs_deveptcfg) module"]
pub type USBHS_DEVEPTCFG = crate::Reg<u32, _USBHS_DEVEPTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTCFG;
#[doc = "`read()` method returns [usbhs_deveptcfg::R](usbhs_deveptcfg::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTCFG {}
#[doc = "`write(|w| ..)` method takes [usbhs_deveptcfg::W](usbhs_deveptcfg::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTCFG {}
#[doc = "Device Endpoint Configuration Register"]
pub mod usbhs_deveptcfg;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptisr_ctrl_mode](usbhs_deveptisr_ctrl_mode) module"]
pub type USBHS_DEVEPTISR_CTRL_MODE = crate::Reg<u32, _USBHS_DEVEPTISR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTISR_CTRL_MODE;
#[doc = "`read()` method returns [usbhs_deveptisr_ctrl_mode::R](usbhs_deveptisr_ctrl_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTISR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptisr_iso_mode](usbhs_deveptisr_iso_mode) module"]
pub type USBHS_DEVEPTISR_ISO_MODE = crate::Reg<u32, _USBHS_DEVEPTISR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTISR_ISO_MODE;
#[doc = "`read()` method returns [usbhs_deveptisr_iso_mode::R](usbhs_deveptisr_iso_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTISR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_iso_mode;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptisr_blk_mode](usbhs_deveptisr_blk_mode) module"]
pub type USBHS_DEVEPTISR_BLK_MODE = crate::Reg<u32, _USBHS_DEVEPTISR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTISR_BLK_MODE;
#[doc = "`read()` method returns [usbhs_deveptisr_blk_mode::R](usbhs_deveptisr_blk_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTISR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_blk_mode;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptisr_intrpt_mode](usbhs_deveptisr_intrpt_mode) module"]
pub type USBHS_DEVEPTISR_INTRPT_MODE = crate::Reg<u32, _USBHS_DEVEPTISR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTISR_INTRPT_MODE;
#[doc = "`read()` method returns [usbhs_deveptisr_intrpt_mode::R](usbhs_deveptisr_intrpt_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTISR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devepticr_ctrl_mode](usbhs_devepticr_ctrl_mode) module"]
pub type USBHS_DEVEPTICR_CTRL_MODE = crate::Reg<u32, _USBHS_DEVEPTICR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTICR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_devepticr_ctrl_mode::W](usbhs_devepticr_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTICR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devepticr_iso_mode](usbhs_devepticr_iso_mode) module"]
pub type USBHS_DEVEPTICR_ISO_MODE = crate::Reg<u32, _USBHS_DEVEPTICR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTICR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_devepticr_iso_mode::W](usbhs_devepticr_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTICR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_iso_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devepticr_blk_mode](usbhs_devepticr_blk_mode) module"]
pub type USBHS_DEVEPTICR_BLK_MODE = crate::Reg<u32, _USBHS_DEVEPTICR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTICR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_devepticr_blk_mode::W](usbhs_devepticr_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTICR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_blk_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devepticr_intrpt_mode](usbhs_devepticr_intrpt_mode) module"]
pub type USBHS_DEVEPTICR_INTRPT_MODE = crate::Reg<u32, _USBHS_DEVEPTICR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTICR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_devepticr_intrpt_mode::W](usbhs_devepticr_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTICR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptifr_ctrl_mode](usbhs_deveptifr_ctrl_mode) module"]
pub type USBHS_DEVEPTIFR_CTRL_MODE = crate::Reg<u32, _USBHS_DEVEPTIFR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIFR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptifr_ctrl_mode::W](usbhs_deveptifr_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIFR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptifr_iso_mode](usbhs_deveptifr_iso_mode) module"]
pub type USBHS_DEVEPTIFR_ISO_MODE = crate::Reg<u32, _USBHS_DEVEPTIFR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIFR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptifr_iso_mode::W](usbhs_deveptifr_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIFR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_iso_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptifr_blk_mode](usbhs_deveptifr_blk_mode) module"]
pub type USBHS_DEVEPTIFR_BLK_MODE = crate::Reg<u32, _USBHS_DEVEPTIFR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIFR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptifr_blk_mode::W](usbhs_deveptifr_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIFR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_blk_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptifr_intrpt_mode](usbhs_deveptifr_intrpt_mode) module"]
pub type USBHS_DEVEPTIFR_INTRPT_MODE = crate::Reg<u32, _USBHS_DEVEPTIFR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIFR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptifr_intrpt_mode::W](usbhs_deveptifr_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIFR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptimr_ctrl_mode](usbhs_deveptimr_ctrl_mode) module"]
pub type USBHS_DEVEPTIMR_CTRL_MODE = crate::Reg<u32, _USBHS_DEVEPTIMR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIMR_CTRL_MODE;
#[doc = "`read()` method returns [usbhs_deveptimr_ctrl_mode::R](usbhs_deveptimr_ctrl_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTIMR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptimr_iso_mode](usbhs_deveptimr_iso_mode) module"]
pub type USBHS_DEVEPTIMR_ISO_MODE = crate::Reg<u32, _USBHS_DEVEPTIMR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIMR_ISO_MODE;
#[doc = "`read()` method returns [usbhs_deveptimr_iso_mode::R](usbhs_deveptimr_iso_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTIMR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_iso_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptimr_blk_mode](usbhs_deveptimr_blk_mode) module"]
pub type USBHS_DEVEPTIMR_BLK_MODE = crate::Reg<u32, _USBHS_DEVEPTIMR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIMR_BLK_MODE;
#[doc = "`read()` method returns [usbhs_deveptimr_blk_mode::R](usbhs_deveptimr_blk_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTIMR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_blk_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptimr_intrpt_mode](usbhs_deveptimr_intrpt_mode) module"]
pub type USBHS_DEVEPTIMR_INTRPT_MODE = crate::Reg<u32, _USBHS_DEVEPTIMR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIMR_INTRPT_MODE;
#[doc = "`read()` method returns [usbhs_deveptimr_intrpt_mode::R](usbhs_deveptimr_intrpt_mode::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTIMR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptier_ctrl_mode](usbhs_deveptier_ctrl_mode) module"]
pub type USBHS_DEVEPTIER_CTRL_MODE = crate::Reg<u32, _USBHS_DEVEPTIER_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIER_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptier_ctrl_mode::W](usbhs_deveptier_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIER_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_ctrl_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptier_iso_mode](usbhs_deveptier_iso_mode) module"]
pub type USBHS_DEVEPTIER_ISO_MODE = crate::Reg<u32, _USBHS_DEVEPTIER_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIER_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptier_iso_mode::W](usbhs_deveptier_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIER_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_iso_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptier_blk_mode](usbhs_deveptier_blk_mode) module"]
pub type USBHS_DEVEPTIER_BLK_MODE = crate::Reg<u32, _USBHS_DEVEPTIER_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIER_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptier_blk_mode::W](usbhs_deveptier_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIER_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_blk_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptier_intrpt_mode](usbhs_deveptier_intrpt_mode) module"]
pub type USBHS_DEVEPTIER_INTRPT_MODE = crate::Reg<u32, _USBHS_DEVEPTIER_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIER_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptier_intrpt_mode::W](usbhs_deveptier_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIER_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_intrpt_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptidr_ctrl_mode](usbhs_deveptidr_ctrl_mode) module"]
pub type USBHS_DEVEPTIDR_CTRL_MODE = crate::Reg<u32, _USBHS_DEVEPTIDR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIDR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptidr_ctrl_mode::W](usbhs_deveptidr_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIDR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptidr_iso_mode](usbhs_deveptidr_iso_mode) module"]
pub type USBHS_DEVEPTIDR_ISO_MODE = crate::Reg<u32, _USBHS_DEVEPTIDR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIDR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptidr_iso_mode::W](usbhs_deveptidr_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIDR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_iso_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptidr_blk_mode](usbhs_deveptidr_blk_mode) module"]
pub type USBHS_DEVEPTIDR_BLK_MODE = crate::Reg<u32, _USBHS_DEVEPTIDR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIDR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptidr_blk_mode::W](usbhs_deveptidr_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIDR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_blk_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptidr_intrpt_mode](usbhs_deveptidr_intrpt_mode) module"]
pub type USBHS_DEVEPTIDR_INTRPT_MODE = crate::Reg<u32, _USBHS_DEVEPTIDR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIDR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptidr_intrpt_mode::W](usbhs_deveptidr_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIDR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_intrpt_mode;
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstctrl](usbhs_hstctrl) module"]
pub type USBHS_HSTCTRL = crate::Reg<u32, _USBHS_HSTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTCTRL;
#[doc = "`read()` method returns [usbhs_hstctrl::R](usbhs_hstctrl::R) reader structure"]
impl crate::Readable for USBHS_HSTCTRL {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstctrl::W](usbhs_hstctrl::W) writer structure"]
impl crate::Writable for USBHS_HSTCTRL {}
#[doc = "Host General Control Register"]
pub mod usbhs_hstctrl;
#[doc = "Host Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstisr](usbhs_hstisr) module"]
pub type USBHS_HSTISR = crate::Reg<u32, _USBHS_HSTISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTISR;
#[doc = "`read()` method returns [usbhs_hstisr::R](usbhs_hstisr::R) reader structure"]
impl crate::Readable for USBHS_HSTISR {}
#[doc = "Host Global Interrupt Status Register"]
pub mod usbhs_hstisr;
#[doc = "Host Global Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hsticr](usbhs_hsticr) module"]
pub type USBHS_HSTICR = crate::Reg<u32, _USBHS_HSTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTICR;
#[doc = "`write(|w| ..)` method takes [usbhs_hsticr::W](usbhs_hsticr::W) writer structure"]
impl crate::Writable for USBHS_HSTICR {}
#[doc = "Host Global Interrupt Clear Register"]
pub mod usbhs_hsticr;
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstifr](usbhs_hstifr) module"]
pub type USBHS_HSTIFR = crate::Reg<u32, _USBHS_HSTIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIFR;
#[doc = "`write(|w| ..)` method takes [usbhs_hstifr::W](usbhs_hstifr::W) writer structure"]
impl crate::Writable for USBHS_HSTIFR {}
#[doc = "Host Global Interrupt Set Register"]
pub mod usbhs_hstifr;
#[doc = "Host Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstimr](usbhs_hstimr) module"]
pub type USBHS_HSTIMR = crate::Reg<u32, _USBHS_HSTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIMR;
#[doc = "`read()` method returns [usbhs_hstimr::R](usbhs_hstimr::R) reader structure"]
impl crate::Readable for USBHS_HSTIMR {}
#[doc = "Host Global Interrupt Mask Register"]
pub mod usbhs_hstimr;
#[doc = "Host Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstidr](usbhs_hstidr) module"]
pub type USBHS_HSTIDR = crate::Reg<u32, _USBHS_HSTIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIDR;
#[doc = "`write(|w| ..)` method takes [usbhs_hstidr::W](usbhs_hstidr::W) writer structure"]
impl crate::Writable for USBHS_HSTIDR {}
#[doc = "Host Global Interrupt Disable Register"]
pub mod usbhs_hstidr;
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstier](usbhs_hstier) module"]
pub type USBHS_HSTIER = crate::Reg<u32, _USBHS_HSTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIER;
#[doc = "`write(|w| ..)` method takes [usbhs_hstier::W](usbhs_hstier::W) writer structure"]
impl crate::Writable for USBHS_HSTIER {}
#[doc = "Host Global Interrupt Enable Register"]
pub mod usbhs_hstier;
#[doc = "Host Pipe Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpip](usbhs_hstpip) module"]
pub type USBHS_HSTPIP = crate::Reg<u32, _USBHS_HSTPIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIP;
#[doc = "`read()` method returns [usbhs_hstpip::R](usbhs_hstpip::R) reader structure"]
impl crate::Readable for USBHS_HSTPIP {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpip::W](usbhs_hstpip::W) writer structure"]
impl crate::Writable for USBHS_HSTPIP {}
#[doc = "Host Pipe Register"]
pub mod usbhs_hstpip;
#[doc = "Host Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstfnum](usbhs_hstfnum) module"]
pub type USBHS_HSTFNUM = crate::Reg<u32, _USBHS_HSTFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTFNUM;
#[doc = "`read()` method returns [usbhs_hstfnum::R](usbhs_hstfnum::R) reader structure"]
impl crate::Readable for USBHS_HSTFNUM {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstfnum::W](usbhs_hstfnum::W) writer structure"]
impl crate::Writable for USBHS_HSTFNUM {}
#[doc = "Host Frame Number Register"]
pub mod usbhs_hstfnum;
#[doc = "Host Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr1](usbhs_hstaddr1) module"]
pub type USBHS_HSTADDR1 = crate::Reg<u32, _USBHS_HSTADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTADDR1;
#[doc = "`read()` method returns [usbhs_hstaddr1::R](usbhs_hstaddr1::R) reader structure"]
impl crate::Readable for USBHS_HSTADDR1 {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr1::W](usbhs_hstaddr1::W) writer structure"]
impl crate::Writable for USBHS_HSTADDR1 {}
#[doc = "Host Address 1 Register"]
pub mod usbhs_hstaddr1;
#[doc = "Host Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr2](usbhs_hstaddr2) module"]
pub type USBHS_HSTADDR2 = crate::Reg<u32, _USBHS_HSTADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTADDR2;
#[doc = "`read()` method returns [usbhs_hstaddr2::R](usbhs_hstaddr2::R) reader structure"]
impl crate::Readable for USBHS_HSTADDR2 {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr2::W](usbhs_hstaddr2::W) writer structure"]
impl crate::Writable for USBHS_HSTADDR2 {}
#[doc = "Host Address 2 Register"]
pub mod usbhs_hstaddr2;
#[doc = "Host Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr3](usbhs_hstaddr3) module"]
pub type USBHS_HSTADDR3 = crate::Reg<u32, _USBHS_HSTADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTADDR3;
#[doc = "`read()` method returns [usbhs_hstaddr3::R](usbhs_hstaddr3::R) reader structure"]
impl crate::Readable for USBHS_HSTADDR3 {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr3::W](usbhs_hstaddr3::W) writer structure"]
impl crate::Writable for USBHS_HSTADDR3 {}
#[doc = "Host Address 3 Register"]
pub mod usbhs_hstaddr3;
#[doc = "Host Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipcfg](usbhs_hstpipcfg) module"]
pub type USBHS_HSTPIPCFG = crate::Reg<u32, _USBHS_HSTPIPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPCFG;
#[doc = "`read()` method returns [usbhs_hstpipcfg::R](usbhs_hstpipcfg::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPCFG {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipcfg::W](usbhs_hstpipcfg::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPCFG {}
#[doc = "Host Pipe Configuration Register"]
pub mod usbhs_hstpipcfg;
#[doc = "Host Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipcfg_ctrl_bulk_mode](usbhs_hstpipcfg_ctrl_bulk_mode) module"]
pub type USBHS_HSTPIPCFG_CTRL_BULK_MODE = crate::Reg<u32, _USBHS_HSTPIPCFG_CTRL_BULK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPCFG_CTRL_BULK_MODE;
#[doc = "`read()` method returns [usbhs_hstpipcfg_ctrl_bulk_mode::R](usbhs_hstpipcfg_ctrl_bulk_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPCFG_CTRL_BULK_MODE {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipcfg_ctrl_bulk_mode::W](usbhs_hstpipcfg_ctrl_bulk_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPCFG_CTRL_BULK_MODE {}
#[doc = "Host Pipe Configuration Register"]
pub mod usbhs_hstpipcfg_ctrl_bulk_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipisr_ctrl_mode](usbhs_hstpipisr_ctrl_mode) module"]
pub type USBHS_HSTPIPISR_CTRL_MODE = crate::Reg<u32, _USBHS_HSTPIPISR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPISR_CTRL_MODE;
#[doc = "`read()` method returns [usbhs_hstpipisr_ctrl_mode::R](usbhs_hstpipisr_ctrl_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPISR_CTRL_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_ctrl_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipisr_iso_mode](usbhs_hstpipisr_iso_mode) module"]
pub type USBHS_HSTPIPISR_ISO_MODE = crate::Reg<u32, _USBHS_HSTPIPISR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPISR_ISO_MODE;
#[doc = "`read()` method returns [usbhs_hstpipisr_iso_mode::R](usbhs_hstpipisr_iso_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPISR_ISO_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_iso_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipisr_blk_mode](usbhs_hstpipisr_blk_mode) module"]
pub type USBHS_HSTPIPISR_BLK_MODE = crate::Reg<u32, _USBHS_HSTPIPISR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPISR_BLK_MODE;
#[doc = "`read()` method returns [usbhs_hstpipisr_blk_mode::R](usbhs_hstpipisr_blk_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPISR_BLK_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_blk_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipisr_intrpt_mode](usbhs_hstpipisr_intrpt_mode) module"]
pub type USBHS_HSTPIPISR_INTRPT_MODE = crate::Reg<u32, _USBHS_HSTPIPISR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPISR_INTRPT_MODE;
#[doc = "`read()` method returns [usbhs_hstpipisr_intrpt_mode::R](usbhs_hstpipisr_intrpt_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPISR_INTRPT_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_intrpt_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipicr_ctrl_mode](usbhs_hstpipicr_ctrl_mode) module"]
pub type USBHS_HSTPIPICR_CTRL_MODE = crate::Reg<u32, _USBHS_HSTPIPICR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPICR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipicr_ctrl_mode::W](usbhs_hstpipicr_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPICR_CTRL_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_ctrl_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipicr_iso_mode](usbhs_hstpipicr_iso_mode) module"]
pub type USBHS_HSTPIPICR_ISO_MODE = crate::Reg<u32, _USBHS_HSTPIPICR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPICR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipicr_iso_mode::W](usbhs_hstpipicr_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPICR_ISO_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_iso_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipicr_blk_mode](usbhs_hstpipicr_blk_mode) module"]
pub type USBHS_HSTPIPICR_BLK_MODE = crate::Reg<u32, _USBHS_HSTPIPICR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPICR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipicr_blk_mode::W](usbhs_hstpipicr_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPICR_BLK_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_blk_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipicr_intrpt_mode](usbhs_hstpipicr_intrpt_mode) module"]
pub type USBHS_HSTPIPICR_INTRPT_MODE = crate::Reg<u32, _USBHS_HSTPIPICR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPICR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipicr_intrpt_mode::W](usbhs_hstpipicr_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPICR_INTRPT_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_intrpt_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipifr_ctrl_mode](usbhs_hstpipifr_ctrl_mode) module"]
pub type USBHS_HSTPIPIFR_CTRL_MODE = crate::Reg<u32, _USBHS_HSTPIPIFR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIFR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipifr_ctrl_mode::W](usbhs_hstpipifr_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIFR_CTRL_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_ctrl_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipifr_iso_mode](usbhs_hstpipifr_iso_mode) module"]
pub type USBHS_HSTPIPIFR_ISO_MODE = crate::Reg<u32, _USBHS_HSTPIPIFR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIFR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipifr_iso_mode::W](usbhs_hstpipifr_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIFR_ISO_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_iso_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipifr_blk_mode](usbhs_hstpipifr_blk_mode) module"]
pub type USBHS_HSTPIPIFR_BLK_MODE = crate::Reg<u32, _USBHS_HSTPIPIFR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIFR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipifr_blk_mode::W](usbhs_hstpipifr_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIFR_BLK_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_blk_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipifr_intrpt_mode](usbhs_hstpipifr_intrpt_mode) module"]
pub type USBHS_HSTPIPIFR_INTRPT_MODE = crate::Reg<u32, _USBHS_HSTPIPIFR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIFR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipifr_intrpt_mode::W](usbhs_hstpipifr_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIFR_INTRPT_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_intrpt_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipimr_ctrl_mode](usbhs_hstpipimr_ctrl_mode) module"]
pub type USBHS_HSTPIPIMR_CTRL_MODE = crate::Reg<u32, _USBHS_HSTPIPIMR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIMR_CTRL_MODE;
#[doc = "`read()` method returns [usbhs_hstpipimr_ctrl_mode::R](usbhs_hstpipimr_ctrl_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPIMR_CTRL_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_ctrl_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipimr_iso_mode](usbhs_hstpipimr_iso_mode) module"]
pub type USBHS_HSTPIPIMR_ISO_MODE = crate::Reg<u32, _USBHS_HSTPIPIMR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIMR_ISO_MODE;
#[doc = "`read()` method returns [usbhs_hstpipimr_iso_mode::R](usbhs_hstpipimr_iso_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPIMR_ISO_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_iso_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipimr_blk_mode](usbhs_hstpipimr_blk_mode) module"]
pub type USBHS_HSTPIPIMR_BLK_MODE = crate::Reg<u32, _USBHS_HSTPIPIMR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIMR_BLK_MODE;
#[doc = "`read()` method returns [usbhs_hstpipimr_blk_mode::R](usbhs_hstpipimr_blk_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPIMR_BLK_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_blk_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipimr_intrpt_mode](usbhs_hstpipimr_intrpt_mode) module"]
pub type USBHS_HSTPIPIMR_INTRPT_MODE = crate::Reg<u32, _USBHS_HSTPIPIMR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIMR_INTRPT_MODE;
#[doc = "`read()` method returns [usbhs_hstpipimr_intrpt_mode::R](usbhs_hstpipimr_intrpt_mode::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPIMR_INTRPT_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_intrpt_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipier_ctrl_mode](usbhs_hstpipier_ctrl_mode) module"]
pub type USBHS_HSTPIPIER_CTRL_MODE = crate::Reg<u32, _USBHS_HSTPIPIER_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIER_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipier_ctrl_mode::W](usbhs_hstpipier_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIER_CTRL_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_ctrl_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipier_iso_mode](usbhs_hstpipier_iso_mode) module"]
pub type USBHS_HSTPIPIER_ISO_MODE = crate::Reg<u32, _USBHS_HSTPIPIER_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIER_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipier_iso_mode::W](usbhs_hstpipier_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIER_ISO_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_iso_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipier_blk_mode](usbhs_hstpipier_blk_mode) module"]
pub type USBHS_HSTPIPIER_BLK_MODE = crate::Reg<u32, _USBHS_HSTPIPIER_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIER_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipier_blk_mode::W](usbhs_hstpipier_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIER_BLK_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_blk_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipier_intrpt_mode](usbhs_hstpipier_intrpt_mode) module"]
pub type USBHS_HSTPIPIER_INTRPT_MODE = crate::Reg<u32, _USBHS_HSTPIPIER_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIER_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipier_intrpt_mode::W](usbhs_hstpipier_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIER_INTRPT_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_intrpt_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipidr_ctrl_mode](usbhs_hstpipidr_ctrl_mode) module"]
pub type USBHS_HSTPIPIDR_CTRL_MODE = crate::Reg<u32, _USBHS_HSTPIPIDR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIDR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipidr_ctrl_mode::W](usbhs_hstpipidr_ctrl_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIDR_CTRL_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_ctrl_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipidr_iso_mode](usbhs_hstpipidr_iso_mode) module"]
pub type USBHS_HSTPIPIDR_ISO_MODE = crate::Reg<u32, _USBHS_HSTPIPIDR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIDR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipidr_iso_mode::W](usbhs_hstpipidr_iso_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIDR_ISO_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_iso_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipidr_blk_mode](usbhs_hstpipidr_blk_mode) module"]
pub type USBHS_HSTPIPIDR_BLK_MODE = crate::Reg<u32, _USBHS_HSTPIPIDR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIDR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipidr_blk_mode::W](usbhs_hstpipidr_blk_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIDR_BLK_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_blk_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipidr_intrpt_mode](usbhs_hstpipidr_intrpt_mode) module"]
pub type USBHS_HSTPIPIDR_INTRPT_MODE = crate::Reg<u32, _USBHS_HSTPIPIDR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIDR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipidr_intrpt_mode::W](usbhs_hstpipidr_intrpt_mode::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIDR_INTRPT_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_intrpt_mode;
#[doc = "Host Pipe IN Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipinrq](usbhs_hstpipinrq) module"]
pub type USBHS_HSTPIPINRQ = crate::Reg<u32, _USBHS_HSTPIPINRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPINRQ;
#[doc = "`read()` method returns [usbhs_hstpipinrq::R](usbhs_hstpipinrq::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPINRQ {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipinrq::W](usbhs_hstpipinrq::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPINRQ {}
#[doc = "Host Pipe IN Request Register"]
pub mod usbhs_hstpipinrq;
#[doc = "Host Pipe Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpiperr](usbhs_hstpiperr) module"]
pub type USBHS_HSTPIPERR = crate::Reg<u32, _USBHS_HSTPIPERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPERR;
#[doc = "`read()` method returns [usbhs_hstpiperr::R](usbhs_hstpiperr::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPERR {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpiperr::W](usbhs_hstpiperr::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPERR {}
#[doc = "Host Pipe Error Register"]
pub mod usbhs_hstpiperr;
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_ctrl](usbhs_ctrl) module"]
pub type USBHS_CTRL = crate::Reg<u32, _USBHS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_CTRL;
#[doc = "`read()` method returns [usbhs_ctrl::R](usbhs_ctrl::R) reader structure"]
impl crate::Readable for USBHS_CTRL {}
#[doc = "`write(|w| ..)` method takes [usbhs_ctrl::W](usbhs_ctrl::W) writer structure"]
impl crate::Writable for USBHS_CTRL {}
#[doc = "General Control Register"]
pub mod usbhs_ctrl;
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_sr](usbhs_sr) module"]
pub type USBHS_SR = crate::Reg<u32, _USBHS_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_SR;
#[doc = "`read()` method returns [usbhs_sr::R](usbhs_sr::R) reader structure"]
impl crate::Readable for USBHS_SR {}
#[doc = "General Status Register"]
pub mod usbhs_sr;
#[doc = "General Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_scr](usbhs_scr) module"]
pub type USBHS_SCR = crate::Reg<u32, _USBHS_SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_SCR;
#[doc = "`write(|w| ..)` method takes [usbhs_scr::W](usbhs_scr::W) writer structure"]
impl crate::Writable for USBHS_SCR {}
#[doc = "General Status Clear Register"]
pub mod usbhs_scr;
#[doc = "General Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_sfr](usbhs_sfr) module"]
pub type USBHS_SFR = crate::Reg<u32, _USBHS_SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_SFR;
#[doc = "`write(|w| ..)` method takes [usbhs_sfr::W](usbhs_sfr::W) writer structure"]
impl crate::Writable for USBHS_SFR {}
#[doc = "General Status Set Register"]
pub mod usbhs_sfr;
