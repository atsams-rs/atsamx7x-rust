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

/// Possible [`crate::ehal::timer::Cancel::cancel`] errors.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CountDownError {
    /// The timer is disabled and was thus never started.
    Disabled,
    /// The timer countdown has already expired.
    Expired,
}

pub(crate) mod private {
    /// Supertrait for all public traits in this HAL. Disallows a downstream crate to provide potentially unsound trait implementations.
    /// Refer to <https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed>.
    pub trait Sealed {}
}
pub(crate) use private::Sealed;

/// Provides generic interrupt event management
/// ---
///
/// Exposes methods for listening, unlistening and differentiating between interrupt events.
pub mod events {
    use super::PhantomData;

    /// A generic interrupt iterator
    /// ---
    ///
    /// This interrupt iterator is used by the [`EventHandler`] it yields
    /// an event source for every triggered event.
    pub struct EventIterator<EventEnum: Clone + Copy + TryFrom<u32>> {
        /// The interrupt id currently being checked
        idx: u32,
        /// The entire status register masked with the interrupt mask register
        irq: u32,
        /// The event enumerator used by the event iterator.
        _event_enum: PhantomData<EventEnum>,
    }

    impl<EventEnum: Clone + Copy + TryFrom<u32>> EventIterator<EventEnum> {
        fn new(irq: u32) -> Self {
            Self {
                idx: 0,
                irq,
                _event_enum: PhantomData,
            }
        }
    }

    impl<EventEnum: Clone + Copy + TryFrom<u32>> Iterator for EventIterator<EventEnum> {
        type Item = EventEnum;

        fn next(&mut self) -> Option<Self::Item> {
            match self.idx {
                32.. => {
                    // No more events to check
                    // return
                    None
                }
                idx if self.irq & (1 << idx) != 0 => {
                    // An event has occurred
                    let mask = 1 << idx;
                    self.idx += 1;
                    // Check if it is a valid event, if not just continue
                    match Self::Item::try_from(mask) {
                        Ok(val) => Some(val),
                        Err(_) => self.next(),
                    }
                }
                _ => {
                    // That event was not pending, continue
                    self.idx += 1;
                    self.next()
                }
            }
        }
    }

    /// A generic interface for handling interrupt events
    /// ---
    ///
    /// This exposes methods for [`listening`](EventHandler::listen)/[`unlistening`](EventHandler::unlisten) to single or multiple event sources at once.
    /// It also exposes the [`EventHandler::events`] method that allows for differentiation between event sources.
    pub trait EventHandler: super::Sealed {
        /// The event enum used for [`listening`](EventHandler::listen), [`unlistening`](EventHandler::unlisten)
        /// as well as [`differentiating between interrupts`](EventHandler::events)
        type EventSource: Clone + Copy + TryFrom<u32>;

        /// Starts listening for a given [`Self::EventSource`].
        fn listen(&mut self, event: Self::EventSource);

        /// Stops listening for a given [`Self::EventSource`].
        fn unlisten(&mut self, event: Self::EventSource);

        /// Listens for a set of [`Self::EventSource`].
        fn listen_slice(&mut self, events: &[Self::EventSource]) {
            for event in events.iter() {
                self.listen(*event);
            }
        }

        /// Stops listening for a set of [`Self::EventSource`].
        fn unlisten_slice(&mut self, events: &[Self::EventSource]) {
            for event in events.iter() {
                self.unlisten(*event);
            }
        }

        // Reads the interrupt status register of the peripheral. An operation that in most cases clears the pending interrupt.
        #[doc(hidden)]
        fn irq(&mut self) -> u32;

        /// Returns an iterator for all of the [`Self::EventSource`]s that have ocurred since the last call.
        fn events(&mut self) -> EventIterator<Self::EventSource> {
            EventIterator::new(self.irq())
        }
    }
}
