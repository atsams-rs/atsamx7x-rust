#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_LONMR {
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
pub type COMMT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _COMMTW<'a> {
    w: &'a mut W,
}
impl<'a> _COMMTW<'a> {
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
pub type COLDET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _COLDETW<'a> {
    w: &'a mut W,
}
impl<'a> _COLDETW<'a> {
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
pub type TCOL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TCOLW<'a> {
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
pub type CDTAIL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CDTAILW<'a> {
    w: &'a mut W,
}
impl<'a> _CDTAILW<'a> {
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
pub type DMAM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMAMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMW<'a> {
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
pub type LCDS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDSW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDSW<'a> {
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
pub type EOFS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _EOFSW<'a> {
    w: &'a mut W,
}
impl<'a> _EOFSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    pub fn commt(&self) -> COMMT_R {
        COMMT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    pub fn coldet(&self) -> COLDET_R {
        COLDET_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    pub fn tcol(&self) -> TCOL_R {
        TCOL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    pub fn cdtail(&self) -> CDTAIL_R {
        CDTAIL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    pub fn dmam(&self) -> DMAM_R {
        DMAM_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    pub fn lcds(&self) -> LCDS_R {
        LCDS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    pub fn eofs(&self) -> EOFS_R {
        EOFS_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    pub fn commt(&mut self) -> _COMMTW {
        _COMMTW { w: self }
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    pub fn coldet(&mut self) -> _COLDETW {
        _COLDETW { w: self }
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    pub fn tcol(&mut self) -> _TCOLW {
        _TCOLW { w: self }
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    pub fn cdtail(&mut self) -> _CDTAILW {
        _CDTAILW { w: self }
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    pub fn dmam(&mut self) -> _DMAMW {
        _DMAMW { w: self }
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    pub fn lcds(&mut self) -> _LCDSW {
        _LCDSW { w: self }
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    pub fn eofs(&mut self) -> _EOFSW {
        _EOFSW { w: self }
    }
}
