#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CKGR_MOR {
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
pub struct MOSCXTENR {
    bits: bool,
}
impl MOSCXTENR {
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
pub struct MOSCXTBYR {
    bits: bool,
}
impl MOSCXTBYR {
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
pub struct WAITMODER {
    bits: bool,
}
impl WAITMODER {
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
pub struct MOSCRCENR {
    bits: bool,
}
impl MOSCRCENR {
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
#[doc = "Possible values of the field `MOSCRCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSCRCFR {
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    _4_MHZ,
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    _8_MHZ,
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    _12_MHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MOSCRCFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MOSCRCFR::_4_MHZ => 0,
            MOSCRCFR::_8_MHZ => 1,
            MOSCRCFR::_12_MHZ => 2,
            MOSCRCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MOSCRCFR {
        match value {
            0 => MOSCRCFR::_4_MHZ,
            1 => MOSCRCFR::_8_MHZ,
            2 => MOSCRCFR::_12_MHZ,
            i => MOSCRCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4_MHZ`"]
    #[inline]
    pub fn is_4_mhz(&self) -> bool {
        *self == MOSCRCFR::_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_8_MHZ`"]
    #[inline]
    pub fn is_8_mhz(&self) -> bool {
        *self == MOSCRCFR::_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_12_MHZ`"]
    #[inline]
    pub fn is_12_mhz(&self) -> bool {
        *self == MOSCRCFR::_12_MHZ
    }
}
#[doc = r" Value of the field"]
pub struct MOSCXTSTR {
    bits: u8,
}
impl MOSCXTSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYR::PASSWD => 55,
            KEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYR {
        match value {
            55 => KEYR::PASSWD,
            i => KEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline]
    pub fn is_passwd(&self) -> bool {
        *self == KEYR::PASSWD
    }
}
#[doc = r" Value of the field"]
pub struct MOSCSELR {
    bits: bool,
}
impl MOSCSELR {
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
pub struct CFDENR {
    bits: bool,
}
impl CFDENR {
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
pub struct XT32KFMER {
    bits: bool,
}
impl XT32KFMER {
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
#[doc = r" Proxy"]
pub struct _MOSCXTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCXTENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MOSCXTBYW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCXTBYW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAITMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITMODEW<'a> {
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
pub struct _MOSCRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCRCENW<'a> {
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MOSCRCFW::_4_MHZ => 0,
            MOSCRCFW::_8_MHZ => 1,
            MOSCRCFW::_12_MHZ => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOSCRCFW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCRCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOSCRCFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    #[inline]
    pub fn _4_mhz(self) -> &'a mut W {
        self.variant(MOSCRCFW::_4_MHZ)
    }
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    #[inline]
    pub fn _8_mhz(self) -> &'a mut W {
        self.variant(MOSCRCFW::_8_MHZ)
    }
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    #[inline]
    pub fn _12_mhz(self) -> &'a mut W {
        self.variant(MOSCRCFW::_12_MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MOSCXTSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCXTSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYW::PASSWD => 55,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEYW::PASSWD)
    }
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
pub struct _MOSCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCSELW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CFDENW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XT32KFMEW<'a> {
    w: &'a mut W,
}
impl<'a> _XT32KFMEW<'a> {
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline]
    pub fn moscxten(&self) -> MOSCXTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MOSCXTENR { bits }
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline]
    pub fn moscxtby(&self) -> MOSCXTBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MOSCXTBYR { bits }
    }
    #[doc = "Bit 2 - Wait Mode Command (Write-only)"]
    #[inline]
    pub fn waitmode(&self) -> WAITMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAITMODER { bits }
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline]
    pub fn moscrcen(&self) -> MOSCRCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MOSCRCENR { bits }
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline]
    pub fn moscrcf(&self) -> MOSCRCFR {
        MOSCRCFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline]
    pub fn moscxtst(&self) -> MOSCXTSTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MOSCXTSTR { bits }
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline]
    pub fn key(&self) -> KEYR {
        KEYR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline]
    pub fn moscsel(&self) -> MOSCSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MOSCSELR { bits }
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline]
    pub fn cfden(&self) -> CFDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CFDENR { bits }
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline]
    pub fn xt32kfme(&self) -> XT32KFMER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XT32KFMER { bits }
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
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline]
    pub fn moscxten(&mut self) -> _MOSCXTENW {
        _MOSCXTENW { w: self }
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline]
    pub fn moscxtby(&mut self) -> _MOSCXTBYW {
        _MOSCXTBYW { w: self }
    }
    #[doc = "Bit 2 - Wait Mode Command (Write-only)"]
    #[inline]
    pub fn waitmode(&mut self) -> _WAITMODEW {
        _WAITMODEW { w: self }
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline]
    pub fn moscrcen(&mut self) -> _MOSCRCENW {
        _MOSCRCENW { w: self }
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline]
    pub fn moscrcf(&mut self) -> _MOSCRCFW {
        _MOSCRCFW { w: self }
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline]
    pub fn moscxtst(&mut self) -> _MOSCXTSTW {
        _MOSCXTSTW { w: self }
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline]
    pub fn moscsel(&mut self) -> _MOSCSELW {
        _MOSCSELW { w: self }
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline]
    pub fn cfden(&mut self) -> _CFDENW {
        _CFDENW { w: self }
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline]
    pub fn xt32kfme(&mut self) -> _XT32KFMEW {
        _XT32KFMEW { w: self }
    }
}
