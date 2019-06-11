#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_MDR {
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
#[doc = "Possible values of the field `MD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDR {
    #[doc = "SDRAM"]
    SDRAM,
    #[doc = "Low-power SDRAM"]
    LPSDRAM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MDR::SDRAM => 0,
            MDR::LPSDRAM => 1,
            MDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MDR {
        match value {
            0 => MDR::SDRAM,
            1 => MDR::LPSDRAM,
            i => MDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDRAM`"]
    #[inline]
    pub fn is_sdram(&self) -> bool {
        *self == MDR::SDRAM
    }
    #[doc = "Checks if the value of the field is `LPSDRAM`"]
    #[inline]
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDW::SDRAM => 0,
            MDW::LPSDRAM => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDW<'a> {
    w: &'a mut W,
}
impl<'a> _MDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDRAM"]
    #[inline]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MDW::SDRAM)
    }
    #[doc = "Low-power SDRAM"]
    #[inline]
    pub fn lpsdram(self) -> &'a mut W {
        self.variant(MDW::LPSDRAM)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline]
    pub fn md(&self) -> MDR {
        MDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline]
    pub fn md(&mut self) -> _MDW {
        _MDW { w: self }
    }
}
