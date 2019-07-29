#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::US_NER {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type NB_ERRORS_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Number of Errors"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NB_ERRORS_R {
        NB_ERRORS_R::new((self.bits() & 0xff) as u8)
    }
}
