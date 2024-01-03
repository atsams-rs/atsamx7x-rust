/*!
ATSAMx7x HAL - Hardware Abstraction Layer for the Microchip SAM E70/S70/V70/V71 (rev. B) MCUs
---

This crate is a non-exhaustive abstraction layer of the peripherals
available on the Microchip SAM E70/S70/V70/V71 MCUs. Only the
B-revision is supported.

Where able, hardware state is tracked by the type system and does not incur run-time overhead.

Examples for most implemented peripherals [can be found in the git repository, under `boards/`](https://git.grepit.se/embedded-rust/atsamx7x-hal/-/tree/master/boards).

# Getting Started

After system start, the device's wathdog is active, and will trigger a
system reset after about ~15 seconds. Additionally, before any work
can be done, the clock hierarchy must be configured, because it is
upstream of all other peripherals. Refer to [`clocks`].

[`clock`]: crate::clocks

# References

Any and all references to figures, pages and sections in this crate (both in documentation and source) refer to the complete datasheet of the SAM E70/S70/V70/V71 family of MCUs.
The datasheet (DS60001527F) is available via [Microchip](https://ww1.microchip.com/downloads/aemDocuments/documents/MCU32/ProductDocuments/DataSheets/SAM-E70-S70-V70-V71-Family-Data-Sheet-DS60001527.pdf) (fetched 2022-08-04).
*/

#![cfg_attr(not(test), no_std)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::bare_urls)]

pub use embedded_hal as ehal;
pub use fugit;
pub use nb;

#[cfg(feature = "same70j19")]
pub use atsame70j19 as pac;
#[cfg(feature = "same70j19b")]
pub use atsame70j19b as pac;
#[cfg(feature = "same70j20")]
pub use atsame70j20 as pac;
#[cfg(feature = "same70j20b")]
pub use atsame70j20b as pac;
#[cfg(feature = "same70j21")]
pub use atsame70j21 as pac;
#[cfg(feature = "same70j21b")]
pub use atsame70j21b as pac;
#[cfg(feature = "same70n19")]
pub use atsame70n19 as pac;
#[cfg(feature = "same70n19b")]
pub use atsame70n19b as pac;
#[cfg(feature = "same70n20")]
pub use atsame70n20 as pac;
#[cfg(feature = "same70n20b")]
pub use atsame70n20b as pac;
#[cfg(feature = "same70n21")]
pub use atsame70n21 as pac;
#[cfg(feature = "same70n21b")]
pub use atsame70n21b as pac;
#[cfg(feature = "same70q19")]
pub use atsame70q19 as pac;
#[cfg(feature = "same70q19b")]
pub use atsame70q19b as pac;
#[cfg(feature = "same70q20")]
pub use atsame70q20 as pac;
#[cfg(feature = "same70q20b")]
pub use atsame70q20b as pac;
#[cfg(feature = "same70q21")]
pub use atsame70q21 as pac;
#[cfg(feature = "same70q21b")]
pub use atsame70q21b as pac;
#[cfg(feature = "sams70j19")]
pub use atsams70j19 as pac;
#[cfg(feature = "sams70j19b")]
pub use atsams70j19b as pac;
#[cfg(feature = "sams70j20")]
pub use atsams70j20 as pac;
#[cfg(feature = "sams70j20b")]
pub use atsams70j20b as pac;
#[cfg(feature = "sams70j21")]
pub use atsams70j21 as pac;
#[cfg(feature = "sams70j21b")]
pub use atsams70j21b as pac;
#[cfg(feature = "sams70n19")]
pub use atsams70n19 as pac;
#[cfg(feature = "sams70n19b")]
pub use atsams70n19b as pac;
#[cfg(feature = "sams70n20")]
pub use atsams70n20 as pac;
#[cfg(feature = "sams70n20b")]
pub use atsams70n20b as pac;
#[cfg(feature = "sams70n21")]
pub use atsams70n21 as pac;
#[cfg(feature = "sams70n21b")]
pub use atsams70n21b as pac;
#[cfg(feature = "sams70q19")]
pub use atsams70q19 as pac;
#[cfg(feature = "sams70q19b")]
pub use atsams70q19b as pac;
#[cfg(feature = "sams70q20")]
pub use atsams70q20 as pac;
#[cfg(feature = "sams70q20b")]
pub use atsams70q20b as pac;
#[cfg(feature = "sams70q21")]
pub use atsams70q21 as pac;
#[cfg(feature = "sams70q21b")]
pub use atsams70q21b as pac;
#[cfg(feature = "samv70j19")]
pub use atsamv70j19 as pac;
#[cfg(feature = "samv70j19b")]
pub use atsamv70j19b as pac;
#[cfg(feature = "samv70j20")]
pub use atsamv70j20 as pac;
#[cfg(feature = "samv70j20b")]
pub use atsamv70j20b as pac;
#[cfg(feature = "samv70n19")]
pub use atsamv70n19 as pac;
#[cfg(feature = "samv70n19b")]
pub use atsamv70n19b as pac;
#[cfg(feature = "samv70n20")]
pub use atsamv70n20 as pac;
#[cfg(feature = "samv70n20b")]
pub use atsamv70n20b as pac;
#[cfg(feature = "samv70q19")]
pub use atsamv70q19 as pac;
#[cfg(feature = "samv70q19b")]
pub use atsamv70q19b as pac;
#[cfg(feature = "samv70q20")]
pub use atsamv70q20 as pac;
#[cfg(feature = "samv70q20b")]
pub use atsamv70q20b as pac;
#[cfg(feature = "samv71j19")]
pub use atsamv71j19 as pac;
#[cfg(feature = "samv71j19b")]
pub use atsamv71j19b as pac;
#[cfg(feature = "samv71j20")]
pub use atsamv71j20 as pac;
#[cfg(feature = "samv71j20b")]
pub use atsamv71j20b as pac;
#[cfg(feature = "samv71j21")]
pub use atsamv71j21 as pac;
#[cfg(feature = "samv71j21b")]
pub use atsamv71j21b as pac;
#[cfg(feature = "samv71n19")]
pub use atsamv71n19 as pac;
#[cfg(feature = "samv71n19b")]
pub use atsamv71n19b as pac;
#[cfg(feature = "samv71n20")]
pub use atsamv71n20 as pac;
#[cfg(feature = "samv71n20b")]
pub use atsamv71n20b as pac;
#[cfg(feature = "samv71n21")]
pub use atsamv71n21 as pac;
#[cfg(feature = "samv71n21b")]
pub use atsamv71n21b as pac;
#[cfg(feature = "samv71q19")]
pub use atsamv71q19 as pac;
#[cfg(feature = "samv71q19b")]
pub use atsamv71q19b as pac;
#[cfg(feature = "samv71q20")]
pub use atsamv71q20 as pac;
#[cfg(feature = "samv71q20b")]
pub use atsamv71q20b as pac;
#[cfg(feature = "samv71q21")]
pub use atsamv71q21 as pac;
#[cfg(feature = "samv71q21b")]
pub use atsamv71q21b as pac;

#[cfg(feature = "__device-selected")]
pub mod afec;
#[cfg(all(feature = "__device-selected", feature = "can"))]
pub mod can;
#[cfg(feature = "__device-selected")]
pub mod clocks;
#[cfg(feature = "__device-selected")]
pub mod efc;
#[cfg(feature = "__device-selected")]
pub mod generics;
#[cfg(feature = "__device-selected")]
pub mod pio;
#[cfg(feature = "__device-selected")]
pub mod pwm;
#[cfg(feature = "__device-selected")]
pub mod rtt;
#[cfg(feature = "__device-selected")]
pub mod serial;
#[cfg(feature = "__device-selected")]
pub mod tc;
#[cfg(feature = "__device-selected")]
pub mod usb;
#[cfg(feature = "__device-selected")]
pub mod watchdog;
