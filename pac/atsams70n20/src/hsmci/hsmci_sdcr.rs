#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSMCI_SDCR {
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
#[doc = "Possible values of the field `SDCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCSELR {
    #[doc = "Slot A is selected."]
    SLOTA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SDCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDCSELR::SLOTA => 0,
            SDCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDCSELR {
        match value {
            0 => SDCSELR::SLOTA,
            i => SDCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOTA`"]
    #[inline]
    pub fn is_slota(&self) -> bool {
        *self == SDCSELR::SLOTA
    }
}
#[doc = "Possible values of the field `SDCBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCBUSR {
    #[doc = "1 bit"]
    _1,
    #[doc = "4 bits"]
    _4,
    #[doc = "8 bits"]
    _8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SDCBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDCBUSR::_1 => 0,
            SDCBUSR::_4 => 2,
            SDCBUSR::_8 => 3,
            SDCBUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDCBUSR {
        match value {
            0 => SDCBUSR::_1,
            2 => SDCBUSR::_4,
            3 => SDCBUSR::_8,
            i => SDCBUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDCBUSR::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SDCBUSR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == SDCBUSR::_8
    }
}
#[doc = "Values that can be written to the field `SDCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCSELW {
    #[doc = "Slot A is selected."]
    SLOTA,
}
impl SDCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDCSELW::SLOTA => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Slot A is selected."]
    #[inline]
    pub fn slota(self) -> &'a mut W {
        self.variant(SDCSELW::SLOTA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDCBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCBUSW {
    #[doc = "1 bit"]
    _1,
    #[doc = "4 bits"]
    _4,
    #[doc = "8 bits"]
    _8,
}
impl SDCBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDCBUSW::_1 => 0,
            SDCBUSW::_4 => 2,
            SDCBUSW::_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCBUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCBUSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 bit"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCBUSW::_1)
    }
    #[doc = "4 bits"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(SDCBUSW::_4)
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(SDCBUSW::_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline]
    pub fn sdcsel(&self) -> SDCSELR {
        SDCSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline]
    pub fn sdcbus(&self) -> SDCBUSR {
        SDCBUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline]
    pub fn sdcsel(&mut self) -> _SDCSELW {
        _SDCSELW { w: self }
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline]
    pub fn sdcbus(&mut self) -> _SDCBUSW {
        _SDCBUSW { w: self }
    }
}
