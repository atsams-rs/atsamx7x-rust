#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_WPCR {
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
#[doc = "Values that can be written to the field `WPCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPCMDW {
    #[doc = "Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    DISABLE_SW_PROT,
    #[doc = "Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    ENABLE_SW_PROT,
    #[doc = "Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    ENABLE_HW_PROT,
}
impl WPCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            WPCMDW::DISABLE_SW_PROT => 0,
            WPCMDW::ENABLE_SW_PROT => 1,
            WPCMDW::ENABLE_HW_PROT => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WPCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _WPCMDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn disable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMDW::DISABLE_SW_PROT)
    }
    #[doc = "Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn enable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMDW::ENABLE_SW_PROT)
    }
    #[doc = "Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    #[inline(always)]
    pub fn enable_hw_prot(self) -> &'a mut W {
        self.variant(WPCMDW::ENABLE_HW_PROT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _WPRG0W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG0W<'a> {
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
pub struct _WPRG1W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG1W<'a> {
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
pub struct _WPRG2W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG2W<'a> {
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
pub struct _WPRG3W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG3W<'a> {
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
pub struct _WPRG4W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG4W<'a> {
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
pub struct _WPRG5W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG5W<'a> {
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
#[doc = "Values that can be written to the field `WPKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPKEYW {
    #[doc = "Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    PASSWD,
}
impl WPKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u32 {
        match *self {
            WPKEYW::PASSWD => 5265229,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WPKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _WPKEYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEYW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
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
    #[doc = "Bits 0:1 - Write Protection Command"]
    #[inline(always)]
    pub fn wpcmd(&mut self) -> _WPCMDW {
        _WPCMDW { w: self }
    }
    #[doc = "Bit 2 - Write Protection Register Group 0"]
    #[inline(always)]
    pub fn wprg0(&mut self) -> _WPRG0W {
        _WPRG0W { w: self }
    }
    #[doc = "Bit 3 - Write Protection Register Group 1"]
    #[inline(always)]
    pub fn wprg1(&mut self) -> _WPRG1W {
        _WPRG1W { w: self }
    }
    #[doc = "Bit 4 - Write Protection Register Group 2"]
    #[inline(always)]
    pub fn wprg2(&mut self) -> _WPRG2W {
        _WPRG2W { w: self }
    }
    #[doc = "Bit 5 - Write Protection Register Group 3"]
    #[inline(always)]
    pub fn wprg3(&mut self) -> _WPRG3W {
        _WPRG3W { w: self }
    }
    #[doc = "Bit 6 - Write Protection Register Group 4"]
    #[inline(always)]
    pub fn wprg4(&mut self) -> _WPRG4W {
        _WPRG4W { w: self }
    }
    #[doc = "Bit 7 - Write Protection Register Group 5"]
    #[inline(always)]
    pub fn wprg5(&mut self) -> _WPRG5W {
        _WPRG5W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> _WPKEYW {
        _WPKEYW { w: self }
    }
}
