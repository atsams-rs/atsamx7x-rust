#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACC_ACR {
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
#[doc = "Possible values of the field `ISEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISELR {
    #[doc = "Low-power option."]
    LOPW,
    #[doc = "High-speed option."]
    HISP,
}
impl crate::ToBits<bool> for ISELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ISELR::LOPW => false,
            ISELR::HISP => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ISEL_R = crate::FR<bool, ISELR>;
impl ISEL_R {
    #[doc = "Checks if the value of the field is `LOPW`"]
    #[inline(always)]
    pub fn is_lopw(&self) -> bool {
        *self == ISELR::LOPW
    }
    #[doc = "Checks if the value of the field is `HISP`"]
    #[inline(always)]
    pub fn is_hisp(&self) -> bool {
        *self == ISELR::HISP
    }
}
#[doc = "Values that can be written to the field `ISEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISELW {
    #[doc = "Low-power option."]
    LOPW,
    #[doc = "High-speed option."]
    HISP,
}
impl ISELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ISELW::LOPW => false,
            ISELW::HISP => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ISELW<'a> {
    w: &'a mut W,
}
impl<'a> _ISELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low-power option."]
    #[inline(always)]
    pub fn lopw(self) -> &'a mut W {
        self.variant(ISELW::LOPW)
    }
    #[doc = "High-speed option."]
    #[inline(always)]
    pub fn hisp(self) -> &'a mut W {
        self.variant(ISELW::HISP)
    }
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
pub type HYST_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&mut self) -> _ISELW {
        _ISELW { w: self }
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
}
