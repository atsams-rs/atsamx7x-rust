#[doc = "Register `CCCR` reader"]
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCCR` writer"]
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Initialization (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Normal operation."]
    DISABLED = 0,
    #[doc = "1: Initialization is started."]
    ENABLED = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization (read/write)"]
pub type INIT_R = crate::BitReader<INIT_A>;
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::DISABLED,
            true => INIT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INIT_A::ENABLED
    }
}
#[doc = "Field `INIT` writer - Initialization (read/write)"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, INIT_A, O>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INIT_A::DISABLED)
    }
    #[doc = "Initialization is started."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INIT_A::ENABLED)
    }
}
#[doc = "Configuration Change Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: The processor has no write access to the protected configuration registers."]
    PROTECTED = 0,
    #[doc = "1: The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    CONFIGURABLE = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration Change Enable (read/write, write protection)"]
pub type CCE_R = crate::BitReader<CCE_A>;
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::PROTECTED,
            true => CCE_A::CONFIGURABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == CCE_A::PROTECTED
    }
    #[doc = "Checks if the value of the field is `CONFIGURABLE`"]
    #[inline(always)]
    pub fn is_configurable(&self) -> bool {
        *self == CCE_A::CONFIGURABLE
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable (read/write, write protection)"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, CCE_A, O>;
impl<'a, const O: u8> CCE_W<'a, O> {
    #[doc = "The processor has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(CCE_A::PROTECTED)
    }
    #[doc = "The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    #[inline(always)]
    pub fn configurable(self) -> &'a mut W {
        self.variant(CCE_A::CONFIGURABLE)
    }
}
#[doc = "Restricted Operation Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASM_A {
    #[doc = "0: Normal CAN operation."]
    NORMAL = 0,
    #[doc = "1: Restricted operation mode active."]
    RESTRICTED = 1,
}
impl From<ASM_A> for bool {
    #[inline(always)]
    fn from(variant: ASM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASM` reader - Restricted Operation Mode (read/write, write protection against '1')"]
pub type ASM_R = crate::BitReader<ASM_A>;
impl ASM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASM_A {
        match self.bits {
            false => ASM_A::NORMAL,
            true => ASM_A::RESTRICTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ASM_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESTRICTED`"]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        *self == ASM_A::RESTRICTED
    }
}
#[doc = "Field `ASM` writer - Restricted Operation Mode (read/write, write protection against '1')"]
pub type ASM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, ASM_A, O>;
impl<'a, const O: u8> ASM_W<'a, O> {
    #[doc = "Normal CAN operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ASM_A::NORMAL)
    }
    #[doc = "Restricted operation mode active."]
    #[inline(always)]
    pub fn restricted(self) -> &'a mut W {
        self.variant(ASM_A::RESTRICTED)
    }
}
#[doc = "Field `CSA` reader - Clock Stop Acknowledge (read-only)"]
pub type CSA_R = crate::BitReader<bool>;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge (read-only)"]
pub type CSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Clock Stop Request (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR_A {
    #[doc = "0: No clock stop is requested."]
    NO_CLOCK_STOP = 0,
    #[doc = "1: Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    CLOCK_STOP = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSR` reader - Clock Stop Request (read/write)"]
pub type CSR_R = crate::BitReader<CSR_A>;
impl CSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::NO_CLOCK_STOP,
            true => CSR_A::CLOCK_STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK_STOP`"]
    #[inline(always)]
    pub fn is_no_clock_stop(&self) -> bool {
        *self == CSR_A::NO_CLOCK_STOP
    }
    #[doc = "Checks if the value of the field is `CLOCK_STOP`"]
    #[inline(always)]
    pub fn is_clock_stop(&self) -> bool {
        *self == CSR_A::CLOCK_STOP
    }
}
#[doc = "Field `CSR` writer - Clock Stop Request (read/write)"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, CSR_A, O>;
impl<'a, const O: u8> CSR_W<'a, O> {
    #[doc = "No clock stop is requested."]
    #[inline(always)]
    pub fn no_clock_stop(self) -> &'a mut W {
        self.variant(CSR_A::NO_CLOCK_STOP)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn clock_stop(self) -> &'a mut W {
        self.variant(CSR_A::CLOCK_STOP)
    }
}
#[doc = "Bus Monitoring Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MON_A {
    #[doc = "0: Bus Monitoring mode is disabled."]
    DISABLED = 0,
    #[doc = "1: Bus Monitoring mode is enabled."]
    ENABLED = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MON` reader - Bus Monitoring Mode (read/write, write protection against '1')"]
pub type MON_R = crate::BitReader<MON_A>;
impl MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::DISABLED,
            true => MON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MON_A::ENABLED
    }
}
#[doc = "Field `MON` writer - Bus Monitoring Mode (read/write, write protection against '1')"]
pub type MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, MON_A, O>;
impl<'a, const O: u8> MON_W<'a, O> {
    #[doc = "Bus Monitoring mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MON_A::DISABLED)
    }
    #[doc = "Bus Monitoring mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MON_A::ENABLED)
    }
}
#[doc = "Disable Automatic Retransmission (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAR_A {
    #[doc = "0: Automatic retransmission of messages not transmitted successfully enabled."]
    AUTO_RETX = 0,
    #[doc = "1: Automatic retransmission disabled."]
    NO_AUTO_RETX = 1,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAR` reader - Disable Automatic Retransmission (read/write, write protection)"]
pub type DAR_R = crate::BitReader<DAR_A>;
impl DAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::AUTO_RETX,
            true => DAR_A::NO_AUTO_RETX,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_RETX`"]
    #[inline(always)]
    pub fn is_auto_retx(&self) -> bool {
        *self == DAR_A::AUTO_RETX
    }
    #[doc = "Checks if the value of the field is `NO_AUTO_RETX`"]
    #[inline(always)]
    pub fn is_no_auto_retx(&self) -> bool {
        *self == DAR_A::NO_AUTO_RETX
    }
}
#[doc = "Field `DAR` writer - Disable Automatic Retransmission (read/write, write protection)"]
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, DAR_A, O>;
impl<'a, const O: u8> DAR_W<'a, O> {
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled."]
    #[inline(always)]
    pub fn auto_retx(self) -> &'a mut W {
        self.variant(DAR_A::AUTO_RETX)
    }
    #[doc = "Automatic retransmission disabled."]
    #[inline(always)]
    pub fn no_auto_retx(self) -> &'a mut W {
        self.variant(DAR_A::NO_AUTO_RETX)
    }
}
#[doc = "Test Mode Enable (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_A {
    #[doc = "0: Normal operation, MCAN_TEST register holds reset values."]
    DISABLED = 0,
    #[doc = "1: Test mode, write access to MCAN_TEST register enabled."]
    ENABLED = 1,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST` reader - Test Mode Enable (read/write, write protection against '1')"]
pub type TEST_R = crate::BitReader<TEST_A>;
impl TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::DISABLED,
            true => TEST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEST_A::ENABLED
    }
}
#[doc = "Field `TEST` writer - Test Mode Enable (read/write, write protection against '1')"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, TEST_A, O>;
impl<'a, const O: u8> TEST_W<'a, O> {
    #[doc = "Normal operation, MCAN_TEST register holds reset values."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEST_A::DISABLED)
    }
    #[doc = "Test mode, write access to MCAN_TEST register enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEST_A::ENABLED)
    }
}
#[doc = "CAN Mode Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CME_A {
    #[doc = "0: CAN operation according to ISO11898-1 enabled"]
    ISO11898_1 = 0,
    #[doc = "1: CAN FD operation enabled"]
    FD = 1,
}
impl From<CME_A> for u8 {
    #[inline(always)]
    fn from(variant: CME_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CME` reader - CAN Mode Enable (read/write, write protection)"]
pub type CME_R = crate::FieldReader<u8, CME_A>;
impl CME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CME_A> {
        match self.bits {
            0 => Some(CME_A::ISO11898_1),
            1 => Some(CME_A::FD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ISO11898_1`"]
    #[inline(always)]
    pub fn is_iso11898_1(&self) -> bool {
        *self == CME_A::ISO11898_1
    }
    #[doc = "Checks if the value of the field is `FD`"]
    #[inline(always)]
    pub fn is_fd(&self) -> bool {
        *self == CME_A::FD
    }
}
#[doc = "Field `CME` writer - CAN Mode Enable (read/write, write protection)"]
pub type CME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCCR_SPEC, u8, CME_A, 2, O>;
impl<'a, const O: u8> CME_W<'a, O> {
    #[doc = "CAN operation according to ISO11898-1 enabled"]
    #[inline(always)]
    pub fn iso11898_1(self) -> &'a mut W {
        self.variant(CME_A::ISO11898_1)
    }
    #[doc = "CAN FD operation enabled"]
    #[inline(always)]
    pub fn fd(self) -> &'a mut W {
        self.variant(CME_A::FD)
    }
}
#[doc = "CAN Mode Request (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMR_A {
    #[doc = "0: No mode change"]
    NO_CHANGE = 0,
    #[doc = "1: Request CAN FD operation"]
    FD = 1,
    #[doc = "2: Request CAN FD operation with bit rate switching"]
    FD_BITRATE_SWITCH = 2,
    #[doc = "3: Request CAN operation according ISO11898-1"]
    ISO11898_1 = 3,
}
impl From<CMR_A> for u8 {
    #[inline(always)]
    fn from(variant: CMR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMR` reader - CAN Mode Request (read/write)"]
pub type CMR_R = crate::FieldReader<u8, CMR_A>;
impl CMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMR_A {
        match self.bits {
            0 => CMR_A::NO_CHANGE,
            1 => CMR_A::FD,
            2 => CMR_A::FD_BITRATE_SWITCH,
            3 => CMR_A::ISO11898_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CMR_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `FD`"]
    #[inline(always)]
    pub fn is_fd(&self) -> bool {
        *self == CMR_A::FD
    }
    #[doc = "Checks if the value of the field is `FD_BITRATE_SWITCH`"]
    #[inline(always)]
    pub fn is_fd_bitrate_switch(&self) -> bool {
        *self == CMR_A::FD_BITRATE_SWITCH
    }
    #[doc = "Checks if the value of the field is `ISO11898_1`"]
    #[inline(always)]
    pub fn is_iso11898_1(&self) -> bool {
        *self == CMR_A::ISO11898_1
    }
}
#[doc = "Field `CMR` writer - CAN Mode Request (read/write)"]
pub type CMR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCCR_SPEC, u8, CMR_A, 2, O>;
impl<'a, const O: u8> CMR_W<'a, O> {
    #[doc = "No mode change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CMR_A::NO_CHANGE)
    }
    #[doc = "Request CAN FD operation"]
    #[inline(always)]
    pub fn fd(self) -> &'a mut W {
        self.variant(CMR_A::FD)
    }
    #[doc = "Request CAN FD operation with bit rate switching"]
    #[inline(always)]
    pub fn fd_bitrate_switch(self) -> &'a mut W {
        self.variant(CMR_A::FD_BITRATE_SWITCH)
    }
    #[doc = "Request CAN operation according ISO11898-1"]
    #[inline(always)]
    pub fn iso11898_1(self) -> &'a mut W {
        self.variant(CMR_A::ISO11898_1)
    }
}
#[doc = "Field `FDO` reader - CAN FD Operation (read-only)"]
pub type FDO_R = crate::BitReader<bool>;
#[doc = "Field `FDO` writer - CAN FD Operation (read-only)"]
pub type FDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `FDBS` reader - CAN FD Bit Rate Switching (read-only)"]
pub type FDBS_R = crate::BitReader<bool>;
#[doc = "Field `FDBS` writer - CAN FD Bit Rate Switching (read-only)"]
pub type FDBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `TXP` reader - Transmit Pause (read/write, write protection)"]
pub type TXP_R = crate::BitReader<bool>;
#[doc = "Field `TXP` writer - Transmit Pause (read/write, write protection)"]
pub type TXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CAN Mode Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cme(&self) -> CME_R {
        CME_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CAN Mode Request (read/write)"]
    #[inline(always)]
    pub fn cmr(&self) -> CMR_R {
        CMR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - CAN FD Operation (read-only)"]
    #[inline(always)]
    pub fn fdo(&self) -> FDO_R {
        FDO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN FD Bit Rate Switching (read-only)"]
    #[inline(always)]
    pub fn fdbs(&self) -> FDBS_R {
        FDBS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W<1> {
        CCE_W::new(self)
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W<2> {
        ASM_W::new(self)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    pub fn csa(&mut self) -> CSA_W<3> {
        CSA_W::new(self)
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<4> {
        CSR_W::new(self)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W<5> {
        MON_W::new(self)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<6> {
        DAR_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<7> {
        TEST_W::new(self)
    }
    #[doc = "Bits 8:9 - CAN Mode Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cme(&mut self) -> CME_W<8> {
        CME_W::new(self)
    }
    #[doc = "Bits 10:11 - CAN Mode Request (read/write)"]
    #[inline(always)]
    pub fn cmr(&mut self) -> CMR_W<10> {
        CMR_W::new(self)
    }
    #[doc = "Bit 12 - CAN FD Operation (read-only)"]
    #[inline(always)]
    pub fn fdo(&mut self) -> FDO_W<12> {
        FDO_W::new(self)
    }
    #[doc = "Bit 13 - CAN FD Bit Rate Switching (read-only)"]
    #[inline(always)]
    pub fn fdbs(&mut self) -> FDBS_W<13> {
        FDBS_W::new(self)
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W<14> {
        TXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](index.html) module"]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cccr::R](R) reader structure"]
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cccr::W](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCCR to value 0"]
impl crate::Resettable for CCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
