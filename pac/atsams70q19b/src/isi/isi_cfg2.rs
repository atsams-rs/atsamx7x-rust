#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISI_CFG2 {
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
pub struct IM_VSIZER {
    bits: u16,
}
impl IM_VSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GS_MODER {
    bits: bool,
}
impl GS_MODER {
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
pub struct RGB_MODER {
    bits: bool,
}
impl RGB_MODER {
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
pub struct GRAYSCALER {
    bits: bool,
}
impl GRAYSCALER {
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
pub struct RGB_SWAPR {
    bits: bool,
}
impl RGB_SWAPR {
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
pub struct COL_SPACER {
    bits: bool,
}
impl COL_SPACER {
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
pub struct IM_HSIZER {
    bits: u16,
}
impl IM_HSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `YCC_SWAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YCC_SWAPR {
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    DEFAULT,
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    MODE1,
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    MODE2,
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    MODE3,
}
impl YCC_SWAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            YCC_SWAPR::DEFAULT => 0,
            YCC_SWAPR::MODE1 => 1,
            YCC_SWAPR::MODE2 => 2,
            YCC_SWAPR::MODE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> YCC_SWAPR {
        match value {
            0 => YCC_SWAPR::DEFAULT,
            1 => YCC_SWAPR::MODE1,
            2 => YCC_SWAPR::MODE2,
            3 => YCC_SWAPR::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == YCC_SWAPR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == YCC_SWAPR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == YCC_SWAPR::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == YCC_SWAPR::MODE3
    }
}
#[doc = "Possible values of the field `RGB_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGB_CFGR {
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    DEFAULT,
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    MODE1,
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    MODE2,
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    MODE3,
}
impl RGB_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RGB_CFGR::DEFAULT => 0,
            RGB_CFGR::MODE1 => 1,
            RGB_CFGR::MODE2 => 2,
            RGB_CFGR::MODE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RGB_CFGR {
        match value {
            0 => RGB_CFGR::DEFAULT,
            1 => RGB_CFGR::MODE1,
            2 => RGB_CFGR::MODE2,
            3 => RGB_CFGR::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == RGB_CFGR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == RGB_CFGR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == RGB_CFGR::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == RGB_CFGR::MODE3
    }
}
#[doc = r" Proxy"]
pub struct _IM_VSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _IM_VSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GS_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _GS_MODEW<'a> {
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
pub struct _RGB_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RGB_MODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _GRAYSCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _GRAYSCALEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RGB_SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _RGB_SWAPW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COL_SPACEW<'a> {
    w: &'a mut W,
}
impl<'a> _COL_SPACEW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IM_HSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _IM_HSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `YCC_SWAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YCC_SWAPW {
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    DEFAULT,
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    MODE1,
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    MODE2,
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    MODE3,
}
impl YCC_SWAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            YCC_SWAPW::DEFAULT => 0,
            YCC_SWAPW::MODE1 => 1,
            YCC_SWAPW::MODE2 => 2,
            YCC_SWAPW::MODE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _YCC_SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _YCC_SWAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: YCC_SWAPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(YCC_SWAPW::DEFAULT)
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(YCC_SWAPW::MODE1)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(YCC_SWAPW::MODE2)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(YCC_SWAPW::MODE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RGB_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGB_CFGW {
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    DEFAULT,
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    MODE1,
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    MODE2,
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    MODE3,
}
impl RGB_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RGB_CFGW::DEFAULT => 0,
            RGB_CFGW::MODE1 => 1,
            RGB_CFGW::MODE2 => 2,
            RGB_CFGW::MODE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGB_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _RGB_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGB_CFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(RGB_CFGW::DEFAULT)
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(RGB_CFGW::MODE1)
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(RGB_CFGW::MODE2)
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(RGB_CFGW::MODE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline]
    pub fn im_vsize(&self) -> IM_VSIZER {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IM_VSIZER { bits }
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline]
    pub fn gs_mode(&self) -> GS_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GS_MODER { bits }
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline]
    pub fn rgb_mode(&self) -> RGB_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RGB_MODER { bits }
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline]
    pub fn grayscale(&self) -> GRAYSCALER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GRAYSCALER { bits }
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline]
    pub fn rgb_swap(&self) -> RGB_SWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RGB_SWAPR { bits }
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline]
    pub fn col_space(&self) -> COL_SPACER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COL_SPACER { bits }
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline]
    pub fn im_hsize(&self) -> IM_HSIZER {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IM_HSIZER { bits }
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline]
    pub fn ycc_swap(&self) -> YCC_SWAPR {
        YCC_SWAPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline]
    pub fn rgb_cfg(&self) -> RGB_CFGR {
        RGB_CFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline]
    pub fn im_vsize(&mut self) -> _IM_VSIZEW {
        _IM_VSIZEW { w: self }
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline]
    pub fn gs_mode(&mut self) -> _GS_MODEW {
        _GS_MODEW { w: self }
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline]
    pub fn rgb_mode(&mut self) -> _RGB_MODEW {
        _RGB_MODEW { w: self }
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline]
    pub fn grayscale(&mut self) -> _GRAYSCALEW {
        _GRAYSCALEW { w: self }
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline]
    pub fn rgb_swap(&mut self) -> _RGB_SWAPW {
        _RGB_SWAPW { w: self }
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline]
    pub fn col_space(&mut self) -> _COL_SPACEW {
        _COL_SPACEW { w: self }
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline]
    pub fn im_hsize(&mut self) -> _IM_HSIZEW {
        _IM_HSIZEW { w: self }
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline]
    pub fn ycc_swap(&mut self) -> _YCC_SWAPW {
        _YCC_SWAPW { w: self }
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline]
    pub fn rgb_cfg(&mut self) -> _RGB_CFGW {
        _RGB_CFGW { w: self }
    }
}
