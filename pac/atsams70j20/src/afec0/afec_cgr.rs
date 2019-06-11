#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_CGR {
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
pub struct GAIN0R {
    bits: u8,
}
impl GAIN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN1R {
    bits: u8,
}
impl GAIN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN2R {
    bits: u8,
}
impl GAIN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN3R {
    bits: u8,
}
impl GAIN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN4R {
    bits: u8,
}
impl GAIN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN5R {
    bits: u8,
}
impl GAIN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN6R {
    bits: u8,
}
impl GAIN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN7R {
    bits: u8,
}
impl GAIN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN8R {
    bits: u8,
}
impl GAIN8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN9R {
    bits: u8,
}
impl GAIN9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN10R {
    bits: u8,
}
impl GAIN10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAIN11R {
    bits: u8,
}
impl GAIN11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _GAIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN0W<'a> {
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
pub struct _GAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN2W<'a> {
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
pub struct _GAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN4W<'a> {
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
pub struct _GAIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN6W<'a> {
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
pub struct _GAIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAIN8W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN8W<'a> {
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
pub struct _GAIN9W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAIN10W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN10W<'a> {
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
pub struct _GAIN11W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline]
    pub fn gain0(&self) -> GAIN0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN0R { bits }
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline]
    pub fn gain1(&self) -> GAIN1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN1R { bits }
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline]
    pub fn gain2(&self) -> GAIN2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN2R { bits }
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline]
    pub fn gain3(&self) -> GAIN3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN3R { bits }
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline]
    pub fn gain4(&self) -> GAIN4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN4R { bits }
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline]
    pub fn gain5(&self) -> GAIN5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN5R { bits }
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline]
    pub fn gain6(&self) -> GAIN6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN6R { bits }
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline]
    pub fn gain7(&self) -> GAIN7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN7R { bits }
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline]
    pub fn gain8(&self) -> GAIN8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN8R { bits }
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline]
    pub fn gain9(&self) -> GAIN9R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN9R { bits }
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline]
    pub fn gain10(&self) -> GAIN10R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN10R { bits }
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline]
    pub fn gain11(&self) -> GAIN11R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAIN11R { bits }
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
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline]
    pub fn gain0(&mut self) -> _GAIN0W {
        _GAIN0W { w: self }
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline]
    pub fn gain1(&mut self) -> _GAIN1W {
        _GAIN1W { w: self }
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline]
    pub fn gain2(&mut self) -> _GAIN2W {
        _GAIN2W { w: self }
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline]
    pub fn gain3(&mut self) -> _GAIN3W {
        _GAIN3W { w: self }
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline]
    pub fn gain4(&mut self) -> _GAIN4W {
        _GAIN4W { w: self }
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline]
    pub fn gain5(&mut self) -> _GAIN5W {
        _GAIN5W { w: self }
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline]
    pub fn gain6(&mut self) -> _GAIN6W {
        _GAIN6W { w: self }
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline]
    pub fn gain7(&mut self) -> _GAIN7W {
        _GAIN7W { w: self }
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline]
    pub fn gain8(&mut self) -> _GAIN8W {
        _GAIN8W { w: self }
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline]
    pub fn gain9(&mut self) -> _GAIN9W {
        _GAIN9W { w: self }
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline]
    pub fn gain10(&mut self) -> _GAIN10W {
        _GAIN10W { w: self }
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline]
    pub fn gain11(&mut self) -> _GAIN11W {
        _GAIN11W { w: self }
    }
}
