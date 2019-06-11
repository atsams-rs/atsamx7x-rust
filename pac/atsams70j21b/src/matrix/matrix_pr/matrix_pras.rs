#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MATRIX_PRAS {
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
pub struct M0PRR {
    bits: u8,
}
impl M0PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M1PRR {
    bits: u8,
}
impl M1PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M2PRR {
    bits: u8,
}
impl M2PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M3PRR {
    bits: u8,
}
impl M3PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M4PRR {
    bits: u8,
}
impl M4PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M5PRR {
    bits: u8,
}
impl M5PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M6PRR {
    bits: u8,
}
impl M6PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _M0PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M0PRW<'a> {
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
#[doc = r" Proxy"]
pub struct _M1PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M1PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M2PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M2PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M3PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M3PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M4PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M4PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M5PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M5PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M6PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M6PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline]
    pub fn m0pr(&self) -> M0PRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M0PRR { bits }
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline]
    pub fn m1pr(&self) -> M1PRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M1PRR { bits }
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline]
    pub fn m2pr(&self) -> M2PRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M2PRR { bits }
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline]
    pub fn m3pr(&self) -> M3PRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M3PRR { bits }
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline]
    pub fn m4pr(&self) -> M4PRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M4PRR { bits }
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline]
    pub fn m5pr(&self) -> M5PRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M5PRR { bits }
    }
    #[doc = "Bits 24:25 - Master 6 Priority"]
    #[inline]
    pub fn m6pr(&self) -> M6PRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M6PRR { bits }
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
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline]
    pub fn m0pr(&mut self) -> _M0PRW {
        _M0PRW { w: self }
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline]
    pub fn m1pr(&mut self) -> _M1PRW {
        _M1PRW { w: self }
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline]
    pub fn m2pr(&mut self) -> _M2PRW {
        _M2PRW { w: self }
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline]
    pub fn m3pr(&mut self) -> _M3PRW {
        _M3PRW { w: self }
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline]
    pub fn m4pr(&mut self) -> _M4PRW {
        _M4PRW { w: self }
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline]
    pub fn m5pr(&mut self) -> _M5PRW {
        _M5PRW { w: self }
    }
    #[doc = "Bits 24:25 - Master 6 Priority"]
    #[inline]
    pub fn m6pr(&mut self) -> _M6PRW {
        _M6PRW { w: self }
    }
}
