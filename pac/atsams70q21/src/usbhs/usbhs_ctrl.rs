#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBHS_CTRL {
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
pub struct RDERRER {
    bits: bool,
}
impl RDERRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VBUSHWCR {
    bits: bool,
}
impl VBUSHWCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FRZCLKR {
    bits: bool,
}
impl FRZCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USBER {
    bits: bool,
}
impl USBER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
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
impl UIMODR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UIMODR::HOST => false,
            UIMODR::DEVICE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIMODR {
        match value {
            false => UIMODR::HOST,
            true => UIMODR::DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline]
    pub fn is_host(&self) -> bool {
        *self == UIMODR::HOST
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline]
    pub fn is_device(&self) -> bool {
        *self == UIMODR::DEVICE
    }
}
#[doc = r" Proxy"]
pub struct _RDERREW<'a> {
    w: &'a mut W,
}
impl<'a> _RDERREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBUSHWCW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSHWCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRZCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FRZCLKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBEW<'a> {
    w: &'a mut W,
}
impl<'a> _USBEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIMODW::HOST => false,
            UIMODW::DEVICE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIMODW<'a> {
    w: &'a mut W,
}
impl<'a> _UIMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The module is in USB Host mode."]
    #[inline]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMODW::HOST)
    }
    #[doc = "The module is in USB Device mode."]
    #[inline]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMODW::DEVICE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline]
    pub fn rderre(&self) -> RDERRER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDERRER { bits }
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline]
    pub fn vbushwc(&self) -> VBUSHWCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSHWCR { bits }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline]
    pub fn frzclk(&self) -> FRZCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRZCLKR { bits }
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline]
    pub fn usbe(&self) -> USBER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBER { bits }
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline]
    pub fn uimod(&self) -> UIMODR {
        UIMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline]
    pub fn rderre(&mut self) -> _RDERREW {
        _RDERREW { w: self }
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline]
    pub fn vbushwc(&mut self) -> _VBUSHWCW {
        _VBUSHWCW { w: self }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline]
    pub fn frzclk(&mut self) -> _FRZCLKW {
        _FRZCLKW { w: self }
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline]
    pub fn usbe(&mut self) -> _USBEW {
        _USBEW { w: self }
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline]
    pub fn uimod(&mut self) -> _UIMODW {
        _UIMODW { w: self }
    }
}
