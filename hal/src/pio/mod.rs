/*!
Parallel Input/Output Controller
---

The ATSAMx7x family of MCUs expose up to five PIO banks (`PIOA`,
`PIOB`, `PIOC`, `PIOD`, `PIOE`). This module provide an
abstraction on top of these banks, and allows singular [`Pin`]s to
be conveniently configured.

# Banks

Each bank contains up to 32 [`Pin`]s. Each bank has its
corresponding interrupt (e.g. [the `PIOA` interrupt] for [`PIOA`],
[the `PIOB` interrupt] for [`PIOB`], etc.) and a prescaler for
minimum debounce durations. That is, all [`Pin`]s in a bank share
the same debounce configuration.

[the `PIOA` interrupt]: crate::pac::Interrupt::PIOA
[the `PIOB` interrupt]: crate::pac::Interrupt::PIOB

# Pins

Each pin can be configured into an [`Input`], [`Output`], or
[`Peripheral`]. To select the correct [`Peripheral`] for a
[`Pin`], refer to the tables in §6 of the data sheet.

The `Input` type is available for type-constrain convenience: the
input level can be read of a [`Pin`] in any [`PinMode`].

The following [`Pin`] configurations are applicable in any
[`PinMode`]s, mirroring the hardware behavior, and are thus not
typechecked (due to the type noise incurred due to lack of
variadic type lists in Rust):

- interrupt configuration;
- pull direction (or floating);
- input filters (glitch or debounce); and
- Schmitt trigger.

# Interrupts

As mentioned above, all interrupts for the [`Pin`]s in a bank are
OR:ed to a singular interrupt for which a handler can be
installed. A further limitation is that it is not possible to
acknowledge the interrupt of a single [`Pin`] without also
acknowledning the entire bank. This makes the ATSAMx7x prone to
false-positive interrupts when the [`InterruptType`] for [`Pin`]s
are being configured. Consider that a clock signal is being input
on [`Pin<PA01, Input>`], and we want to trigger on
[`InterruptType::FallingEdge`]:

1. When [`BankA`] has been instantiated its peripheral clock is
enabled, and all of its [`Pin`]s are by default triggering on
[`InterruptType::AnyEdge`].

2. Before [`Pin<A01, Input>::set_interrupt`] has been called, the
pin would (potentially) see the rising edge of the signal, and the
corresponding bit in the bank's [ISR] would be set (but no bank
interrupt would fire, because [IMR] has not yet been set).

3. [`Pin<A01, Input>::set_interrupt`] is called for
[`InterruptType::FallingEdge`], but because any potential pending
request in [ISR] cannot be cleared without clearing the whole
bank, the pin interrupt is still pending. [IMR] is then set for
[`Pin<PA01, _>`].

4. The bank's interrupt fires, but the trigger was a rising edge,
not a falling edge.

Edge-cases like the one above cannot be filtered out at the
HAL-level. However, it applies some enforcement to ensure that
[`Pin`] interrupts are not ignored; refer to
[`BankInterrupts::iter`].

[ISR]: crate::pac::pioa::pio_isr
[IMR]: crate::pac::pioa::pio_imr

# Example

The below example configures [`Pin<PA11, Input>`] to trigger on
[`InterruptType::FallingEdge`] with a debounce filter of 50ms.
[`Pin<PB8, Output>`] is then connected to a LED.

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# let pac = hal::pac::Peripherals::take().unwrap();
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));

let banka = BankA::new(
    pac.PIOA,
    &mut mck,
    &slck,
    BankConfiguration {
        min_debounce_duration: hal::fugit::MillisDurationU32::from_ticks(50).convert(),
    },
);
let mut button = banka.pa11.into_input(PullDir::PullUp);
button.set_interrupt(Some(InterruptType::FallingEdge));
button.set_filter(Some(InputFilter::Debounce));

let bankb = BankB::new(pac.PIOB, &mut mck, &slck, BankConfiguration::default());
let led = bankb.pb8.into_output(true); // or false, for an initial low state.
```

# System [`Pin`]s

On reset the following [`Pin`]s:

- [`Pin<PB4, _>`];
- [`Pin<PB5, _>`];
- [`Pin<PB6, _>`];
- [`Pin<PB7, _>`]; and
- [`Pin<PB12, _>`]

are in "system I/O mode" where they provide debug, trace, and flash-erase functionality.
By default, trying to configure these pins yield no effect, but can  be enabled via the `reconfigurable-system-pins` feature.
*/

use crate::generics;
use crate::pac::{
    // All PIO banks below use the same register block definition.
    pioa::RegisterBlock,

    // Banks common to all chip sizes.
    PIOA,
    PIOB,
    PIOD,
};
#[cfg(feature = "pins-144")]
use crate::pac::{PIOC, PIOE};

pub mod pin;
pub use pin::*;

pub mod dynpin;
pub use dynpin::*;

pub mod bank;
pub use bank::*;

mod reg;

use paste::paste;

macro_rules! selector {
    (
        $(
            $Letter:ident
        ),+
    ) => {
        paste! {
            $(
                #[
                    doc = "Multi-purpose type-level variant of [`PeripheralConfig`] (for alternative [`Peripheral`] function " $Letter ") and [`PinBank`]."
                ]
                pub enum $Letter {}
                impl generics::Sealed for $Letter {}
            )+
        }
    }
}

selector!(A, B, C, D, E);
