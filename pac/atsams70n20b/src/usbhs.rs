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
#[doc = "Device General Control Register"]
pub struct USBHS_DEVCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device General Control Register"]
pub mod usbhs_devctrl;
#[doc = "Device Global Interrupt Status Register"]
pub struct USBHS_DEVISR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Status Register"]
pub mod usbhs_devisr;
#[doc = "Device Global Interrupt Clear Register"]
pub struct USBHS_DEVICR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Clear Register"]
pub mod usbhs_devicr;
#[doc = "Device Global Interrupt Set Register"]
pub struct USBHS_DEVIFR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Set Register"]
pub mod usbhs_devifr;
#[doc = "Device Global Interrupt Mask Register"]
pub struct USBHS_DEVIMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Mask Register"]
pub mod usbhs_devimr;
#[doc = "Device Global Interrupt Disable Register"]
pub struct USBHS_DEVIDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Disable Register"]
pub mod usbhs_devidr;
#[doc = "Device Global Interrupt Enable Register"]
pub struct USBHS_DEVIER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Enable Register"]
pub mod usbhs_devier;
#[doc = "Device Endpoint Register"]
pub struct USBHS_DEVEPT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Register"]
pub mod usbhs_devept;
#[doc = "Device Frame Number Register"]
pub struct USBHS_DEVFNUM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Frame Number Register"]
pub mod usbhs_devfnum;
#[doc = "Device Endpoint Configuration Register"]
pub struct USBHS_DEVEPTCFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Configuration Register"]
pub mod usbhs_deveptcfg;
#[doc = "Device Endpoint Interrupt Status Register"]
pub struct USBHS_DEVEPTISR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Status Register"]
pub struct USBHS_DEVEPTISR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_iso_mode;
#[doc = "Device Endpoint Interrupt Status Register"]
pub struct USBHS_DEVEPTISR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_blk_mode;
#[doc = "Device Endpoint Interrupt Status Register"]
pub struct USBHS_DEVEPTISR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub struct USBHS_DEVEPTICR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub struct USBHS_DEVEPTICR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_iso_mode;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub struct USBHS_DEVEPTICR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_blk_mode;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub struct USBHS_DEVEPTICR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Set Register"]
pub struct USBHS_DEVEPTIFR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Set Register"]
pub struct USBHS_DEVEPTIFR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_iso_mode;
#[doc = "Device Endpoint Interrupt Set Register"]
pub struct USBHS_DEVEPTIFR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_blk_mode;
#[doc = "Device Endpoint Interrupt Set Register"]
pub struct USBHS_DEVEPTIFR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub struct USBHS_DEVEPTIMR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub struct USBHS_DEVEPTIMR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_iso_mode;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub struct USBHS_DEVEPTIMR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_blk_mode;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub struct USBHS_DEVEPTIMR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub struct USBHS_DEVEPTIER_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_ctrl_mode;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub struct USBHS_DEVEPTIER_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_iso_mode;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub struct USBHS_DEVEPTIER_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_blk_mode;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub struct USBHS_DEVEPTIER_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_intrpt_mode;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub struct USBHS_DEVEPTIDR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub struct USBHS_DEVEPTIDR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_iso_mode;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub struct USBHS_DEVEPTIDR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_blk_mode;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub struct USBHS_DEVEPTIDR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_intrpt_mode;
#[doc = "Host General Control Register"]
pub struct USBHS_HSTCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host General Control Register"]
pub mod usbhs_hstctrl;
#[doc = "Host Global Interrupt Status Register"]
pub struct USBHS_HSTISR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Status Register"]
pub mod usbhs_hstisr;
#[doc = "Host Global Interrupt Clear Register"]
pub struct USBHS_HSTICR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Clear Register"]
pub mod usbhs_hsticr;
#[doc = "Host Global Interrupt Set Register"]
pub struct USBHS_HSTIFR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Set Register"]
pub mod usbhs_hstifr;
#[doc = "Host Global Interrupt Mask Register"]
pub struct USBHS_HSTIMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Mask Register"]
pub mod usbhs_hstimr;
#[doc = "Host Global Interrupt Disable Register"]
pub struct USBHS_HSTIDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Disable Register"]
pub mod usbhs_hstidr;
#[doc = "Host Global Interrupt Enable Register"]
pub struct USBHS_HSTIER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Enable Register"]
pub mod usbhs_hstier;
#[doc = "Host Pipe Register"]
pub struct USBHS_HSTPIP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Register"]
pub mod usbhs_hstpip;
#[doc = "Host Frame Number Register"]
pub struct USBHS_HSTFNUM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Frame Number Register"]
pub mod usbhs_hstfnum;
#[doc = "Host Address 1 Register"]
pub struct USBHS_HSTADDR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Address 1 Register"]
pub mod usbhs_hstaddr1;
#[doc = "Host Address 2 Register"]
pub struct USBHS_HSTADDR2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Address 2 Register"]
pub mod usbhs_hstaddr2;
#[doc = "Host Address 3 Register"]
pub struct USBHS_HSTADDR3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Address 3 Register"]
pub mod usbhs_hstaddr3;
#[doc = "Host Pipe Configuration Register"]
pub struct USBHS_HSTPIPCFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Configuration Register"]
pub mod usbhs_hstpipcfg;
#[doc = "Host Pipe Configuration Register"]
pub struct USBHS_HSTPIPCFG_CTRL_BULK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Configuration Register"]
pub mod usbhs_hstpipcfg_ctrl_bulk_mode;
#[doc = "Host Pipe Status Register"]
pub struct USBHS_HSTPIPISR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_ctrl_mode;
#[doc = "Host Pipe Status Register"]
pub struct USBHS_HSTPIPISR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_iso_mode;
#[doc = "Host Pipe Status Register"]
pub struct USBHS_HSTPIPISR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_blk_mode;
#[doc = "Host Pipe Status Register"]
pub struct USBHS_HSTPIPISR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_intrpt_mode;
#[doc = "Host Pipe Clear Register"]
pub struct USBHS_HSTPIPICR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_ctrl_mode;
#[doc = "Host Pipe Clear Register"]
pub struct USBHS_HSTPIPICR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_iso_mode;
#[doc = "Host Pipe Clear Register"]
pub struct USBHS_HSTPIPICR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_blk_mode;
#[doc = "Host Pipe Clear Register"]
pub struct USBHS_HSTPIPICR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_intrpt_mode;
#[doc = "Host Pipe Set Register"]
pub struct USBHS_HSTPIPIFR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_ctrl_mode;
#[doc = "Host Pipe Set Register"]
pub struct USBHS_HSTPIPIFR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_iso_mode;
#[doc = "Host Pipe Set Register"]
pub struct USBHS_HSTPIPIFR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_blk_mode;
#[doc = "Host Pipe Set Register"]
pub struct USBHS_HSTPIPIFR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_intrpt_mode;
#[doc = "Host Pipe Mask Register"]
pub struct USBHS_HSTPIPIMR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_ctrl_mode;
#[doc = "Host Pipe Mask Register"]
pub struct USBHS_HSTPIPIMR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_iso_mode;
#[doc = "Host Pipe Mask Register"]
pub struct USBHS_HSTPIPIMR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_blk_mode;
#[doc = "Host Pipe Mask Register"]
pub struct USBHS_HSTPIPIMR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_intrpt_mode;
#[doc = "Host Pipe Enable Register"]
pub struct USBHS_HSTPIPIER_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_ctrl_mode;
#[doc = "Host Pipe Enable Register"]
pub struct USBHS_HSTPIPIER_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_iso_mode;
#[doc = "Host Pipe Enable Register"]
pub struct USBHS_HSTPIPIER_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_blk_mode;
#[doc = "Host Pipe Enable Register"]
pub struct USBHS_HSTPIPIER_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_intrpt_mode;
#[doc = "Host Pipe Disable Register"]
pub struct USBHS_HSTPIPIDR_CTRL_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_ctrl_mode;
#[doc = "Host Pipe Disable Register"]
pub struct USBHS_HSTPIPIDR_ISO_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_iso_mode;
#[doc = "Host Pipe Disable Register"]
pub struct USBHS_HSTPIPIDR_BLK_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_blk_mode;
#[doc = "Host Pipe Disable Register"]
pub struct USBHS_HSTPIPIDR_INTRPT_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_intrpt_mode;
#[doc = "Host Pipe IN Request Register"]
pub struct USBHS_HSTPIPINRQ {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe IN Request Register"]
pub mod usbhs_hstpipinrq;
#[doc = "Host Pipe Error Register"]
pub struct USBHS_HSTPIPERR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Error Register"]
pub mod usbhs_hstpiperr;
#[doc = "General Control Register"]
pub struct USBHS_CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "General Control Register"]
pub mod usbhs_ctrl;
#[doc = "General Status Register"]
pub struct USBHS_SR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "General Status Register"]
pub mod usbhs_sr;
#[doc = "General Status Clear Register"]
pub struct USBHS_SCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "General Status Clear Register"]
pub mod usbhs_scr;
#[doc = "General Status Set Register"]
pub struct USBHS_SFR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "General Status Set Register"]
pub mod usbhs_sfr;
