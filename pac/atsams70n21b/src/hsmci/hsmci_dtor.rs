#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSMCI_DTOR {
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
pub struct DTOCYCR {
    bits: u8,
}
impl DTOCYCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DTOMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOMULR {
    #[doc = "DTOCYC"]
    _1,
    #[doc = "DTOCYC x 16"]
    _16,
    #[doc = "DTOCYC x 128"]
    _128,
    #[doc = "DTOCYC x 256"]
    _256,
    #[doc = "DTOCYC x 1024"]
    _1024,
    #[doc = "DTOCYC x 4096"]
    _4096,
    #[doc = "DTOCYC x 65536"]
    _65536,
    #[doc = "DTOCYC x 1048576"]
    _1048576,
}
impl DTOMULR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTOMULR::_1 => 0,
            DTOMULR::_16 => 1,
            DTOMULR::_128 => 2,
            DTOMULR::_256 => 3,
            DTOMULR::_1024 => 4,
            DTOMULR::_4096 => 5,
            DTOMULR::_65536 => 6,
            DTOMULR::_1048576 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTOMULR {
        match value {
            0 => DTOMULR::_1,
            1 => DTOMULR::_16,
            2 => DTOMULR::_128,
            3 => DTOMULR::_256,
            4 => DTOMULR::_1024,
            5 => DTOMULR::_4096,
            6 => DTOMULR::_65536,
            7 => DTOMULR::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTOMULR::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == DTOMULR::_16
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == DTOMULR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == DTOMULR::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == DTOMULR::_1024
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline]
    pub fn is_4096(&self) -> bool {
        *self == DTOMULR::_4096
    }
    #[doc = "Checks if the value of the field is `_65536`"]
    #[inline]
    pub fn is_65536(&self) -> bool {
        *self == DTOMULR::_65536
    }
    #[doc = "Checks if the value of the field is `_1048576`"]
    #[inline]
    pub fn is_1048576(&self) -> bool {
        *self == DTOMULR::_1048576
    }
}
#[doc = r" Proxy"]
pub struct _DTOCYCW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOCYCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTOMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOMULW {
    #[doc = "DTOCYC"]
    _1,
    #[doc = "DTOCYC x 16"]
    _16,
    #[doc = "DTOCYC x 128"]
    _128,
    #[doc = "DTOCYC x 256"]
    _256,
    #[doc = "DTOCYC x 1024"]
    _1024,
    #[doc = "DTOCYC x 4096"]
    _4096,
    #[doc = "DTOCYC x 65536"]
    _65536,
    #[doc = "DTOCYC x 1048576"]
    _1048576,
}
impl DTOMULW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTOMULW::_1 => 0,
            DTOMULW::_16 => 1,
            DTOMULW::_128 => 2,
            DTOMULW::_256 => 3,
            DTOMULW::_1024 => 4,
            DTOMULW::_4096 => 5,
            DTOMULW::_65536 => 6,
            DTOMULW::_1048576 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOMULW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOMULW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOMULW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DTOCYC"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOMULW::_1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(DTOMULW::_16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(DTOMULW::_128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(DTOMULW::_256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(DTOMULW::_1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline]
    pub fn _4096(self) -> &'a mut W {
        self.variant(DTOMULW::_4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline]
    pub fn _65536(self) -> &'a mut W {
        self.variant(DTOMULW::_65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline]
    pub fn _1048576(self) -> &'a mut W {
        self.variant(DTOMULW::_1048576)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline]
    pub fn dtocyc(&self) -> DTOCYCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTOCYCR { bits }
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline]
    pub fn dtomul(&self) -> DTOMULR {
        DTOMULR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline]
    pub fn dtocyc(&mut self) -> _DTOCYCW {
        _DTOCYCW { w: self }
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline]
    pub fn dtomul(&mut self) -> _DTOMULW {
        _DTOMULW { w: self }
    }
}
