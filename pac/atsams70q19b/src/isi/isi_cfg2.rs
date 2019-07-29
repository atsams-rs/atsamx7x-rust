#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISI_CFG2 {
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
pub type IM_VSIZE_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _IM_VSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _IM_VSIZEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type GS_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GS_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _GS_MODEW<'a> {
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
pub type RGB_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RGB_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RGB_MODEW<'a> {
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
#[doc = r"Reader of the field"]
pub type GRAYSCALE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GRAYSCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _GRAYSCALEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RGB_SWAP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RGB_SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _RGB_SWAPW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type COL_SPACE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _COL_SPACEW<'a> {
    w: &'a mut W,
}
impl<'a> _COL_SPACEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type IM_HSIZE_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _IM_HSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _IM_HSIZEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
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
impl crate::ToBits<u8> for YCC_SWAPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            YCC_SWAPR::DEFAULT => 0,
            YCC_SWAPR::MODE1 => 1,
            YCC_SWAPR::MODE2 => 2,
            YCC_SWAPR::MODE3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type YCC_SWAP_R = crate::FR<u8, YCC_SWAPR>;
impl YCC_SWAP_R {
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == YCC_SWAPR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == YCC_SWAPR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == YCC_SWAPR::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == YCC_SWAPR::MODE3
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            YCC_SWAPW::DEFAULT => 0,
            YCC_SWAPW::MODE1 => 1,
            YCC_SWAPW::MODE2 => 2,
            YCC_SWAPW::MODE3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _YCC_SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _YCC_SWAPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YCC_SWAPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(YCC_SWAPW::DEFAULT)
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(YCC_SWAPW::MODE1)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(YCC_SWAPW::MODE2)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(YCC_SWAPW::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
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
impl crate::ToBits<u8> for RGB_CFGR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RGB_CFGR::DEFAULT => 0,
            RGB_CFGR::MODE1 => 1,
            RGB_CFGR::MODE2 => 2,
            RGB_CFGR::MODE3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RGB_CFG_R = crate::FR<u8, RGB_CFGR>;
impl RGB_CFG_R {
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RGB_CFGR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == RGB_CFGR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == RGB_CFGR::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == RGB_CFGR::MODE3
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            RGB_CFGW::DEFAULT => 0,
            RGB_CFGW::MODE1 => 1,
            RGB_CFGW::MODE2 => 2,
            RGB_CFGW::MODE3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RGB_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _RGB_CFGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB_CFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RGB_CFGW::DEFAULT)
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(RGB_CFGW::MODE1)
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(RGB_CFGW::MODE2)
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(RGB_CFGW::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&self) -> IM_VSIZE_R {
        IM_VSIZE_R::new((self.bits() & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&self) -> GS_MODE_R {
        GS_MODE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&self) -> RGB_MODE_R {
        RGB_MODE_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&self) -> GRAYSCALE_R {
        GRAYSCALE_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&self) -> RGB_SWAP_R {
        RGB_SWAP_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&self) -> COL_SPACE_R {
        COL_SPACE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&self) -> IM_HSIZE_R {
        IM_HSIZE_R::new(((self.bits() >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&self) -> YCC_SWAP_R {
        YCC_SWAP_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&self) -> RGB_CFG_R {
        RGB_CFG_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&mut self) -> _IM_VSIZEW {
        _IM_VSIZEW { w: self }
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&mut self) -> _GS_MODEW {
        _GS_MODEW { w: self }
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&mut self) -> _RGB_MODEW {
        _RGB_MODEW { w: self }
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&mut self) -> _GRAYSCALEW {
        _GRAYSCALEW { w: self }
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&mut self) -> _RGB_SWAPW {
        _RGB_SWAPW { w: self }
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&mut self) -> _COL_SPACEW {
        _COL_SPACEW { w: self }
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&mut self) -> _IM_HSIZEW {
        _IM_HSIZEW { w: self }
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&mut self) -> _YCC_SWAPW {
        _YCC_SWAPW { w: self }
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&mut self) -> _RGB_CFGW {
        _RGB_CFGW { w: self }
    }
}
