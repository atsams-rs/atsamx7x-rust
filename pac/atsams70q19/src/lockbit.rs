#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub lockbit_word0: LOCKBIT_WORD0,
}
#[doc = "Lock Bits Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit_word0](lockbit_word0) module"]
pub type LOCKBIT_WORD0 = crate::Reg<u32, _LOCKBIT_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKBIT_WORD0;
#[doc = "`read()` method returns [lockbit_word0::R](lockbit_word0::R) reader structure"]
impl crate::Readable for LOCKBIT_WORD0 {}
#[doc = "`write(|w| ..)` method takes [lockbit_word0::W](lockbit_word0::W) writer structure"]
impl crate::Writable for LOCKBIT_WORD0 {}
#[doc = "Lock Bits Word 0"]
pub mod lockbit_word0;
