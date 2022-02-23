#![no_std]

extern crate embedded_hal as hal;
extern crate nb;

#[cfg(not(feature = "device-selected"))]
compile_error!(
    "The HAL is built for a specific target device selected using a feature, but no such a feature was selected."
);

#[cfg(feature = "same70j19")]
pub use atsame70j19 as target_device;
#[cfg(feature = "same70j19b")]
pub use atsame70j19b as target_device;
#[cfg(feature = "same70j20")]
pub use atsame70j20 as target_device;
#[cfg(feature = "same70j20b")]
pub use atsame70j20b as target_device;
#[cfg(feature = "same70j21")]
pub use atsame70j21 as target_device;
#[cfg(feature = "same70j21b")]
pub use atsame70j21b as target_device;
#[cfg(feature = "same70n19")]
pub use atsame70n19 as target_device;
#[cfg(feature = "same70n19b")]
pub use atsame70n19b as target_device;
#[cfg(feature = "same70n20")]
pub use atsame70n20 as target_device;
#[cfg(feature = "same70n20b")]
pub use atsame70n20b as target_device;
#[cfg(feature = "same70n21")]
pub use atsame70n21 as target_device;
#[cfg(feature = "same70n21b")]
pub use atsame70n21b as target_device;
#[cfg(feature = "same70q19")]
pub use atsame70q19 as target_device;
#[cfg(feature = "same70q19b")]
pub use atsame70q19b as target_device;
#[cfg(feature = "same70q20")]
pub use atsame70q20 as target_device;
#[cfg(feature = "same70q20b")]
pub use atsame70q20b as target_device;
#[cfg(feature = "same70q21")]
pub use atsame70q21 as target_device;
#[cfg(feature = "same70q21b")]
pub use atsame70q21b as target_device;
#[cfg(feature = "sams70j19")] pub use atsams70j19 as target_device;
#[cfg(feature = "sams70j19b")] pub use atsams70j19b as target_device;
#[cfg(feature = "sams70j20")] pub use atsams70j20 as target_device;
#[cfg(feature = "sams70j20b")] pub use atsams70j20b as target_device;
#[cfg(feature = "sams70j21")] pub use atsams70j21 as target_device;
#[cfg(feature = "sams70j21b")] pub use atsams70j21b as target_device;
#[cfg(feature = "sams70n19")] pub use atsams70n19 as target_device;
#[cfg(feature = "sams70n19b")] pub use atsams70n19b as target_device;
#[cfg(feature = "sams70n20")] pub use atsams70n20 as target_device;
#[cfg(feature = "sams70n20b")] pub use atsams70n20b as target_device;
#[cfg(feature = "sams70n21")] pub use atsams70n21 as target_device;
#[cfg(feature = "sams70n21b")] pub use atsams70n21b as target_device;
#[cfg(feature = "sams70q19")] pub use atsams70q19 as target_device;
#[cfg(feature = "sams70q19b")] pub use atsams70q19b as target_device;
#[cfg(feature = "sams70q20")] pub use atsams70q20 as target_device;
#[cfg(feature = "sams70q20b")] pub use atsams70q20b as target_device;
#[cfg(feature = "sams70q21")] pub use atsams70q21 as target_device;
#[cfg(feature = "sams70q21b")] pub use atsams70q21b as target_device;
#[cfg(feature = "samv71j19")]
pub use atsamv71j19 as target_device;
#[cfg(feature = "samv71j19b")]
pub use atsamv71j19b as target_device;
#[cfg(feature = "samv71j20")]
pub use atsamv71j20 as target_device;
#[cfg(feature = "samv71j20b")]
pub use atsamv71j20b as target_device;
#[cfg(feature = "samv71j21")]
pub use atsamv71j21 as target_device;
#[cfg(feature = "samv71j21b")]
pub use atsamv71j21b as target_device;
#[cfg(feature = "samv71n19")]
pub use atsamv71n19 as target_device;
#[cfg(feature = "samv71n19b")]
pub use atsamv71n19b as target_device;
#[cfg(feature = "samv71n20")]
pub use atsamv71n20 as target_device;
#[cfg(feature = "samv71n20b")]
pub use atsamv71n20b as target_device;
#[cfg(feature = "samv71n21")]
pub use atsamv71n21 as target_device;
#[cfg(feature = "samv71n21b")]
pub use atsamv71n21b as target_device;
#[cfg(feature = "samv71q19")]
pub use atsamv71q19 as target_device;
#[cfg(feature = "samv71q19b")]
pub use atsamv71q19b as target_device;
#[cfg(feature = "samv71q20")]
pub use atsamv71q20 as target_device;
#[cfg(feature = "samv71q20b")]
pub use atsamv71q20b as target_device;
#[cfg(feature = "samv71q21")]
pub use atsamv71q21 as target_device;
#[cfg(feature = "samv71q21b")]
pub use atsamv71q21b as target_device;

#[cfg(feature = "device-selected")]
pub mod serial;
