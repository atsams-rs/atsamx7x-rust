#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_SEQ1R {
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
pub struct USCH0R {
    bits: u8,
}
impl USCH0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH1R {
    bits: u8,
}
impl USCH1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH2R {
    bits: u8,
}
impl USCH2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH3R {
    bits: u8,
}
impl USCH3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH4R {
    bits: u8,
}
impl USCH4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH5R {
    bits: u8,
}
impl USCH5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH6R {
    bits: u8,
}
impl USCH6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH7R {
    bits: u8,
}
impl USCH7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _USCH0W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH0W<'a> {
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
#[doc = r" Proxy"]
pub struct _USCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH2W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH3W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH4W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH4W<'a> {
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
pub struct _USCH5W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH6W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH7W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline]
    pub fn usch0(&self) -> USCH0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH0R { bits }
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline]
    pub fn usch1(&self) -> USCH1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH1R { bits }
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline]
    pub fn usch2(&self) -> USCH2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH2R { bits }
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline]
    pub fn usch3(&self) -> USCH3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH3R { bits }
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline]
    pub fn usch4(&self) -> USCH4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH4R { bits }
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline]
    pub fn usch5(&self) -> USCH5R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH5R { bits }
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline]
    pub fn usch6(&self) -> USCH6R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH6R { bits }
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline]
    pub fn usch7(&self) -> USCH7R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH7R { bits }
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
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline]
    pub fn usch0(&mut self) -> _USCH0W {
        _USCH0W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline]
    pub fn usch1(&mut self) -> _USCH1W {
        _USCH1W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline]
    pub fn usch2(&mut self) -> _USCH2W {
        _USCH2W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline]
    pub fn usch3(&mut self) -> _USCH3W {
        _USCH3W { w: self }
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline]
    pub fn usch4(&mut self) -> _USCH4W {
        _USCH4W { w: self }
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline]
    pub fn usch5(&mut self) -> _USCH5W {
        _USCH5W { w: self }
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline]
    pub fn usch6(&mut self) -> _USCH6W {
        _USCH6W { w: self }
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline]
    pub fn usch7(&mut self) -> _USCH7W {
        _USCH7W { w: self }
    }
}
