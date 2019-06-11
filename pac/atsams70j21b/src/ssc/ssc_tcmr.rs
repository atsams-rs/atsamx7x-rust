#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSC_TCMR {
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
#[doc = "Possible values of the field `CKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSR {
    #[doc = "Divided Clock"]
    MCK,
    #[doc = "RK Clock signal"]
    RK,
    #[doc = "TK pin"]
    TK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKSR::MCK => 0,
            CKSR::RK => 1,
            CKSR::TK => 2,
            CKSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKSR {
        match value {
            0 => CKSR::MCK,
            1 => CKSR::RK,
            2 => CKSR::TK,
            i => CKSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline]
    pub fn is_mck(&self) -> bool {
        *self == CKSR::MCK
    }
    #[doc = "Checks if the value of the field is `RK`"]
    #[inline]
    pub fn is_rk(&self) -> bool {
        *self == CKSR::RK
    }
    #[doc = "Checks if the value of the field is `TK`"]
    #[inline]
    pub fn is_tk(&self) -> bool {
        *self == CKSR::TK
    }
}
#[doc = "Possible values of the field `CKO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKOR {
    #[doc = "None, TK pin is an input"]
    NONE,
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    CONTINUOUS,
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    TRANSFER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKOR::NONE => 0,
            CKOR::CONTINUOUS => 1,
            CKOR::TRANSFER => 2,
            CKOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKOR {
        match value {
            0 => CKOR::NONE,
            1 => CKOR::CONTINUOUS,
            2 => CKOR::TRANSFER,
            i => CKOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CKOR::NONE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == CKOR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline]
    pub fn is_transfer(&self) -> bool {
        *self == CKOR::TRANSFER
    }
}
#[doc = r" Value of the field"]
pub struct CKIR {
    bits: bool,
}
impl CKIR {
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
#[doc = "Possible values of the field `CKG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKGR {
    #[doc = "None"]
    CONTINUOUS,
    #[doc = "Transmit Clock enabled only if TF Low"]
    EN_TF_LOW,
    #[doc = "Transmit Clock enabled only if TF High"]
    EN_TF_HIGH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKGR::CONTINUOUS => 0,
            CKGR::EN_TF_LOW => 1,
            CKGR::EN_TF_HIGH => 2,
            CKGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKGR {
        match value {
            0 => CKGR::CONTINUOUS,
            1 => CKGR::EN_TF_LOW,
            2 => CKGR::EN_TF_HIGH,
            i => CKGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == CKGR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `EN_TF_LOW`"]
    #[inline]
    pub fn is_en_tf_low(&self) -> bool {
        *self == CKGR::EN_TF_LOW
    }
    #[doc = "Checks if the value of the field is `EN_TF_HIGH`"]
    #[inline]
    pub fn is_en_tf_high(&self) -> bool {
        *self == CKGR::EN_TF_HIGH
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    CONTINUOUS,
    #[doc = "Receive start"]
    RECEIVE,
    #[doc = "Detection of a low level on TF signal"]
    TF_LOW,
    #[doc = "Detection of a high level on TF signal"]
    TF_HIGH,
    #[doc = "Detection of a falling edge on TF signal"]
    TF_FALLING,
    #[doc = "Detection of a rising edge on TF signal"]
    TF_RISING,
    #[doc = "Detection of any level change on TF signal"]
    TF_LEVEL,
    #[doc = "Detection of any edge on TF signal"]
    TF_EDGE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTR::CONTINUOUS => 0,
            STARTR::RECEIVE => 1,
            STARTR::TF_LOW => 2,
            STARTR::TF_HIGH => 3,
            STARTR::TF_FALLING => 4,
            STARTR::TF_RISING => 5,
            STARTR::TF_LEVEL => 6,
            STARTR::TF_EDGE => 7,
            STARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTR {
        match value {
            0 => STARTR::CONTINUOUS,
            1 => STARTR::RECEIVE,
            2 => STARTR::TF_LOW,
            3 => STARTR::TF_HIGH,
            4 => STARTR::TF_FALLING,
            5 => STARTR::TF_RISING,
            6 => STARTR::TF_LEVEL,
            7 => STARTR::TF_EDGE,
            i => STARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == STARTR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline]
    pub fn is_receive(&self) -> bool {
        *self == STARTR::RECEIVE
    }
    #[doc = "Checks if the value of the field is `TF_LOW`"]
    #[inline]
    pub fn is_tf_low(&self) -> bool {
        *self == STARTR::TF_LOW
    }
    #[doc = "Checks if the value of the field is `TF_HIGH`"]
    #[inline]
    pub fn is_tf_high(&self) -> bool {
        *self == STARTR::TF_HIGH
    }
    #[doc = "Checks if the value of the field is `TF_FALLING`"]
    #[inline]
    pub fn is_tf_falling(&self) -> bool {
        *self == STARTR::TF_FALLING
    }
    #[doc = "Checks if the value of the field is `TF_RISING`"]
    #[inline]
    pub fn is_tf_rising(&self) -> bool {
        *self == STARTR::TF_RISING
    }
    #[doc = "Checks if the value of the field is `TF_LEVEL`"]
    #[inline]
    pub fn is_tf_level(&self) -> bool {
        *self == STARTR::TF_LEVEL
    }
    #[doc = "Checks if the value of the field is `TF_EDGE`"]
    #[inline]
    pub fn is_tf_edge(&self) -> bool {
        *self == STARTR::TF_EDGE
    }
}
#[doc = r" Value of the field"]
pub struct STTDLYR {
    bits: u8,
}
impl STTDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERIODR {
    bits: u8,
}
impl PERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSW {
    #[doc = "Divided Clock"]
    MCK,
    #[doc = "RK Clock signal"]
    RK,
    #[doc = "TK pin"]
    TK,
}
impl CKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSW::MCK => 0,
            CKSW::RK => 1,
            CKSW::TK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divided Clock"]
    #[inline]
    pub fn mck(self) -> &'a mut W {
        self.variant(CKSW::MCK)
    }
    #[doc = "RK Clock signal"]
    #[inline]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKSW::RK)
    }
    #[doc = "TK pin"]
    #[inline]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKSW::TK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKOW {
    #[doc = "None, TK pin is an input"]
    NONE,
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    CONTINUOUS,
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    TRANSFER,
}
impl CKOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKOW::NONE => 0,
            CKOW::CONTINUOUS => 1,
            CKOW::TRANSFER => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKOW<'a> {
    w: &'a mut W,
}
impl<'a> _CKOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None, TK pin is an input"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CKOW::NONE)
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKOW::CONTINUOUS)
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline]
    pub fn transfer(self) -> &'a mut W {
        self.variant(CKOW::TRANSFER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CKIW<'a> {
    w: &'a mut W,
}
impl<'a> _CKIW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKGW {
    #[doc = "None"]
    CONTINUOUS,
    #[doc = "Transmit Clock enabled only if TF Low"]
    EN_TF_LOW,
    #[doc = "Transmit Clock enabled only if TF High"]
    EN_TF_HIGH,
}
impl CKGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKGW::CONTINUOUS => 0,
            CKGW::EN_TF_LOW => 1,
            CKGW::EN_TF_HIGH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKGW<'a> {
    w: &'a mut W,
}
impl<'a> _CKGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKGW::CONTINUOUS)
    }
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline]
    pub fn en_tf_low(self) -> &'a mut W {
        self.variant(CKGW::EN_TF_LOW)
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline]
    pub fn en_tf_high(self) -> &'a mut W {
        self.variant(CKGW::EN_TF_HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTW {
    #[doc = "Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    CONTINUOUS,
    #[doc = "Receive start"]
    RECEIVE,
    #[doc = "Detection of a low level on TF signal"]
    TF_LOW,
    #[doc = "Detection of a high level on TF signal"]
    TF_HIGH,
    #[doc = "Detection of a falling edge on TF signal"]
    TF_FALLING,
    #[doc = "Detection of a rising edge on TF signal"]
    TF_RISING,
    #[doc = "Detection of any level change on TF signal"]
    TF_LEVEL,
    #[doc = "Detection of any edge on TF signal"]
    TF_EDGE,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTW::CONTINUOUS => 0,
            STARTW::RECEIVE => 1,
            STARTW::TF_LOW => 2,
            STARTW::TF_HIGH => 3,
            STARTW::TF_FALLING => 4,
            STARTW::TF_RISING => 5,
            STARTW::TF_LEVEL => 6,
            STARTW::TF_EDGE => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(STARTW::CONTINUOUS)
    }
    #[doc = "Receive start"]
    #[inline]
    pub fn receive(self) -> &'a mut W {
        self.variant(STARTW::RECEIVE)
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline]
    pub fn tf_low(self) -> &'a mut W {
        self.variant(STARTW::TF_LOW)
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline]
    pub fn tf_high(self) -> &'a mut W {
        self.variant(STARTW::TF_HIGH)
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline]
    pub fn tf_falling(self) -> &'a mut W {
        self.variant(STARTW::TF_FALLING)
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline]
    pub fn tf_rising(self) -> &'a mut W {
        self.variant(STARTW::TF_RISING)
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline]
    pub fn tf_level(self) -> &'a mut W {
        self.variant(STARTW::TF_LEVEL)
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline]
    pub fn tf_edge(self) -> &'a mut W {
        self.variant(STARTW::TF_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STTDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _STTDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIODW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline]
    pub fn cks(&self) -> CKSR {
        CKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline]
    pub fn cko(&self) -> CKOR {
        CKOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline]
    pub fn cki(&self) -> CKIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CKIR { bits }
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline]
    pub fn ckg(&self) -> CKGR {
        CKGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline]
    pub fn sttdly(&self) -> STTDLYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STTDLYR { bits }
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline]
    pub fn period(&self) -> PERIODR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERIODR { bits }
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
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline]
    pub fn cks(&mut self) -> _CKSW {
        _CKSW { w: self }
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline]
    pub fn cko(&mut self) -> _CKOW {
        _CKOW { w: self }
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline]
    pub fn cki(&mut self) -> _CKIW {
        _CKIW { w: self }
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline]
    pub fn ckg(&mut self) -> _CKGW {
        _CKGW { w: self }
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline]
    pub fn sttdly(&mut self) -> _STTDLYW {
        _STTDLYW { w: self }
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline]
    pub fn period(&mut self) -> _PERIODW {
        _PERIODW { w: self }
    }
}
