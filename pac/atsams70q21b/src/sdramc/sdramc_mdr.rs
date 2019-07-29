#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_MDR {
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
#[doc = "Possible values of the field `MD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDR {
    #[doc = "SDRAM"]
    SDRAM,
    #[doc = "Low-power SDRAM"]
    LPSDRAM,
}
impl crate::ToBits<u8> for MDR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MDR::SDRAM => 0,
            MDR::LPSDRAM => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MD_R = crate::FR<u8, MDR>;
impl MD_R {
    #[doc = "Checks if the value of the field is `SDRAM`"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == MDR::SDRAM
    }
    #[doc = "Checks if the value of the field is `LPSDRAM`"]
    #[inline(always)]
    pub fn is_lpsdram(&self) -> bool {
        *self == MDR::LPSDRAM
    }
}
#[doc = "Values that can be written to the field `MD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDW {
    #[doc = "SDRAM"]
    SDRAM,
    #[doc = "Low-power SDRAM"]
    LPSDRAM,
}
impl MDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDW::SDRAM => 0,
            MDW::LPSDRAM => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MDW<'a> {
    w: &'a mut W,
}
impl<'a> _MDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MDW::SDRAM)
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn lpsdram(self) -> &'a mut W {
        self.variant(MDW::LPSDRAM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&mut self) -> _MDW {
        _MDW { w: self }
    }
}
