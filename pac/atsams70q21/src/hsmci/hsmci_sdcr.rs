#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSMCI_SDCR {
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
#[doc = "Possible values of the field `SDCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCSELR {
    #[doc = "Slot A is selected."]
    SLOTA,
}
impl crate::ToBits<u8> for SDCSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SDCSELR::SLOTA => 0,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SDCSEL_R = crate::FR<u8, SDCSELR>;
impl SDCSEL_R {
    #[doc = "Checks if the value of the field is `SLOTA`"]
    #[inline(always)]
    pub fn is_slota(&self) -> bool {
        *self == SDCSELR::SLOTA
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDCSELW::SLOTA => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SDCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn slota(self) -> &'a mut W {
        self.variant(SDCSELW::SLOTA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
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
}
impl crate::ToBits<u8> for SDCBUSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SDCBUSR::_1 => 0,
            SDCBUSR::_4 => 2,
            SDCBUSR::_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SDCBUS_R = crate::FR<u8, SDCBUSR>;
impl SDCBUS_R {
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCBUSR::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SDCBUSR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SDCBUSR::_8
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDCBUSW::_1 => 0,
            SDCBUSW::_4 => 2,
            SDCBUSW::_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SDCBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCBUSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCBUSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCBUSW::_1)
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SDCBUSW::_4)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SDCBUSW::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&self) -> SDCSEL_R {
        SDCSEL_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&self) -> SDCBUS_R {
        SDCBUS_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&mut self) -> _SDCSELW {
        _SDCSELW { w: self }
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&mut self) -> _SDCBUSW {
        _SDCBUSW { w: self }
    }
}
