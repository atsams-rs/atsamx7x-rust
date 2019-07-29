#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICM_ISR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RHC_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RDM_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RBE_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RWC_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type REC_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RSU_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type URAD_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Region Hash Completed"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch"]
    #[inline(always)]
    pub fn rdm(&self) -> RDM_R {
        RDM_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region Bus Error"]
    #[inline(always)]
    pub fn rbe(&self) -> RBE_R {
        RBE_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected"]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Region Status Updated Detected"]
    #[inline(always)]
    pub fn rsu(&self) -> RSU_R {
        RSU_R::new(((self.bits() >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Status"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
}
