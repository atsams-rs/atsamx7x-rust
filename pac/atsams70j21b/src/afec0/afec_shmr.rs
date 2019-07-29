#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_SHMR {
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
pub type DUAL0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL0W<'a> {
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
pub type DUAL1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL1W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL1W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL2W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL2W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL3W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL4W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL4W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL5W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL5W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL6W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL6W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL7W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL7W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL8W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL8W<'a> {
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
#[doc = r"Reader of the field"]
pub type DUAL9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL9W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL9W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL10W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL10W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DUAL11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUAL11W<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL11W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&self) -> DUAL0_R {
        DUAL0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&self) -> DUAL1_R {
        DUAL1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&self) -> DUAL2_R {
        DUAL2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&self) -> DUAL3_R {
        DUAL3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&self) -> DUAL4_R {
        DUAL4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&self) -> DUAL5_R {
        DUAL5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&self) -> DUAL6_R {
        DUAL6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&self) -> DUAL7_R {
        DUAL7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&self) -> DUAL8_R {
        DUAL8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&self) -> DUAL9_R {
        DUAL9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&self) -> DUAL10_R {
        DUAL10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&self) -> DUAL11_R {
        DUAL11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&mut self) -> _DUAL0W {
        _DUAL0W { w: self }
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&mut self) -> _DUAL1W {
        _DUAL1W { w: self }
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&mut self) -> _DUAL2W {
        _DUAL2W { w: self }
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&mut self) -> _DUAL3W {
        _DUAL3W { w: self }
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&mut self) -> _DUAL4W {
        _DUAL4W { w: self }
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&mut self) -> _DUAL5W {
        _DUAL5W { w: self }
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&mut self) -> _DUAL6W {
        _DUAL6W { w: self }
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&mut self) -> _DUAL7W {
        _DUAL7W { w: self }
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&mut self) -> _DUAL8W {
        _DUAL8W { w: self }
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&mut self) -> _DUAL9W {
        _DUAL9W { w: self }
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&mut self) -> _DUAL10W {
        _DUAL10W { w: self }
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&mut self) -> _DUAL11W {
        _DUAL11W { w: self }
    }
}
