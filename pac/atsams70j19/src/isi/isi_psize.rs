#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISI_PSIZE {
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
pub type PREV_VSIZE_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _PREV_VSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PREV_VSIZEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PREV_HSIZE_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _PREV_HSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PREV_HSIZEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&self) -> PREV_VSIZE_R {
        PREV_VSIZE_R::new((self.bits() & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&self) -> PREV_HSIZE_R {
        PREV_HSIZE_R::new(((self.bits() >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&mut self) -> _PREV_VSIZEW {
        _PREV_VSIZEW { w: self }
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&mut self) -> _PREV_HSIZEW {
        _PREV_HSIZEW { w: self }
    }
}
