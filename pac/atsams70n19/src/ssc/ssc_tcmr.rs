#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSC_TCMR {
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
#[doc = "Possible values of the field `CKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSR {
    #[doc = "Divided Clock"]
    MCK,
    #[doc = "RK Clock signal"]
    RK,
    #[doc = "TK pin"]
    TK,
}
impl crate::ToBits<u8> for CKSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKSR::MCK => 0,
            CKSR::RK => 1,
            CKSR::TK => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CKS_R = crate::FR<u8, CKSR>;
impl CKS_R {
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CKSR::MCK
    }
    #[doc = "Checks if the value of the field is `RK`"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == CKSR::RK
    }
    #[doc = "Checks if the value of the field is `TK`"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == CKSR::TK
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSW::MCK => 0,
            CKSW::RK => 1,
            CKSW::TK => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CKSW::MCK)
    }
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKSW::RK)
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKSW::TK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
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
}
impl crate::ToBits<u8> for CKOR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKOR::NONE => 0,
            CKOR::CONTINUOUS => 1,
            CKOR::TRANSFER => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CKO_R = crate::FR<u8, CKOR>;
impl CKO_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKOR::NONE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKOR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == CKOR::TRANSFER
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKOW::NONE => 0,
            CKOW::CONTINUOUS => 1,
            CKOW::TRANSFER => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CKOW<'a> {
    w: &'a mut W,
}
impl<'a> _CKOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKOW::NONE)
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKOW::CONTINUOUS)
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(CKOW::TRANSFER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CKI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CKIW<'a> {
    w: &'a mut W,
}
impl<'a> _CKIW<'a> {
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
#[doc = "Possible values of the field `CKG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKGR {
    #[doc = "None"]
    CONTINUOUS,
    #[doc = "Transmit Clock enabled only if TF Low"]
    EN_TF_LOW,
    #[doc = "Transmit Clock enabled only if TF High"]
    EN_TF_HIGH,
}
impl crate::ToBits<u8> for CKGR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKGR::CONTINUOUS => 0,
            CKGR::EN_TF_LOW => 1,
            CKGR::EN_TF_HIGH => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CKG_R = crate::FR<u8, CKGR>;
impl CKG_R {
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKGR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `EN_TF_LOW`"]
    #[inline(always)]
    pub fn is_en_tf_low(&self) -> bool {
        *self == CKGR::EN_TF_LOW
    }
    #[doc = "Checks if the value of the field is `EN_TF_HIGH`"]
    #[inline(always)]
    pub fn is_en_tf_high(&self) -> bool {
        *self == CKGR::EN_TF_HIGH
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKGW::CONTINUOUS => 0,
            CKGW::EN_TF_LOW => 1,
            CKGW::EN_TF_HIGH => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CKGW<'a> {
    w: &'a mut W,
}
impl<'a> _CKGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKGW::CONTINUOUS)
    }
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn en_tf_low(self) -> &'a mut W {
        self.variant(CKGW::EN_TF_LOW)
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn en_tf_high(self) -> &'a mut W {
        self.variant(CKGW::EN_TF_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
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
}
impl crate::ToBits<u8> for STARTR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            STARTR::CONTINUOUS => 0,
            STARTR::RECEIVE => 1,
            STARTR::TF_LOW => 2,
            STARTR::TF_HIGH => 3,
            STARTR::TF_FALLING => 4,
            STARTR::TF_RISING => 5,
            STARTR::TF_LEVEL => 6,
            STARTR::TF_EDGE => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type START_R = crate::FR<u8, STARTR>;
impl START_R {
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == STARTR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == STARTR::RECEIVE
    }
    #[doc = "Checks if the value of the field is `TF_LOW`"]
    #[inline(always)]
    pub fn is_tf_low(&self) -> bool {
        *self == STARTR::TF_LOW
    }
    #[doc = "Checks if the value of the field is `TF_HIGH`"]
    #[inline(always)]
    pub fn is_tf_high(&self) -> bool {
        *self == STARTR::TF_HIGH
    }
    #[doc = "Checks if the value of the field is `TF_FALLING`"]
    #[inline(always)]
    pub fn is_tf_falling(&self) -> bool {
        *self == STARTR::TF_FALLING
    }
    #[doc = "Checks if the value of the field is `TF_RISING`"]
    #[inline(always)]
    pub fn is_tf_rising(&self) -> bool {
        *self == STARTR::TF_RISING
    }
    #[doc = "Checks if the value of the field is `TF_LEVEL`"]
    #[inline(always)]
    pub fn is_tf_level(&self) -> bool {
        *self == STARTR::TF_LEVEL
    }
    #[doc = "Checks if the value of the field is `TF_EDGE`"]
    #[inline(always)]
    pub fn is_tf_edge(&self) -> bool {
        *self == STARTR::TF_EDGE
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(STARTW::CONTINUOUS)
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut W {
        self.variant(STARTW::RECEIVE)
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn tf_low(self) -> &'a mut W {
        self.variant(STARTW::TF_LOW)
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn tf_high(self) -> &'a mut W {
        self.variant(STARTW::TF_HIGH)
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn tf_falling(self) -> &'a mut W {
        self.variant(STARTW::TF_FALLING)
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn tf_rising(self) -> &'a mut W {
        self.variant(STARTW::TF_RISING)
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn tf_level(self) -> &'a mut W {
        self.variant(STARTW::TF_LEVEL)
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn tf_edge(self) -> &'a mut W {
        self.variant(STARTW::TF_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type STTDLY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _STTDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _STTDLYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PERIOD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIODW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits() >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> _CKSW {
        _CKSW { w: self }
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&mut self) -> _CKOW {
        _CKOW { w: self }
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&mut self) -> _CKIW {
        _CKIW { w: self }
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&mut self) -> _CKGW {
        _CKGW { w: self }
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&mut self) -> _STTDLYW {
        _STTDLYW { w: self }
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&mut self) -> _PERIODW {
        _PERIODW { w: self }
    }
}
