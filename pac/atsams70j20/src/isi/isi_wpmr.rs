#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISI_WPMR {
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
pub struct WPENR {
    bits: bool,
}
impl WPENR {
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
#[doc = "Possible values of the field `WPKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPKEYR {
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit.Always reads as 0."]
    PASSWD,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl WPKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            WPKEYR::PASSWD => 4805449,
            WPKEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> WPKEYR {
        match value {
            4805449 => WPKEYR::PASSWD,
            i => WPKEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline]
    pub fn is_passwd(&self) -> bool {
        *self == WPKEYR::PASSWD
    }
}
#[doc = r" Proxy"]
pub struct _WPENW<'a> {
    w: &'a mut W,
}
impl<'a> _WPENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WPKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPKEYW {
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit.Always reads as 0."]
    PASSWD,
}
impl WPKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            WPKEYW::PASSWD => 4805449,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _WPKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit.Always reads as 0."]
    #[inline]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEYW::PASSWD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline]
    pub fn wpen(&self) -> WPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WPENR { bits }
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline]
    pub fn wpkey(&self) -> WPKEYR {
        WPKEYR::_from({
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline]
    pub fn wpen(&mut self) -> _WPENW {
        _WPENW { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline]
    pub fn wpkey(&mut self) -> _WPKEYW {
        _WPKEYW { w: self }
    }
}
