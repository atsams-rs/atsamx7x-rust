#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPI_TDR {
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
}
#[doc = r" Proxy"]
pub struct _TDW<'a> {
    w: &'a mut W,
}
impl<'a> _TDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSW {
    #[doc = "NPCS0 as Chip Select"]
    NPCS0,
    #[doc = "NPCS1 as Chip Select"]
    NPCS1,
    #[doc = "NPCS2 as Chip Select"]
    NPCS2,
    #[doc = "NPCS3 as Chip Select"]
    NPCS3,
}
impl PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSW::NPCS0 => 14,
            PCSW::NPCS1 => 13,
            PCSW::NPCS2 => 11,
            PCSW::NPCS3 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NPCS0 as Chip Select"]
    #[inline]
    pub fn npcs0(self) -> &'a mut W {
        self.variant(PCSW::NPCS0)
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline]
    pub fn npcs1(self) -> &'a mut W {
        self.variant(PCSW::NPCS1)
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline]
    pub fn npcs2(self) -> &'a mut W {
        self.variant(PCSW::NPCS2)
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline]
    pub fn npcs3(self) -> &'a mut W {
        self.variant(PCSW::NPCS3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LASTXFERW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTXFERW<'a> {
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
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline]
    pub fn td(&mut self) -> _TDW {
        _TDW { w: self }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline]
    pub fn lastxfer(&mut self) -> _LASTXFERW {
        _LASTXFERW { w: self }
    }
}
