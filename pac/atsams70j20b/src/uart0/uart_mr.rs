#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UART_MR {
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
#[doc = "Possible values of the field `FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERR {
    #[doc = "UART does not filter the receive line."]
    DISABLED,
    #[doc = "UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    ENABLED,
}
impl crate::ToBits<bool> for FILTERR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FILTERR::DISABLED => false,
            FILTERR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FILTER_R = crate::FR<bool, FILTERR>;
impl FILTER_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FILTERR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FILTERR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERW {
    #[doc = "UART does not filter the receive line."]
    DISABLED,
    #[doc = "UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    ENABLED,
}
impl FILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTERW::DISABLED => false,
            FILTERW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UART does not filter the receive line."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTERW::DISABLED)
    }
    #[doc = "UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTERW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `PAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARR {
    #[doc = "Even Parity"]
    EVEN,
    #[doc = "Odd Parity"]
    ODD,
    #[doc = "Space: parity forced to 0"]
    SPACE,
    #[doc = "Mark: parity forced to 1"]
    MARK,
    #[doc = "No parity"]
    NO,
}
impl crate::ToBits<u8> for PARR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PARR::EVEN => 0,
            PARR::ODD => 1,
            PARR::SPACE => 2,
            PARR::MARK => 3,
            PARR::NO => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PAR_R = crate::FR<u8, PARR>;
impl PAR_R {
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARR::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PARR::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PARR::MARK
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PARR::NO
    }
}
#[doc = "Values that can be written to the field `PAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARW {
    #[doc = "Even Parity"]
    EVEN,
    #[doc = "Odd Parity"]
    ODD,
    #[doc = "Space: parity forced to 0"]
    SPACE,
    #[doc = "Mark: parity forced to 1"]
    MARK,
    #[doc = "No parity"]
    NO,
}
impl PARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARW::EVEN => 0,
            PARW::ODD => 1,
            PARW::SPACE => 2,
            PARW::MARK => 3,
            PARW::NO => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PARW<'a> {
    w: &'a mut W,
}
impl<'a> _PARW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARW::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARW::ODD)
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn space(self) -> &'a mut W {
        self.variant(PARW::SPACE)
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(PARW::MARK)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PARW::NO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `BRSRCCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSRCCKR {
    #[doc = "The baud rate is driven by the peripheral clock"]
    PERIPH_CLK,
    #[doc = "The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    PMC_PCK,
}
impl crate::ToBits<bool> for BRSRCCKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BRSRCCKR::PERIPH_CLK => false,
            BRSRCCKR::PMC_PCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BRSRCCK_R = crate::FR<bool, BRSRCCKR>;
impl BRSRCCK_R {
    #[doc = "Checks if the value of the field is `PERIPH_CLK`"]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        *self == BRSRCCKR::PERIPH_CLK
    }
    #[doc = "Checks if the value of the field is `PMC_PCK`"]
    #[inline(always)]
    pub fn is_pmc_pck(&self) -> bool {
        *self == BRSRCCKR::PMC_PCK
    }
}
#[doc = "Values that can be written to the field `BRSRCCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSRCCKW {
    #[doc = "The baud rate is driven by the peripheral clock"]
    PERIPH_CLK,
    #[doc = "The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    PMC_PCK,
}
impl BRSRCCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BRSRCCKW::PERIPH_CLK => false,
            BRSRCCKW::PMC_PCK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BRSRCCKW<'a> {
    w: &'a mut W,
}
impl<'a> _BRSRCCKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSRCCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The baud rate is driven by the peripheral clock"]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut W {
        self.variant(BRSRCCKW::PERIPH_CLK)
    }
    #[doc = "The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    #[inline(always)]
    pub fn pmc_pck(self) -> &'a mut W {
        self.variant(BRSRCCKW::PMC_PCK)
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
#[doc = "Possible values of the field `CHMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMODER {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Automatic echo"]
    AUTOMATIC,
    #[doc = "Local loopback"]
    LOCAL_LOOPBACK,
    #[doc = "Remote loopback"]
    REMOTE_LOOPBACK,
}
impl crate::ToBits<u8> for CHMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CHMODER::NORMAL => 0,
            CHMODER::AUTOMATIC => 1,
            CHMODER::LOCAL_LOOPBACK => 2,
            CHMODER::REMOTE_LOOPBACK => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CHMODE_R = crate::FR<u8, CHMODER>;
impl CHMODE_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == CHMODER::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOPBACK`"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == CHMODER::LOCAL_LOOPBACK
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOPBACK`"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == CHMODER::REMOTE_LOOPBACK
    }
}
#[doc = "Values that can be written to the field `CHMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMODEW {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Automatic echo"]
    AUTOMATIC,
    #[doc = "Local loopback"]
    LOCAL_LOOPBACK,
    #[doc = "Remote loopback"]
    REMOTE_LOOPBACK,
}
impl CHMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHMODEW::NORMAL => 0,
            CHMODEW::AUTOMATIC => 1,
            CHMODEW::LOCAL_LOOPBACK => 2,
            CHMODEW::REMOTE_LOOPBACK => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CHMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODEW::NORMAL)
    }
    #[doc = "Automatic echo"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(CHMODEW::AUTOMATIC)
    }
    #[doc = "Local loopback"]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut W {
        self.variant(CHMODEW::LOCAL_LOOPBACK)
    }
    #[doc = "Remote loopback"]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut W {
        self.variant(CHMODEW::REMOTE_LOOPBACK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits() >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcck(&self) -> BRSRCCK_R {
        BRSRCCK_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    pub fn filter(&mut self) -> _FILTERW {
        _FILTERW { w: self }
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&mut self) -> _PARW {
        _PARW { w: self }
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcck(&mut self) -> _BRSRCCKW {
        _BRSRCCKW { w: self }
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&mut self) -> _CHMODEW {
        _CHMODEW { w: self }
    }
}
