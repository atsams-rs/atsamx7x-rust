#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBHS_CTRL {
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
pub type RDERRE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RDERREW<'a> {
    w: &'a mut W,
}
impl<'a> _RDERREW<'a> {
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
pub type VBUSHWC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _VBUSHWCW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSHWCW<'a> {
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
pub type FRZCLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FRZCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FRZCLKW<'a> {
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
#[doc = r"Reader of the field"]
pub type USBE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USBEW<'a> {
    w: &'a mut W,
}
impl<'a> _USBEW<'a> {
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
#[doc = "Possible values of the field `UIMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIMODR {
    #[doc = "The module is in USB Host mode."]
    HOST,
    #[doc = "The module is in USB Device mode."]
    DEVICE,
}
impl crate::ToBits<bool> for UIMODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UIMODR::HOST => false,
            UIMODR::DEVICE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UIMOD_R = crate::FR<bool, UIMODR>;
impl UIMOD_R {
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == UIMODR::HOST
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == UIMODR::DEVICE
    }
}
#[doc = "Values that can be written to the field `UIMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIMODW {
    #[doc = "The module is in USB Host mode."]
    HOST,
    #[doc = "The module is in USB Device mode."]
    DEVICE,
}
impl UIMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UIMODW::HOST => false,
            UIMODW::DEVICE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UIMODW<'a> {
    w: &'a mut W,
}
impl<'a> _UIMODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The module is in USB Host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMODW::HOST)
    }
    #[doc = "The module is in USB Device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMODW::DEVICE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn rderre(&self) -> RDERRE_R {
        RDERRE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VBUSHWC_R {
        VBUSHWC_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn rderre(&mut self) -> _RDERREW {
        _RDERREW { w: self }
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&mut self) -> _VBUSHWCW {
        _VBUSHWCW { w: self }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&mut self) -> _FRZCLKW {
        _FRZCLKW { w: self }
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> _USBEW {
        _USBEW { w: self }
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    pub fn uimod(&mut self) -> _UIMODW {
        _UIMODW { w: self }
    }
}
