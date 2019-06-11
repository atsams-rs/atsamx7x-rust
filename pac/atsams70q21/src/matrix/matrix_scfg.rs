#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MATRIX_SCFG {
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
pub struct SLOT_CYCLER {
    bits: u16,
}
impl SLOT_CYCLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DEFMSTR_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEFMSTR_TYPER {
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    NONE,
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    LAST,
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    FIXED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEFMSTR_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEFMSTR_TYPER::NONE => 0,
            DEFMSTR_TYPER::LAST => 1,
            DEFMSTR_TYPER::FIXED => 2,
            DEFMSTR_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEFMSTR_TYPER {
        match value {
            0 => DEFMSTR_TYPER::NONE,
            1 => DEFMSTR_TYPER::LAST,
            2 => DEFMSTR_TYPER::FIXED,
            i => DEFMSTR_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DEFMSTR_TYPER::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline]
    pub fn is_last(&self) -> bool {
        *self == DEFMSTR_TYPER::LAST
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline]
    pub fn is_fixed(&self) -> bool {
        *self == DEFMSTR_TYPER::FIXED
    }
}
#[doc = r" Value of the field"]
pub struct FIXED_DEFMSTRR {
    bits: u8,
}
impl FIXED_DEFMSTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SLOT_CYCLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOT_CYCLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEFMSTR_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEFMSTR_TYPEW {
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    NONE,
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    LAST,
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    FIXED,
}
impl DEFMSTR_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEFMSTR_TYPEW::NONE => 0,
            DEFMSTR_TYPEW::LAST => 1,
            DEFMSTR_TYPEW::FIXED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEFMSTR_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEFMSTR_TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEFMSTR_TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPEW::NONE)
    }
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    #[inline]
    pub fn last(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPEW::LAST)
    }
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPEW::FIXED)
    }
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
pub struct _FIXED_DEFMSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _FIXED_DEFMSTRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline]
    pub fn slot_cycle(&self) -> SLOT_CYCLER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SLOT_CYCLER { bits }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPER {
        DEFMSTR_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIXED_DEFMSTRR { bits }
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
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline]
    pub fn slot_cycle(&mut self) -> _SLOT_CYCLEW {
        _SLOT_CYCLEW { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline]
    pub fn defmstr_type(&mut self) -> _DEFMSTR_TYPEW {
        _DEFMSTR_TYPEW { w: self }
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline]
    pub fn fixed_defmstr(&mut self) -> _FIXED_DEFMSTRW {
        _FIXED_DEFMSTRW { w: self }
    }
}
