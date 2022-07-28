/*!
Crate generics
---

A collection of traits and structures used in a generic context
throughout the crate.

*/

use core::marker::PhantomData;

/// A `T` singleton that is consumed on configuration, returning the
/// `T`.
pub struct Token<T>(PhantomData<T>);

impl<T> Default for Token<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
