use core::marker;
#[doc = r"Converting enumerated values to bits"]
pub trait ToBits<N> {
    #[doc = r"Conversion method"]
    fn _bits(&self) -> N;
}
#[doc = r"Value read from the register"]
pub struct FR<U, T> {
    bits: U,
    _reg: marker::PhantomData<T>,
}
impl<U, T, FI> PartialEq<FI> for FR<U, T>
where
    U: PartialEq,
    FI: ToBits<U>,
{
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&other._bits())
    }
}
impl<U, T> FR<U, T>
where
    U: Copy,
{
    #[doc = r"Create new instance of reader"]
    #[inline(always)]
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
    #[doc = r"Read raw bits from field"]
    #[inline(always)]
    pub fn bits(&self) -> U {
        self.bits
    }
}
impl<FI> FR<bool, FI> {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
