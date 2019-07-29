#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_GRWR {
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
#[doc = r"Proxy"]
pub struct _RWR0W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR0W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR1W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR1W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR2W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR2W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR3W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR3W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR4W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR4W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR5W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR5W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR6W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR6W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR7W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR7W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR8W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR8W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR9W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR9W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR10W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR10W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR11W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR11W<'a> {
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
#[doc = r"Proxy"]
pub struct _RWR12W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR12W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR13W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR13W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR14W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR14W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR15W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR15W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR16W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR16W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR17W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR17W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR18W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR18W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR19W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR19W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR20W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR20W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR21W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR21W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR22W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR22W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWR23W<'a> {
    w: &'a mut W,
}
impl<'a> _RWR23W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr0(&mut self) -> _RWR0W {
        _RWR0W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr1(&mut self) -> _RWR1W {
        _RWR1W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr2(&mut self) -> _RWR2W {
        _RWR2W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr3(&mut self) -> _RWR3W {
        _RWR3W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr4(&mut self) -> _RWR4W {
        _RWR4W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr5(&mut self) -> _RWR5W {
        _RWR5W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr6(&mut self) -> _RWR6W {
        _RWR6W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr7(&mut self) -> _RWR7W {
        _RWR7W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr8(&mut self) -> _RWR8W {
        _RWR8W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr9(&mut self) -> _RWR9W {
        _RWR9W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr10(&mut self) -> _RWR10W {
        _RWR10W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr11(&mut self) -> _RWR11W {
        _RWR11W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr12(&mut self) -> _RWR12W {
        _RWR12W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr13(&mut self) -> _RWR13W {
        _RWR13W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr14(&mut self) -> _RWR14W {
        _RWR14W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr15(&mut self) -> _RWR15W {
        _RWR15W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr16(&mut self) -> _RWR16W {
        _RWR16W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr17(&mut self) -> _RWR17W {
        _RWR17W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr18(&mut self) -> _RWR18W {
        _RWR18W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr19(&mut self) -> _RWR19W {
        _RWR19W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr20(&mut self) -> _RWR20W {
        _RWR20W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr21(&mut self) -> _RWR21W {
        _RWR21W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr22(&mut self) -> _RWR22W {
        _RWR22W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr23(&mut self) -> _RWR23W {
        _RWR23W { w: self }
    }
}
