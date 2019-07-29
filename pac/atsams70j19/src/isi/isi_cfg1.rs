#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISI_CFG1 {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type HSYNC_POL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSYNC_POLW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type VSYNC_POL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _VSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_POLW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PIXCLK_POL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PIXCLK_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _PIXCLK_POLW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type GRAYLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GRAYLEW<'a> {
    w: &'a mut W,
}
impl<'a> _GRAYLEW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EMB_SYNC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMB_SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMB_SYNCW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CRC_SYNC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CRC_SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_SYNCW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FRATE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FRATEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRATEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DISCR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DISCRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FULL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `THMASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THMASKR {
    #[doc = "Only 4 beats AHB burst allowed"]
    BEATS_4,
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    BEATS_8,
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    BEATS_16,
}
impl crate::ToBits<u8> for THMASKR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            THMASKR::BEATS_4 => 0,
            THMASKR::BEATS_8 => 1,
            THMASKR::BEATS_16 => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type THMASK_R = crate::FR<u8, THMASKR>;
impl THMASK_R {
    #[doc = "Checks if the value of the field is `BEATS_4`"]
    #[inline(always)]
    pub fn is_beats_4(&self) -> bool {
        *self == THMASKR::BEATS_4
    }
    #[doc = "Checks if the value of the field is `BEATS_8`"]
    #[inline(always)]
    pub fn is_beats_8(&self) -> bool {
        *self == THMASKR::BEATS_8
    }
    #[doc = "Checks if the value of the field is `BEATS_16`"]
    #[inline(always)]
    pub fn is_beats_16(&self) -> bool {
        *self == THMASKR::BEATS_16
    }
}
#[doc = "Values that can be written to the field `THMASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THMASKW {
    #[doc = "Only 4 beats AHB burst allowed"]
    BEATS_4,
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    BEATS_8,
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    BEATS_16,
}
impl THMASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            THMASKW::BEATS_4 => 0,
            THMASKW::BEATS_8 => 1,
            THMASKW::BEATS_16 => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _THMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _THMASKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THMASKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only 4 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_4(self) -> &'a mut W {
        self.variant(THMASKW::BEATS_4)
    }
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_8(self) -> &'a mut W {
        self.variant(THMASKW::BEATS_8)
    }
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_16(self) -> &'a mut W {
        self.variant(THMASKW::BEATS_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SLD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SLDW<'a> {
    w: &'a mut W,
}
impl<'a> _SLDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SFD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SFDW<'a> {
    w: &'a mut W,
}
impl<'a> _SFDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HSYNC_POL_R {
        HSYNC_POL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&self) -> VSYNC_POL_R {
        VSYNC_POL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&self) -> PIXCLK_POL_R {
        PIXCLK_POL_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&self) -> GRAYLE_R {
        GRAYLE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&self) -> EMB_SYNC_R {
        EMB_SYNC_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&self) -> CRC_SYNC_R {
        CRC_SYNC_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&self) -> FRATE_R {
        FRATE_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&self) -> DISCR_R {
        DISCR_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&self) -> THMASK_R {
        THMASK_R::new(((self.bits() >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&self) -> SLD_R {
        SLD_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&mut self) -> _HSYNC_POLW {
        _HSYNC_POLW { w: self }
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&mut self) -> _VSYNC_POLW {
        _VSYNC_POLW { w: self }
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&mut self) -> _PIXCLK_POLW {
        _PIXCLK_POLW { w: self }
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&mut self) -> _GRAYLEW {
        _GRAYLEW { w: self }
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&mut self) -> _EMB_SYNCW {
        _EMB_SYNCW { w: self }
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&mut self) -> _CRC_SYNCW {
        _CRC_SYNCW { w: self }
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&mut self) -> _FRATEW {
        _FRATEW { w: self }
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&mut self) -> _DISCRW {
        _DISCRW { w: self }
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&mut self) -> _FULLW {
        _FULLW { w: self }
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&mut self) -> _THMASKW {
        _THMASKW { w: self }
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&mut self) -> _SLDW {
        _SLDW { w: self }
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&mut self) -> _SFDW {
        _SFDW { w: self }
    }
}
