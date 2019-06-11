#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_MCKR {
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
#[doc = "Possible values of the field `CSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSR {
    #[doc = "Slow Clock is selected"]
    SLOW_CLK,
    #[doc = "Main Clock is selected"]
    MAIN_CLK,
    #[doc = "PLLA Clock is selected"]
    PLLA_CLK,
    #[doc = "Divided UPLL Clock is selected"]
    UPLL_CLK,
}
impl CSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSSR::SLOW_CLK => 0,
            CSSR::MAIN_CLK => 1,
            CSSR::PLLA_CLK => 2,
            CSSR::UPLL_CLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSSR {
        match value {
            0 => CSSR::SLOW_CLK,
            1 => CSSR::MAIN_CLK,
            2 => CSSR::PLLA_CLK,
            3 => CSSR::UPLL_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSSR::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline]
    pub fn is_main_clk(&self) -> bool {
        *self == CSSR::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSSR::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSSR::UPLL_CLK
    }
}
#[doc = "Possible values of the field `PRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESR {
    #[doc = "Selected clock"]
    CLK_1,
    #[doc = "Selected clock divided by 2"]
    CLK_2,
    #[doc = "Selected clock divided by 4"]
    CLK_4,
    #[doc = "Selected clock divided by 8"]
    CLK_8,
    #[doc = "Selected clock divided by 16"]
    CLK_16,
    #[doc = "Selected clock divided by 32"]
    CLK_32,
    #[doc = "Selected clock divided by 64"]
    CLK_64,
    #[doc = "Selected clock divided by 3"]
    CLK_3,
}
impl PRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESR::CLK_1 => 0,
            PRESR::CLK_2 => 1,
            PRESR::CLK_4 => 2,
            PRESR::CLK_8 => 3,
            PRESR::CLK_16 => 4,
            PRESR::CLK_32 => 5,
            PRESR::CLK_64 => 6,
            PRESR::CLK_3 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESR {
        match value {
            0 => PRESR::CLK_1,
            1 => PRESR::CLK_2,
            2 => PRESR::CLK_4,
            3 => PRESR::CLK_8,
            4 => PRESR::CLK_16,
            5 => PRESR::CLK_32,
            6 => PRESR::CLK_64,
            7 => PRESR::CLK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline]
    pub fn is_clk_1(&self) -> bool {
        *self == PRESR::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline]
    pub fn is_clk_2(&self) -> bool {
        *self == PRESR::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline]
    pub fn is_clk_4(&self) -> bool {
        *self == PRESR::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline]
    pub fn is_clk_8(&self) -> bool {
        *self == PRESR::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline]
    pub fn is_clk_16(&self) -> bool {
        *self == PRESR::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline]
    pub fn is_clk_32(&self) -> bool {
        *self == PRESR::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline]
    pub fn is_clk_64(&self) -> bool {
        *self == PRESR::CLK_64
    }
    #[doc = "Checks if the value of the field is `CLK_3`"]
    #[inline]
    pub fn is_clk_3(&self) -> bool {
        *self == PRESR::CLK_3
    }
}
#[doc = "Possible values of the field `MDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIVR {
    #[doc = "Master Clock is Prescaler Output Clock divided by 1."]
    EQ_PCK,
    #[doc = "Master Clock is Prescaler Output Clock divided by 2."]
    PCK_DIV2,
    #[doc = "Master Clock is Prescaler Output Clock divided by 4."]
    PCK_DIV4,
    #[doc = "Master Clock is Prescaler Output Clock divided by 3."]
    PCK_DIV3,
}
impl MDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MDIVR::EQ_PCK => 0,
            MDIVR::PCK_DIV2 => 1,
            MDIVR::PCK_DIV4 => 2,
            MDIVR::PCK_DIV3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MDIVR {
        match value {
            0 => MDIVR::EQ_PCK,
            1 => MDIVR::PCK_DIV2,
            2 => MDIVR::PCK_DIV4,
            3 => MDIVR::PCK_DIV3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EQ_PCK`"]
    #[inline]
    pub fn is_eq_pck(&self) -> bool {
        *self == MDIVR::EQ_PCK
    }
    #[doc = "Checks if the value of the field is `PCK_DIV2`"]
    #[inline]
    pub fn is_pck_div2(&self) -> bool {
        *self == MDIVR::PCK_DIV2
    }
    #[doc = "Checks if the value of the field is `PCK_DIV4`"]
    #[inline]
    pub fn is_pck_div4(&self) -> bool {
        *self == MDIVR::PCK_DIV4
    }
    #[doc = "Checks if the value of the field is `PCK_DIV3`"]
    #[inline]
    pub fn is_pck_div3(&self) -> bool {
        *self == MDIVR::PCK_DIV3
    }
}
#[doc = r" Value of the field"]
pub struct UPLLDIV2R {
    bits: bool,
}
impl UPLLDIV2R {
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
#[doc = "Values that can be written to the field `CSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSW {
    #[doc = "Slow Clock is selected"]
    SLOW_CLK,
    #[doc = "Main Clock is selected"]
    MAIN_CLK,
    #[doc = "PLLA Clock is selected"]
    PLLA_CLK,
    #[doc = "Divided UPLL Clock is selected"]
    UPLL_CLK,
}
impl CSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSSW::SLOW_CLK => 0,
            CSSW::MAIN_CLK => 1,
            CSSW::PLLA_CLK => 2,
            CSSW::UPLL_CLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slow Clock is selected"]
    #[inline]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSSW::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSSW::MAIN_CLK)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSSW::PLLA_CLK)
    }
    #[doc = "Divided UPLL Clock is selected"]
    #[inline]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSSW::UPLL_CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESW {
    #[doc = "Selected clock"]
    CLK_1,
    #[doc = "Selected clock divided by 2"]
    CLK_2,
    #[doc = "Selected clock divided by 4"]
    CLK_4,
    #[doc = "Selected clock divided by 8"]
    CLK_8,
    #[doc = "Selected clock divided by 16"]
    CLK_16,
    #[doc = "Selected clock divided by 32"]
    CLK_32,
    #[doc = "Selected clock divided by 64"]
    CLK_64,
    #[doc = "Selected clock divided by 3"]
    CLK_3,
}
impl PRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESW::CLK_1 => 0,
            PRESW::CLK_2 => 1,
            PRESW::CLK_4 => 2,
            PRESW::CLK_8 => 3,
            PRESW::CLK_16 => 4,
            PRESW::CLK_32 => 5,
            PRESW::CLK_64 => 6,
            PRESW::CLK_3 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selected clock"]
    #[inline]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRESW::CLK_1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRESW::CLK_2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRESW::CLK_4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRESW::CLK_8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRESW::CLK_16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRESW::CLK_32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRESW::CLK_64)
    }
    #[doc = "Selected clock divided by 3"]
    #[inline]
    pub fn clk_3(self) -> &'a mut W {
        self.variant(PRESW::CLK_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIVW {
    #[doc = "Master Clock is Prescaler Output Clock divided by 1."]
    EQ_PCK,
    #[doc = "Master Clock is Prescaler Output Clock divided by 2."]
    PCK_DIV2,
    #[doc = "Master Clock is Prescaler Output Clock divided by 4."]
    PCK_DIV4,
    #[doc = "Master Clock is Prescaler Output Clock divided by 3."]
    PCK_DIV3,
}
impl MDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDIVW::EQ_PCK => 0,
            MDIVW::PCK_DIV2 => 1,
            MDIVW::PCK_DIV4 => 2,
            MDIVW::PCK_DIV3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 1."]
    #[inline]
    pub fn eq_pck(self) -> &'a mut W {
        self.variant(MDIVW::EQ_PCK)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 2."]
    #[inline]
    pub fn pck_div2(self) -> &'a mut W {
        self.variant(MDIVW::PCK_DIV2)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 4."]
    #[inline]
    pub fn pck_div4(self) -> &'a mut W {
        self.variant(MDIVW::PCK_DIV4)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 3."]
    #[inline]
    pub fn pck_div3(self) -> &'a mut W {
        self.variant(MDIVW::PCK_DIV3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPLLDIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _UPLLDIV2W<'a> {
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
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline]
    pub fn css(&self) -> CSSR {
        CSSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline]
    pub fn pres(&self) -> PRESR {
        PRESR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline]
    pub fn mdiv(&self) -> MDIVR {
        MDIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline]
    pub fn uplldiv2(&self) -> UPLLDIV2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPLLDIV2R { bits }
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
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline]
    pub fn css(&mut self) -> _CSSW {
        _CSSW { w: self }
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline]
    pub fn pres(&mut self) -> _PRESW {
        _PRESW { w: self }
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline]
    pub fn mdiv(&mut self) -> _MDIVW {
        _MDIVW { w: self }
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline]
    pub fn uplldiv2(&mut self) -> _UPLLDIV2W {
        _UPLLDIV2W { w: self }
    }
}
