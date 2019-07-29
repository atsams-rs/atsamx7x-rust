#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPC_SMMR {
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
pub type SMTH_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SMTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SMTHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
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
}
impl crate::ToBits<u8> for SMSMPLR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SMSMPLR::SMD => 0,
            SMSMPLR::CSM => 1,
            SMSMPLR::_32SLCK => 2,
            SMSMPLR::_256SLCK => 3,
            SMSMPLR::_2048SLCK => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMSMPL_R = crate::FR<u8, SMSMPLR>;
impl SMSMPL_R {
    #[doc = "Checks if the value of the field is `SMD`"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        *self == SMSMPLR::SMD
    }
    #[doc = "Checks if the value of the field is `CSM`"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        *self == SMSMPLR::CSM
    }
    #[doc = "Checks if the value of the field is `_32SLCK`"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        *self == SMSMPLR::_32SLCK
    }
    #[doc = "Checks if the value of the field is `_256SLCK`"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        *self == SMSMPLR::_256SLCK
    }
    #[doc = "Checks if the value of the field is `_2048SLCK`"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        *self == SMSMPLR::_2048SLCK
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _SMSMPLW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSMPLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMSMPLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut W {
        self.variant(SMSMPLW::SMD)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut W {
        self.variant(SMSMPLW::CSM)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_32SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_256SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_2048SLCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
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
impl crate::ToBits<bool> for SMRSTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMRSTENR::NOT_ENABLE => false,
            SMRSTENR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMRSTEN_R = crate::FR<bool, SMRSTENR>;
impl SMRSTEN_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMRSTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMRSTENR::ENABLE
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SMRSTENW::NOT_ENABLE => false,
            SMRSTENW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SMRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMRSTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMRSTENW::NOT_ENABLE)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMRSTENW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
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
impl crate::ToBits<bool> for SMIENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMIENR::NOT_ENABLE => false,
            SMIENR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMIEN_R = crate::FR<bool, SMIENR>;
impl SMIEN_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMIENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMIENR::ENABLE
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SMIENW::NOT_ENABLE => false,
            SMIENW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SMIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMIENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMIENW::NOT_ENABLE)
    }
    #[doc = "The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMIENW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SMTH_R {
        SMTH_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SMSMPL_R {
        SMSMPL_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SMRSTEN_R {
        SMRSTEN_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SMIEN_R {
        SMIEN_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&mut self) -> _SMTHW {
        _SMTHW { w: self }
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&mut self) -> _SMSMPLW {
        _SMSMPLW { w: self }
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&mut self) -> _SMRSTENW {
        _SMRSTENW { w: self }
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&mut self) -> _SMIENW {
        _SMIENW { w: self }
    }
}
