#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CKGR_MOR {
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
pub type MOSCXTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MOSCXTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCXTENW<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MOSCXTBY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MOSCXTBYW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCXTBYW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type WAITMODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WAITMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITMODEW<'a> {
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
pub type MOSCRCEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MOSCRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCRCENW<'a> {
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
#[doc = "Possible values of the field `MOSCRCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSCRCFR {
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    _4_MHZ,
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    _8_MHZ,
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    _12_MHZ,
}
impl crate::ToBits<u8> for MOSCRCFR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MOSCRCFR::_4_MHZ => 0,
            MOSCRCFR::_8_MHZ => 1,
            MOSCRCFR::_12_MHZ => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MOSCRCF_R = crate::FR<u8, MOSCRCFR>;
impl MOSCRCF_R {
    #[doc = "Checks if the value of the field is `_4_MHZ`"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        *self == MOSCRCFR::_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_8_MHZ`"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        *self == MOSCRCFR::_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_12_MHZ`"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        *self == MOSCRCFR::_12_MHZ
    }
}
#[doc = "Values that can be written to the field `MOSCRCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSCRCFW {
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    _4_MHZ,
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    _8_MHZ,
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    _12_MHZ,
}
impl MOSCRCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MOSCRCFW::_4_MHZ => 0,
            MOSCRCFW::_8_MHZ => 1,
            MOSCRCFW::_12_MHZ => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MOSCRCFW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCRCFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOSCRCFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut W {
        self.variant(MOSCRCFW::_4_MHZ)
    }
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut W {
        self.variant(MOSCRCFW::_8_MHZ)
    }
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut W {
        self.variant(MOSCRCFW::_12_MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MOSCXTST_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MOSCXTSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCXTSTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD,
}
impl crate::ToBits<u8> for KEYR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            KEYR::PASSWD => 55,
        }
    }
}
#[doc = r"Reader of the field"]
pub type KEY_R = crate::FR<u8, KEYR>;
impl KEY_R {
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEYR::PASSWD
    }
}
#[doc = "Values that can be written to the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYW {
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYW::PASSWD => 55,
        }
    }
}
#[doc = r"Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEYW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MOSCSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MOSCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CFDEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CFDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CFDENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type XT32KFME_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _XT32KFMEW<'a> {
    w: &'a mut W,
}
impl<'a> _XT32KFMEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MOSCXTEN_R {
        MOSCXTEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MOSCXTBY_R {
        MOSCXTBY_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wait Mode Command (Write-only)"]
    #[inline(always)]
    pub fn waitmode(&self) -> WAITMODE_R {
        WAITMODE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MOSCRCEN_R {
        MOSCRCEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&self) -> MOSCRCF_R {
        MOSCRCF_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MOSCXTST_R {
        MOSCXTST_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MOSCSEL_R {
        MOSCSEL_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline(always)]
    pub fn xt32kfme(&self) -> XT32KFME_R {
        XT32KFME_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&mut self) -> _MOSCXTENW {
        _MOSCXTENW { w: self }
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&mut self) -> _MOSCXTBYW {
        _MOSCXTBYW { w: self }
    }
    #[doc = "Bit 2 - Wait Mode Command (Write-only)"]
    #[inline(always)]
    pub fn waitmode(&mut self) -> _WAITMODEW {
        _WAITMODEW { w: self }
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&mut self) -> _MOSCRCENW {
        _MOSCRCENW { w: self }
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&mut self) -> _MOSCRCFW {
        _MOSCRCFW { w: self }
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline(always)]
    pub fn moscxtst(&mut self) -> _MOSCXTSTW {
        _MOSCXTSTW { w: self }
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&mut self) -> _MOSCSELW {
        _MOSCSELW { w: self }
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> _CFDENW {
        _CFDENW { w: self }
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline(always)]
    pub fn xt32kfme(&mut self) -> _XT32KFMEW {
        _XT32KFMEW { w: self }
    }
}
