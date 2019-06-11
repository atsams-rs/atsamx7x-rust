#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPC_SMMR {
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
pub struct SMTHR {
    bits: u8,
}
impl SMTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SMSMPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSMPLR {
    #[doc = "Supply Monitor disabled"]
    SMD,
    #[doc = "Continuous Supply Monitor"]
    CSM,
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMSMPLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMSMPLR::SMD => 0,
            SMSMPLR::CSM => 1,
            SMSMPLR::_32SLCK => 2,
            SMSMPLR::_256SLCK => 3,
            SMSMPLR::_2048SLCK => 4,
            SMSMPLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMSMPLR {
        match value {
            0 => SMSMPLR::SMD,
            1 => SMSMPLR::CSM,
            2 => SMSMPLR::_32SLCK,
            3 => SMSMPLR::_256SLCK,
            4 => SMSMPLR::_2048SLCK,
            i => SMSMPLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SMD`"]
    #[inline]
    pub fn is_smd(&self) -> bool {
        *self == SMSMPLR::SMD
    }
    #[doc = "Checks if the value of the field is `CSM`"]
    #[inline]
    pub fn is_csm(&self) -> bool {
        *self == SMSMPLR::CSM
    }
    #[doc = "Checks if the value of the field is `_32SLCK`"]
    #[inline]
    pub fn is_32slck(&self) -> bool {
        *self == SMSMPLR::_32SLCK
    }
    #[doc = "Checks if the value of the field is `_256SLCK`"]
    #[inline]
    pub fn is_256slck(&self) -> bool {
        *self == SMSMPLR::_256SLCK
    }
    #[doc = "Checks if the value of the field is `_2048SLCK`"]
    #[inline]
    pub fn is_2048slck(&self) -> bool {
        *self == SMSMPLR::_2048SLCK
    }
}
#[doc = "Possible values of the field `SMRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTENR {
    #[doc = "The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMRSTENR {
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
            SMRSTENR::NOT_ENABLE => false,
            SMRSTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMRSTENR {
        match value {
            false => SMRSTENR::NOT_ENABLE,
            true => SMRSTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == SMRSTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SMRSTENR::ENABLE
    }
}
#[doc = "Possible values of the field `SMIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMIENR {
    #[doc = "The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMIENR {
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
            SMIENR::NOT_ENABLE => false,
            SMIENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMIENR {
        match value {
            false => SMIENR::NOT_ENABLE,
            true => SMIENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == SMIENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SMIENR::ENABLE
    }
}
#[doc = r" Proxy"]
pub struct _SMTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SMTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMSMPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSMPLW {
    #[doc = "Supply Monitor disabled"]
    SMD,
    #[doc = "Continuous Supply Monitor"]
    CSM,
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK,
}
impl SMSMPLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMSMPLW::SMD => 0,
            SMSMPLW::CSM => 1,
            SMSMPLW::_32SLCK => 2,
            SMSMPLW::_256SLCK => 3,
            SMSMPLW::_2048SLCK => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMSMPLW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSMPLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMSMPLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline]
    pub fn smd(self) -> &'a mut W {
        self.variant(SMSMPLW::SMD)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline]
    pub fn csm(self) -> &'a mut W {
        self.variant(SMSMPLW::CSM)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline]
    pub fn _32slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_32SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline]
    pub fn _256slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_256SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline]
    pub fn _2048slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_2048SLCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTENW {
    #[doc = "The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMRSTENW::NOT_ENABLE => false,
            SMRSTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMRSTENW::NOT_ENABLE)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMRSTENW::ENABLE)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMIENW {
    #[doc = "The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMIENW::NOT_ENABLE => false,
            SMIENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMIENW::NOT_ENABLE)
    }
    #[doc = "The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMIENW::ENABLE)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline]
    pub fn smth(&self) -> SMTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMTHR { bits }
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline]
    pub fn smsmpl(&self) -> SMSMPLR {
        SMSMPLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline]
    pub fn smrsten(&self) -> SMRSTENR {
        SMRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline]
    pub fn smien(&self) -> SMIENR {
        SMIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline]
    pub fn smth(&mut self) -> _SMTHW {
        _SMTHW { w: self }
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline]
    pub fn smsmpl(&mut self) -> _SMSMPLW {
        _SMSMPLW { w: self }
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline]
    pub fn smrsten(&mut self) -> _SMRSTENW {
        _SMRSTENW { w: self }
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline]
    pub fn smien(&mut self) -> _SMIENW {
        _SMIENW { w: self }
    }
}
