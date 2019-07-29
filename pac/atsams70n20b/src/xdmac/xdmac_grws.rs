#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_GRWS {
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
pub struct _RWS0W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS0W<'a> {
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
pub struct _RWS1W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS1W<'a> {
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
pub struct _RWS2W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS2W<'a> {
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
pub struct _RWS3W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS3W<'a> {
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
pub struct _RWS4W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS4W<'a> {
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
pub struct _RWS5W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS5W<'a> {
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
pub struct _RWS6W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS6W<'a> {
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
pub struct _RWS7W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS7W<'a> {
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
pub struct _RWS8W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS8W<'a> {
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
pub struct _RWS9W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS9W<'a> {
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
pub struct _RWS10W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS10W<'a> {
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
pub struct _RWS11W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS11W<'a> {
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
pub struct _RWS12W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS12W<'a> {
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
pub struct _RWS13W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS13W<'a> {
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
pub struct _RWS14W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS14W<'a> {
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
pub struct _RWS15W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS15W<'a> {
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
pub struct _RWS16W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS16W<'a> {
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
pub struct _RWS17W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS17W<'a> {
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
pub struct _RWS18W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS18W<'a> {
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
pub struct _RWS19W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS19W<'a> {
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
pub struct _RWS20W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS20W<'a> {
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
pub struct _RWS21W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS21W<'a> {
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
pub struct _RWS22W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS22W<'a> {
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
pub struct _RWS23W<'a> {
    w: &'a mut W,
}
impl<'a> _RWS23W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws0(&mut self) -> _RWS0W {
        _RWS0W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws1(&mut self) -> _RWS1W {
        _RWS1W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws2(&mut self) -> _RWS2W {
        _RWS2W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws3(&mut self) -> _RWS3W {
        _RWS3W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws4(&mut self) -> _RWS4W {
        _RWS4W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws5(&mut self) -> _RWS5W {
        _RWS5W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws6(&mut self) -> _RWS6W {
        _RWS6W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws7(&mut self) -> _RWS7W {
        _RWS7W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws8(&mut self) -> _RWS8W {
        _RWS8W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws9(&mut self) -> _RWS9W {
        _RWS9W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws10(&mut self) -> _RWS10W {
        _RWS10W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws11(&mut self) -> _RWS11W {
        _RWS11W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws12(&mut self) -> _RWS12W {
        _RWS12W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws13(&mut self) -> _RWS13W {
        _RWS13W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws14(&mut self) -> _RWS14W {
        _RWS14W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws15(&mut self) -> _RWS15W {
        _RWS15W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws16(&mut self) -> _RWS16W {
        _RWS16W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws17(&mut self) -> _RWS17W {
        _RWS17W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws18(&mut self) -> _RWS18W {
        _RWS18W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws19(&mut self) -> _RWS19W {
        _RWS19W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws20(&mut self) -> _RWS20W {
        _RWS20W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws21(&mut self) -> _RWS21W {
        _RWS21W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws22(&mut self) -> _RWS22W {
        _RWS22W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws23(&mut self) -> _RWS23W {
        _RWS23W { w: self }
    }
}
