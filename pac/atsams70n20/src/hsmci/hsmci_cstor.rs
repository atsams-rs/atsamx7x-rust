#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSMCI_CSTOR {
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
pub type CSTOCYC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CSTOCYCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTOCYCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `CSTOMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTOMULR {
    #[doc = "CSTOCYC x 1"]
    _1,
    #[doc = "CSTOCYC x 16"]
    _16,
    #[doc = "CSTOCYC x 128"]
    _128,
    #[doc = "CSTOCYC x 256"]
    _256,
    #[doc = "CSTOCYC x 1024"]
    _1024,
    #[doc = "CSTOCYC x 4096"]
    _4096,
    #[doc = "CSTOCYC x 65536"]
    _65536,
    #[doc = "CSTOCYC x 1048576"]
    _1048576,
}
impl crate::ToBits<u8> for CSTOMULR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CSTOMULR::_1 => 0,
            CSTOMULR::_16 => 1,
            CSTOMULR::_128 => 2,
            CSTOMULR::_256 => 3,
            CSTOMULR::_1024 => 4,
            CSTOMULR::_4096 => 5,
            CSTOMULR::_65536 => 6,
            CSTOMULR::_1048576 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CSTOMUL_R = crate::FR<u8, CSTOMULR>;
impl CSTOMUL_R {
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOMULR::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CSTOMULR::_16
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == CSTOMULR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == CSTOMULR::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == CSTOMULR::_1024
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == CSTOMULR::_4096
    }
    #[doc = "Checks if the value of the field is `_65536`"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == CSTOMULR::_65536
    }
    #[doc = "Checks if the value of the field is `_1048576`"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        *self == CSTOMULR::_1048576
    }
}
#[doc = "Values that can be written to the field `CSTOMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTOMULW {
    #[doc = "CSTOCYC x 1"]
    _1,
    #[doc = "CSTOCYC x 16"]
    _16,
    #[doc = "CSTOCYC x 128"]
    _128,
    #[doc = "CSTOCYC x 256"]
    _256,
    #[doc = "CSTOCYC x 1024"]
    _1024,
    #[doc = "CSTOCYC x 4096"]
    _4096,
    #[doc = "CSTOCYC x 65536"]
    _65536,
    #[doc = "CSTOCYC x 1048576"]
    _1048576,
}
impl CSTOMULW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSTOMULW::_1 => 0,
            CSTOMULW::_16 => 1,
            CSTOMULW::_128 => 2,
            CSTOMULW::_256 => 3,
            CSTOMULW::_1024 => 4,
            CSTOMULW::_4096 => 5,
            CSTOMULW::_65536 => 6,
            CSTOMULW::_1048576 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CSTOMULW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTOMULW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSTOMULW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSTOCYC x 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOMULW::_1)
    }
    #[doc = "CSTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CSTOMULW::_16)
    }
    #[doc = "CSTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(CSTOMULW::_128)
    }
    #[doc = "CSTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(CSTOMULW::_256)
    }
    #[doc = "CSTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(CSTOMULW::_1024)
    }
    #[doc = "CSTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(CSTOMULW::_4096)
    }
    #[doc = "CSTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut W {
        self.variant(CSTOMULW::_65536)
    }
    #[doc = "CSTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut W {
        self.variant(CSTOMULW::_1048576)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&self) -> CSTOCYC_R {
        CSTOCYC_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&self) -> CSTOMUL_R {
        CSTOMUL_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&mut self) -> _CSTOCYCW {
        _CSTOCYCW { w: self }
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&mut self) -> _CSTOMULW {
        _CSTOMULW { w: self }
    }
}
