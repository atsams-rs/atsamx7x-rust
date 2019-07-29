#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_CFR1 {
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
pub type TMRD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TMRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `UNAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALR {
    #[doc = "Unaligned access is not supported."]
    UNSUPPORTED,
    #[doc = "Unaligned access is supported."]
    SUPPORTED,
}
impl crate::ToBits<bool> for UNALR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNALR::UNSUPPORTED => false,
            UNALR::SUPPORTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UNAL_R = crate::FR<bool, UNALR>;
impl UNAL_R {
    #[doc = "Checks if the value of the field is `UNSUPPORTED`"]
    #[inline(always)]
    pub fn is_unsupported(&self) -> bool {
        *self == UNALR::UNSUPPORTED
    }
    #[doc = "Checks if the value of the field is `SUPPORTED`"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == UNALR::SUPPORTED
    }
}
#[doc = "Values that can be written to the field `UNAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALW {
    #[doc = "Unaligned access is not supported."]
    UNSUPPORTED,
    #[doc = "Unaligned access is supported."]
    SUPPORTED,
}
impl UNALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALW::UNSUPPORTED => false,
            UNALW::SUPPORTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UNALW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Unaligned access is not supported."]
    #[inline(always)]
    pub fn unsupported(self) -> &'a mut W {
        self.variant(UNALW::UNSUPPORTED)
    }
    #[doc = "Unaligned access is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut W {
        self.variant(UNALW::SUPPORTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&self) -> UNAL_R {
        UNAL_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> _TMRDW {
        _TMRDW { w: self }
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&mut self) -> _UNALW {
        _UNALW { w: self }
    }
}
