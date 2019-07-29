#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DACC_TRIGR {
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
#[doc = "Possible values of the field `TRGEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN0R {
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    DIS,
    #[doc = "External trigger mode enabled."]
    EN,
}
impl crate::ToBits<bool> for TRGEN0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRGEN0R::DIS => false,
            TRGEN0R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGEN0_R = crate::FR<bool, TRGEN0R>;
impl TRGEN0_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN0R::EN
    }
}
#[doc = "Values that can be written to the field `TRGEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN0W {
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    DIS,
    #[doc = "External trigger mode enabled."]
    EN,
}
impl TRGEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGEN0W::DIS => false,
            TRGEN0W::EN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _TRGEN0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN0W::DIS)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN0W::EN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `TRGEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN1R {
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    DIS,
    #[doc = "External trigger mode enabled."]
    EN,
}
impl crate::ToBits<bool> for TRGEN1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRGEN1R::DIS => false,
            TRGEN1R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGEN1_R = crate::FR<bool, TRGEN1R>;
impl TRGEN1_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN1R::EN
    }
}
#[doc = "Values that can be written to the field `TRGEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN1W {
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    DIS,
    #[doc = "External trigger mode enabled."]
    EN,
}
impl TRGEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGEN1W::DIS => false,
            TRGEN1W::EN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _TRGEN1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN1W::DIS)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN1W::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `TRGSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSEL0R {
    #[doc = "DATRG"]
    TRGSEL0,
    #[doc = "TC0 output"]
    TRGSEL1,
    #[doc = "TC1 output"]
    TRGSEL2,
    #[doc = "TC2 output"]
    TRGSEL3,
    #[doc = "PWM0 event 0"]
    TRGSEL4,
    #[doc = "PWM0 event 1"]
    TRGSEL5,
    #[doc = "PWM1 event 0"]
    TRGSEL6,
    #[doc = "PWM1 event 1"]
    TRGSEL7,
}
impl crate::ToBits<u8> for TRGSEL0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRGSEL0R::TRGSEL0 => 0,
            TRGSEL0R::TRGSEL1 => 1,
            TRGSEL0R::TRGSEL2 => 2,
            TRGSEL0R::TRGSEL3 => 3,
            TRGSEL0R::TRGSEL4 => 4,
            TRGSEL0R::TRGSEL5 => 5,
            TRGSEL0R::TRGSEL6 => 6,
            TRGSEL0R::TRGSEL7 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGSEL0_R = crate::FR<u8, TRGSEL0R>;
impl TRGSEL0_R {
    #[doc = "Checks if the value of the field is `TRGSEL0`"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL0R::TRGSEL0
    }
    #[doc = "Checks if the value of the field is `TRGSEL1`"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL0R::TRGSEL1
    }
    #[doc = "Checks if the value of the field is `TRGSEL2`"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL0R::TRGSEL2
    }
    #[doc = "Checks if the value of the field is `TRGSEL3`"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL0R::TRGSEL3
    }
    #[doc = "Checks if the value of the field is `TRGSEL4`"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == TRGSEL0R::TRGSEL4
    }
    #[doc = "Checks if the value of the field is `TRGSEL5`"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == TRGSEL0R::TRGSEL5
    }
    #[doc = "Checks if the value of the field is `TRGSEL6`"]
    #[inline(always)]
    pub fn is_trgsel6(&self) -> bool {
        *self == TRGSEL0R::TRGSEL6
    }
    #[doc = "Checks if the value of the field is `TRGSEL7`"]
    #[inline(always)]
    pub fn is_trgsel7(&self) -> bool {
        *self == TRGSEL0R::TRGSEL7
    }
}
#[doc = "Values that can be written to the field `TRGSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSEL0W {
    #[doc = "DATRG"]
    TRGSEL0,
    #[doc = "TC0 output"]
    TRGSEL1,
    #[doc = "TC1 output"]
    TRGSEL2,
    #[doc = "TC2 output"]
    TRGSEL3,
    #[doc = "PWM0 event 0"]
    TRGSEL4,
    #[doc = "PWM0 event 1"]
    TRGSEL5,
    #[doc = "PWM1 event 0"]
    TRGSEL6,
    #[doc = "PWM1 event 1"]
    TRGSEL7,
}
impl TRGSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGSEL0W::TRGSEL0 => 0,
            TRGSEL0W::TRGSEL1 => 1,
            TRGSEL0W::TRGSEL2 => 2,
            TRGSEL0W::TRGSEL3 => 3,
            TRGSEL0W::TRGSEL4 => 4,
            TRGSEL0W::TRGSEL5 => 5,
            TRGSEL0W::TRGSEL6 => 6,
            TRGSEL0W::TRGSEL7 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSEL0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DATRG"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL0)
    }
    #[doc = "TC0 output"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL1)
    }
    #[doc = "TC1 output"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL2)
    }
    #[doc = "TC2 output"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL3)
    }
    #[doc = "PWM0 event 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL4)
    }
    #[doc = "PWM0 event 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL5)
    }
    #[doc = "PWM1 event 0"]
    #[inline(always)]
    pub fn trgsel6(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL6)
    }
    #[doc = "PWM1 event 1"]
    #[inline(always)]
    pub fn trgsel7(self) -> &'a mut W {
        self.variant(TRGSEL0W::TRGSEL7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `TRGSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSEL1R {
    #[doc = "DATRG"]
    TRGSEL0,
    #[doc = "TC0 output"]
    TRGSEL1,
    #[doc = "TC1 output"]
    TRGSEL2,
    #[doc = "TC2 output"]
    TRGSEL3,
    #[doc = "PWM0 event 0"]
    TRGSEL4,
    #[doc = "PWM0 event 1"]
    TRGSEL5,
    #[doc = "PWM1 event 0"]
    TRGSEL6,
    #[doc = "PWM1 event 1"]
    TRGSEL7,
}
impl crate::ToBits<u8> for TRGSEL1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRGSEL1R::TRGSEL0 => 0,
            TRGSEL1R::TRGSEL1 => 1,
            TRGSEL1R::TRGSEL2 => 2,
            TRGSEL1R::TRGSEL3 => 3,
            TRGSEL1R::TRGSEL4 => 4,
            TRGSEL1R::TRGSEL5 => 5,
            TRGSEL1R::TRGSEL6 => 6,
            TRGSEL1R::TRGSEL7 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGSEL1_R = crate::FR<u8, TRGSEL1R>;
impl TRGSEL1_R {
    #[doc = "Checks if the value of the field is `TRGSEL0`"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL1R::TRGSEL0
    }
    #[doc = "Checks if the value of the field is `TRGSEL1`"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL1R::TRGSEL1
    }
    #[doc = "Checks if the value of the field is `TRGSEL2`"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL1R::TRGSEL2
    }
    #[doc = "Checks if the value of the field is `TRGSEL3`"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL1R::TRGSEL3
    }
    #[doc = "Checks if the value of the field is `TRGSEL4`"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == TRGSEL1R::TRGSEL4
    }
    #[doc = "Checks if the value of the field is `TRGSEL5`"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == TRGSEL1R::TRGSEL5
    }
    #[doc = "Checks if the value of the field is `TRGSEL6`"]
    #[inline(always)]
    pub fn is_trgsel6(&self) -> bool {
        *self == TRGSEL1R::TRGSEL6
    }
    #[doc = "Checks if the value of the field is `TRGSEL7`"]
    #[inline(always)]
    pub fn is_trgsel7(&self) -> bool {
        *self == TRGSEL1R::TRGSEL7
    }
}
#[doc = "Values that can be written to the field `TRGSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSEL1W {
    #[doc = "DATRG"]
    TRGSEL0,
    #[doc = "TC0 output"]
    TRGSEL1,
    #[doc = "TC1 output"]
    TRGSEL2,
    #[doc = "TC2 output"]
    TRGSEL3,
    #[doc = "PWM0 event 0"]
    TRGSEL4,
    #[doc = "PWM0 event 1"]
    TRGSEL5,
    #[doc = "PWM1 event 0"]
    TRGSEL6,
    #[doc = "PWM1 event 1"]
    TRGSEL7,
}
impl TRGSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGSEL1W::TRGSEL0 => 0,
            TRGSEL1W::TRGSEL1 => 1,
            TRGSEL1W::TRGSEL2 => 2,
            TRGSEL1W::TRGSEL3 => 3,
            TRGSEL1W::TRGSEL4 => 4,
            TRGSEL1W::TRGSEL5 => 5,
            TRGSEL1W::TRGSEL6 => 6,
            TRGSEL1W::TRGSEL7 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSEL1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DATRG"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL0)
    }
    #[doc = "TC0 output"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL1)
    }
    #[doc = "TC1 output"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL2)
    }
    #[doc = "TC2 output"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL3)
    }
    #[doc = "PWM0 event 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL4)
    }
    #[doc = "PWM0 event 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL5)
    }
    #[doc = "PWM1 event 0"]
    #[inline(always)]
    pub fn trgsel6(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL6)
    }
    #[doc = "PWM1 event 1"]
    #[inline(always)]
    pub fn trgsel7(self) -> &'a mut W {
        self.variant(TRGSEL1W::TRGSEL7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `OSR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSR0R {
    #[doc = "OSR = 1"]
    OSR_1,
    #[doc = "OSR = 2"]
    OSR_2,
    #[doc = "OSR = 4"]
    OSR_4,
    #[doc = "OSR = 8"]
    OSR_8,
    #[doc = "OSR = 16"]
    OSR_16,
    #[doc = "OSR = 32"]
    OSR_32,
}
impl crate::ToBits<u8> for OSR0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            OSR0R::OSR_1 => 0,
            OSR0R::OSR_2 => 1,
            OSR0R::OSR_4 => 2,
            OSR0R::OSR_8 => 3,
            OSR0R::OSR_16 => 4,
            OSR0R::OSR_32 => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OSR0_R = crate::FR<u8, OSR0R>;
impl OSR0_R {
    #[doc = "Checks if the value of the field is `OSR_1`"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == OSR0R::OSR_1
    }
    #[doc = "Checks if the value of the field is `OSR_2`"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == OSR0R::OSR_2
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSR0R::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSR0R::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSR0R::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_32`"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == OSR0R::OSR_32
    }
}
#[doc = "Values that can be written to the field `OSR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSR0W {
    #[doc = "OSR = 1"]
    OSR_1,
    #[doc = "OSR = 2"]
    OSR_2,
    #[doc = "OSR = 4"]
    OSR_4,
    #[doc = "OSR = 8"]
    OSR_8,
    #[doc = "OSR = 16"]
    OSR_16,
    #[doc = "OSR = 32"]
    OSR_32,
}
impl OSR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSR0W::OSR_1 => 0,
            OSR0W::OSR_2 => 1,
            OSR0W::OSR_4 => 2,
            OSR0W::OSR_8 => 3,
            OSR0W::OSR_16 => 4,
            OSR0W::OSR_32 => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OSR0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSR0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut W {
        self.variant(OSR0W::OSR_1)
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut W {
        self.variant(OSR0W::OSR_2)
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSR0W::OSR_4)
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSR0W::OSR_8)
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSR0W::OSR_16)
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut W {
        self.variant(OSR0W::OSR_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `OSR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSR1R {
    #[doc = "OSR = 1"]
    OSR_1,
    #[doc = "OSR = 2"]
    OSR_2,
    #[doc = "OSR = 4"]
    OSR_4,
    #[doc = "OSR = 8"]
    OSR_8,
    #[doc = "OSR = 16"]
    OSR_16,
    #[doc = "OSR = 32"]
    OSR_32,
}
impl crate::ToBits<u8> for OSR1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            OSR1R::OSR_1 => 0,
            OSR1R::OSR_2 => 1,
            OSR1R::OSR_4 => 2,
            OSR1R::OSR_8 => 3,
            OSR1R::OSR_16 => 4,
            OSR1R::OSR_32 => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OSR1_R = crate::FR<u8, OSR1R>;
impl OSR1_R {
    #[doc = "Checks if the value of the field is `OSR_1`"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == OSR1R::OSR_1
    }
    #[doc = "Checks if the value of the field is `OSR_2`"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == OSR1R::OSR_2
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSR1R::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSR1R::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSR1R::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_32`"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == OSR1R::OSR_32
    }
}
#[doc = "Values that can be written to the field `OSR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSR1W {
    #[doc = "OSR = 1"]
    OSR_1,
    #[doc = "OSR = 2"]
    OSR_2,
    #[doc = "OSR = 4"]
    OSR_4,
    #[doc = "OSR = 8"]
    OSR_8,
    #[doc = "OSR = 16"]
    OSR_16,
    #[doc = "OSR = 32"]
    OSR_32,
}
impl OSR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSR1W::OSR_1 => 0,
            OSR1W::OSR_2 => 1,
            OSR1W::OSR_4 => 2,
            OSR1W::OSR_8 => 3,
            OSR1W::OSR_16 => 4,
            OSR1W::OSR_32 => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OSR1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSR1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut W {
        self.variant(OSR1W::OSR_1)
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut W {
        self.variant(OSR1W::OSR_2)
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSR1W::OSR_4)
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSR1W::OSR_8)
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSR1W::OSR_16)
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut W {
        self.variant(OSR1W::OSR_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Trigger Enable of Channel 0"]
    #[inline(always)]
    pub fn trgen0(&self) -> TRGEN0_R {
        TRGEN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Trigger Enable of Channel 1"]
    #[inline(always)]
    pub fn trgen1(&self) -> TRGEN1_R {
        TRGEN1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Trigger Selection of Channel 0"]
    #[inline(always)]
    pub fn trgsel0(&self) -> TRGSEL0_R {
        TRGSEL0_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Trigger Selection of Channel 1"]
    #[inline(always)]
    pub fn trgsel1(&self) -> TRGSEL1_R {
        TRGSEL1_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Over Sampling Ratio of Channel 0"]
    #[inline(always)]
    pub fn osr0(&self) -> OSR0_R {
        OSR0_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Over Sampling Ratio of Channel 1"]
    #[inline(always)]
    pub fn osr1(&self) -> OSR1_R {
        OSR1_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Trigger Enable of Channel 0"]
    #[inline(always)]
    pub fn trgen0(&mut self) -> _TRGEN0W {
        _TRGEN0W { w: self }
    }
    #[doc = "Bit 1 - Trigger Enable of Channel 1"]
    #[inline(always)]
    pub fn trgen1(&mut self) -> _TRGEN1W {
        _TRGEN1W { w: self }
    }
    #[doc = "Bits 4:6 - Trigger Selection of Channel 0"]
    #[inline(always)]
    pub fn trgsel0(&mut self) -> _TRGSEL0W {
        _TRGSEL0W { w: self }
    }
    #[doc = "Bits 8:10 - Trigger Selection of Channel 1"]
    #[inline(always)]
    pub fn trgsel1(&mut self) -> _TRGSEL1W {
        _TRGSEL1W { w: self }
    }
    #[doc = "Bits 16:18 - Over Sampling Ratio of Channel 0"]
    #[inline(always)]
    pub fn osr0(&mut self) -> _OSR0W {
        _OSR0W { w: self }
    }
    #[doc = "Bits 20:22 - Over Sampling Ratio of Channel 1"]
    #[inline(always)]
    pub fn osr1(&mut self) -> _OSR1W {
        _OSR1W { w: self }
    }
}
