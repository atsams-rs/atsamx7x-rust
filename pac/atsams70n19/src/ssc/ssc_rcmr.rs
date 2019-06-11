#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSC_RCMR {
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
    #[doc = "TK Clock signal"]
    TK,
    #[doc = "RK pin"]
    RK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKSR::MCK => 0,
            CKSR::TK => 1,
            CKSR::RK => 2,
            CKSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKSR {
        match value {
            0 => CKSR::MCK,
            1 => CKSR::TK,
            2 => CKSR::RK,
            i => CKSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline]
    pub fn is_mck(&self) -> bool {
        *self == CKSR::MCK
    }
    #[doc = "Checks if the value of the field is `TK`"]
    #[inline]
    pub fn is_tk(&self) -> bool {
        *self == CKSR::TK
    }
    #[doc = "Checks if the value of the field is `RK`"]
    #[inline]
    pub fn is_rk(&self) -> bool {
        *self == CKSR::RK
    }
}
#[doc = "Possible values of the field `CKO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKOR {
    #[doc = "None, RK pin is an input"]
    NONE,
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    CONTINUOUS,
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
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
    #[doc = "Receive Clock enabled only if RF Low"]
    EN_RF_LOW,
    #[doc = "Receive Clock enabled only if RF High"]
    EN_RF_HIGH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKGR::CONTINUOUS => 0,
            CKGR::EN_RF_LOW => 1,
            CKGR::EN_RF_HIGH => 2,
            CKGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKGR {
        match value {
            0 => CKGR::CONTINUOUS,
            1 => CKGR::EN_RF_LOW,
            2 => CKGR::EN_RF_HIGH,
            i => CKGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == CKGR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `EN_RF_LOW`"]
    #[inline]
    pub fn is_en_rf_low(&self) -> bool {
        *self == CKGR::EN_RF_LOW
    }
    #[doc = "Checks if the value of the field is `EN_RF_HIGH`"]
    #[inline]
    pub fn is_en_rf_high(&self) -> bool {
        *self == CKGR::EN_RF_HIGH
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    CONTINUOUS,
    #[doc = "Transmit start"]
    TRANSMIT,
    #[doc = "Detection of a low level on RF signal"]
    RF_LOW,
    #[doc = "Detection of a high level on RF signal"]
    RF_HIGH,
    #[doc = "Detection of a falling edge on RF signal"]
    RF_FALLING,
    #[doc = "Detection of a rising edge on RF signal"]
    RF_RISING,
    #[doc = "Detection of any level change on RF signal"]
    RF_LEVEL,
    #[doc = "Detection of any edge on RF signal"]
    RF_EDGE,
    #[doc = "Compare 0"]
    CMP_0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTR::CONTINUOUS => 0,
            STARTR::TRANSMIT => 1,
            STARTR::RF_LOW => 2,
            STARTR::RF_HIGH => 3,
            STARTR::RF_FALLING => 4,
            STARTR::RF_RISING => 5,
            STARTR::RF_LEVEL => 6,
            STARTR::RF_EDGE => 7,
            STARTR::CMP_0 => 8,
            STARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTR {
        match value {
            0 => STARTR::CONTINUOUS,
            1 => STARTR::TRANSMIT,
            2 => STARTR::RF_LOW,
            3 => STARTR::RF_HIGH,
            4 => STARTR::RF_FALLING,
            5 => STARTR::RF_RISING,
            6 => STARTR::RF_LEVEL,
            7 => STARTR::RF_EDGE,
            8 => STARTR::CMP_0,
            i => STARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == STARTR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline]
    pub fn is_transmit(&self) -> bool {
        *self == STARTR::TRANSMIT
    }
    #[doc = "Checks if the value of the field is `RF_LOW`"]
    #[inline]
    pub fn is_rf_low(&self) -> bool {
        *self == STARTR::RF_LOW
    }
    #[doc = "Checks if the value of the field is `RF_HIGH`"]
    #[inline]
    pub fn is_rf_high(&self) -> bool {
        *self == STARTR::RF_HIGH
    }
    #[doc = "Checks if the value of the field is `RF_FALLING`"]
    #[inline]
    pub fn is_rf_falling(&self) -> bool {
        *self == STARTR::RF_FALLING
    }
    #[doc = "Checks if the value of the field is `RF_RISING`"]
    #[inline]
    pub fn is_rf_rising(&self) -> bool {
        *self == STARTR::RF_RISING
    }
    #[doc = "Checks if the value of the field is `RF_LEVEL`"]
    #[inline]
    pub fn is_rf_level(&self) -> bool {
        *self == STARTR::RF_LEVEL
    }
    #[doc = "Checks if the value of the field is `RF_EDGE`"]
    #[inline]
    pub fn is_rf_edge(&self) -> bool {
        *self == STARTR::RF_EDGE
    }
    #[doc = "Checks if the value of the field is `CMP_0`"]
    #[inline]
    pub fn is_cmp_0(&self) -> bool {
        *self == STARTR::CMP_0
    }
}
#[doc = r" Value of the field"]
pub struct STOPR {
    bits: bool,
}
impl STOPR {
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
    #[doc = "TK Clock signal"]
    TK,
    #[doc = "RK pin"]
    RK,
}
impl CKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSW::MCK => 0,
            CKSW::TK => 1,
            CKSW::RK => 2,
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
    #[doc = "TK Clock signal"]
    #[inline]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKSW::TK)
    }
    #[doc = "RK pin"]
    #[inline]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKSW::RK)
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
    #[doc = "None, RK pin is an input"]
    NONE,
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    CONTINUOUS,
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
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
    #[doc = "None, RK pin is an input"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CKOW::NONE)
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKOW::CONTINUOUS)
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
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
    #[doc = "Receive Clock enabled only if RF Low"]
    EN_RF_LOW,
    #[doc = "Receive Clock enabled only if RF High"]
    EN_RF_HIGH,
}
impl CKGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKGW::CONTINUOUS => 0,
            CKGW::EN_RF_LOW => 1,
            CKGW::EN_RF_HIGH => 2,
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
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline]
    pub fn en_rf_low(self) -> &'a mut W {
        self.variant(CKGW::EN_RF_LOW)
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline]
    pub fn en_rf_high(self) -> &'a mut W {
        self.variant(CKGW::EN_RF_HIGH)
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
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    CONTINUOUS,
    #[doc = "Transmit start"]
    TRANSMIT,
    #[doc = "Detection of a low level on RF signal"]
    RF_LOW,
    #[doc = "Detection of a high level on RF signal"]
    RF_HIGH,
    #[doc = "Detection of a falling edge on RF signal"]
    RF_FALLING,
    #[doc = "Detection of a rising edge on RF signal"]
    RF_RISING,
    #[doc = "Detection of any level change on RF signal"]
    RF_LEVEL,
    #[doc = "Detection of any edge on RF signal"]
    RF_EDGE,
    #[doc = "Compare 0"]
    CMP_0,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTW::CONTINUOUS => 0,
            STARTW::TRANSMIT => 1,
            STARTW::RF_LOW => 2,
            STARTW::RF_HIGH => 3,
            STARTW::RF_FALLING => 4,
            STARTW::RF_RISING => 5,
            STARTW::RF_LEVEL => 6,
            STARTW::RF_EDGE => 7,
            STARTW::CMP_0 => 8,
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
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(STARTW::CONTINUOUS)
    }
    #[doc = "Transmit start"]
    #[inline]
    pub fn transmit(self) -> &'a mut W {
        self.variant(STARTW::TRANSMIT)
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline]
    pub fn rf_low(self) -> &'a mut W {
        self.variant(STARTW::RF_LOW)
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline]
    pub fn rf_high(self) -> &'a mut W {
        self.variant(STARTW::RF_HIGH)
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline]
    pub fn rf_falling(self) -> &'a mut W {
        self.variant(STARTW::RF_FALLING)
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline]
    pub fn rf_rising(self) -> &'a mut W {
        self.variant(STARTW::RF_RISING)
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline]
    pub fn rf_level(self) -> &'a mut W {
        self.variant(STARTW::RF_LEVEL)
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline]
    pub fn rf_edge(self) -> &'a mut W {
        self.variant(STARTW::RF_EDGE)
    }
    #[doc = "Compare 0"]
    #[inline]
    pub fn cmp_0(self) -> &'a mut W {
        self.variant(STARTW::CMP_0)
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
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
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
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline]
    pub fn cks(&self) -> CKSR {
        CKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline]
    pub fn cko(&self) -> CKOR {
        CKOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline]
    pub fn cki(&self) -> CKIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CKIR { bits }
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline]
    pub fn ckg(&self) -> CKGR {
        CKGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOPR { bits }
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline]
    pub fn sttdly(&self) -> STTDLYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STTDLYR { bits }
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
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
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline]
    pub fn cks(&mut self) -> _CKSW {
        _CKSW { w: self }
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline]
    pub fn cko(&mut self) -> _CKOW {
        _CKOW { w: self }
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline]
    pub fn cki(&mut self) -> _CKIW {
        _CKIW { w: self }
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline]
    pub fn ckg(&mut self) -> _CKGW {
        _CKGW { w: self }
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline]
    pub fn sttdly(&mut self) -> _STTDLYW {
        _STTDLYW { w: self }
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline]
    pub fn period(&mut self) -> _PERIODW {
        _PERIODW { w: self }
    }
}
