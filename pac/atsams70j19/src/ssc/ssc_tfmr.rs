#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSC_TFMR {
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
pub type DATLEN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DATLENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATLENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DATDEF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATDEFW<'a> {
    w: &'a mut W,
}
impl<'a> _DATDEFW<'a> {
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
#[doc = r"Reader of the field"]
pub type MSBF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFW<'a> {
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
#[doc = r"Reader of the field"]
pub type DATNB_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DATNBW<'a> {
    w: &'a mut W,
}
impl<'a> _DATNBW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSLEN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FSLENW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `FSOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSOSR {
    #[doc = "None, TF pin is an input"]
    NONE,
    #[doc = "Negative Pulse, TF pin is an output"]
    NEGATIVE,
    #[doc = "Positive Pulse, TF pin is an output"]
    POSITIVE,
    #[doc = "Driven Low during data transfer"]
    LOW,
    #[doc = "Driven High during data transfer"]
    HIGH,
    #[doc = "Toggling at each start of data transfer"]
    TOGGLING,
}
impl crate::ToBits<u8> for FSOSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FSOSR::NONE => 0,
            FSOSR::NEGATIVE => 1,
            FSOSR::POSITIVE => 2,
            FSOSR::LOW => 3,
            FSOSR::HIGH => 4,
            FSOSR::TOGGLING => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FSOS_R = crate::FR<u8, FSOSR>;
impl FSOS_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FSOSR::NONE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSOSR::NEGATIVE
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSOSR::POSITIVE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FSOSR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FSOSR::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLING`"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == FSOSR::TOGGLING
    }
}
#[doc = "Values that can be written to the field `FSOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSOSW {
    #[doc = "None, TF pin is an input"]
    NONE,
    #[doc = "Negative Pulse, TF pin is an output"]
    NEGATIVE,
    #[doc = "Positive Pulse, TF pin is an output"]
    POSITIVE,
    #[doc = "Driven Low during data transfer"]
    LOW,
    #[doc = "Driven High during data transfer"]
    HIGH,
    #[doc = "Toggling at each start of data transfer"]
    TOGGLING,
}
impl FSOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSOSW::NONE => 0,
            FSOSW::NEGATIVE => 1,
            FSOSW::POSITIVE => 2,
            FSOSW::LOW => 3,
            FSOSW::HIGH => 4,
            FSOSW::TOGGLING => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FSOSW<'a> {
    w: &'a mut W,
}
impl<'a> _FSOSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSOSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None, TF pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(FSOSW::NONE)
    }
    #[doc = "Negative Pulse, TF pin is an output"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSOSW::NEGATIVE)
    }
    #[doc = "Positive Pulse, TF pin is an output"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSOSW::POSITIVE)
    }
    #[doc = "Driven Low during data transfer"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FSOSW::LOW)
    }
    #[doc = "Driven High during data transfer"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FSOSW::HIGH)
    }
    #[doc = "Toggling at each start of data transfer"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut W {
        self.variant(FSOSW::TOGGLING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSDEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSDENW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `FSEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSEDGER {
    #[doc = "Positive Edge Detection"]
    POSITIVE,
    #[doc = "Negative Edge Detection"]
    NEGATIVE,
}
impl crate::ToBits<bool> for FSEDGER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FSEDGER::POSITIVE => false,
            FSEDGER::NEGATIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FSEDGE_R = crate::FR<bool, FSEDGER>;
impl FSEDGE_R {
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSEDGER::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSEDGER::NEGATIVE
    }
}
#[doc = "Values that can be written to the field `FSEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSEDGEW {
    #[doc = "Positive Edge Detection"]
    POSITIVE,
    #[doc = "Negative Edge Detection"]
    NEGATIVE,
}
impl FSEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FSEDGEW::POSITIVE => false,
            FSEDGEW::NEGATIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FSEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _FSEDGEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSEDGEW::POSITIVE)
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSEDGEW::NEGATIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSLEN_EXT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FSLEN_EXTW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLEN_EXTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline(always)]
    pub fn datdef(&self) -> DATDEF_R {
        DATDEF_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&self) -> DATNB_R {
        DATNB_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FSLEN_R {
        FSLEN_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&self) -> FSOS_R {
        FSOS_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline(always)]
    pub fn fsden(&self) -> FSDEN_R {
        FSDEN_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&self) -> FSEDGE_R {
        FSEDGE_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&self) -> FSLEN_EXT_R {
        FSLEN_EXT_R::new(((self.bits() >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&mut self) -> _DATLENW {
        _DATLENW { w: self }
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline(always)]
    pub fn datdef(&mut self) -> _DATDEFW {
        _DATDEFW { w: self }
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> _MSBFW {
        _MSBFW { w: self }
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&mut self) -> _DATNBW {
        _DATNBW { w: self }
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&mut self) -> _FSLENW {
        _FSLENW { w: self }
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&mut self) -> _FSOSW {
        _FSOSW { w: self }
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline(always)]
    pub fn fsden(&mut self) -> _FSDENW {
        _FSDENW { w: self }
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&mut self) -> _FSEDGEW {
        _FSEDGEW { w: self }
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&mut self) -> _FSLEN_EXTW {
        _FSLEN_EXTW { w: self }
    }
}
