#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISI_CFG1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct HSYNC_POLR {
    bits: bool,
}
impl HSYNC_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VSYNC_POLR {
    bits: bool,
}
impl VSYNC_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PIXCLK_POLR {
    bits: bool,
}
impl PIXCLK_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct GRAYLER {
    bits: bool,
}
impl GRAYLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct EMB_SYNCR {
    bits: bool,
}
impl EMB_SYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CRC_SYNCR {
    bits: bool,
}
impl CRC_SYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FRATER {
    bits: u8,
}
impl FRATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DISCRR {
    bits: bool,
}
impl DISCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FULLR {
    bits: bool,
}
impl FULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl THMASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            THMASKR::BEATS_4 => 0,
            THMASKR::BEATS_8 => 1,
            THMASKR::BEATS_16 => 2,
            THMASKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> THMASKR {
        match value {
            0 => THMASKR::BEATS_4,
            1 => THMASKR::BEATS_8,
            2 => THMASKR::BEATS_16,
            i => THMASKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BEATS_4`"]
    #[inline]
    pub fn is_beats_4(&self) -> bool {
        *self == THMASKR::BEATS_4
    }
    #[doc = "Checks if the value of the field is `BEATS_8`"]
    #[inline]
    pub fn is_beats_8(&self) -> bool {
        *self == THMASKR::BEATS_8
    }
    #[doc = "Checks if the value of the field is `BEATS_16`"]
    #[inline]
    pub fn is_beats_16(&self) -> bool {
        *self == THMASKR::BEATS_16
    }
}
#[doc = r" Value of the field"]
pub struct SLDR {
    bits: u8,
}
impl SLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SFDR {
    bits: u8,
}
impl SFDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _HSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSYNC_POLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_POLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIXCLK_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _PIXCLK_POLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GRAYLEW<'a> {
    w: &'a mut W,
}
impl<'a> _GRAYLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EMB_SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMB_SYNCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRC_SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_SYNCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRATEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRATEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISCRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            THMASKW::BEATS_4 => 0,
            THMASKW::BEATS_8 => 1,
            THMASKW::BEATS_16 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _THMASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THMASKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only 4 beats AHB burst allowed"]
    #[inline]
    pub fn beats_4(self) -> &'a mut W {
        self.variant(THMASKW::BEATS_4)
    }
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    #[inline]
    pub fn beats_8(self) -> &'a mut W {
        self.variant(THMASKW::BEATS_8)
    }
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    #[inline]
    pub fn beats_16(self) -> &'a mut W {
        self.variant(THMASKW::BEATS_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLDW<'a> {
    w: &'a mut W,
}
impl<'a> _SLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFDW<'a> {
    w: &'a mut W,
}
impl<'a> _SFDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline]
    pub fn hsync_pol(&self) -> HSYNC_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSYNC_POLR { bits }
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline]
    pub fn vsync_pol(&self) -> VSYNC_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VSYNC_POLR { bits }
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline]
    pub fn pixclk_pol(&self) -> PIXCLK_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIXCLK_POLR { bits }
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline]
    pub fn grayle(&self) -> GRAYLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GRAYLER { bits }
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline]
    pub fn emb_sync(&self) -> EMB_SYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMB_SYNCR { bits }
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline]
    pub fn crc_sync(&self) -> CRC_SYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CRC_SYNCR { bits }
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline]
    pub fn frate(&self) -> FRATER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRATER { bits }
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline]
    pub fn discr(&self) -> DISCRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISCRR { bits }
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline]
    pub fn full(&self) -> FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FULLR { bits }
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline]
    pub fn thmask(&self) -> THMASKR {
        THMASKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline]
    pub fn sld(&self) -> SLDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLDR { bits }
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline]
    pub fn sfd(&self) -> SFDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SFDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline]
    pub fn hsync_pol(&mut self) -> _HSYNC_POLW {
        _HSYNC_POLW { w: self }
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline]
    pub fn vsync_pol(&mut self) -> _VSYNC_POLW {
        _VSYNC_POLW { w: self }
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline]
    pub fn pixclk_pol(&mut self) -> _PIXCLK_POLW {
        _PIXCLK_POLW { w: self }
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline]
    pub fn grayle(&mut self) -> _GRAYLEW {
        _GRAYLEW { w: self }
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline]
    pub fn emb_sync(&mut self) -> _EMB_SYNCW {
        _EMB_SYNCW { w: self }
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline]
    pub fn crc_sync(&mut self) -> _CRC_SYNCW {
        _CRC_SYNCW { w: self }
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline]
    pub fn frate(&mut self) -> _FRATEW {
        _FRATEW { w: self }
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline]
    pub fn discr(&mut self) -> _DISCRW {
        _DISCRW { w: self }
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline]
    pub fn full(&mut self) -> _FULLW {
        _FULLW { w: self }
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline]
    pub fn thmask(&mut self) -> _THMASKW {
        _THMASKW { w: self }
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline]
    pub fn sld(&mut self) -> _SLDW {
        _SLDW { w: self }
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline]
    pub fn sfd(&mut self) -> _SFDW {
        _SFDW { w: self }
    }
}
