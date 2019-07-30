#![no_std]

extern crate embedded_hal as hal;
extern crate nb;

#[cfg(any(feature = "sams70j19"))]  pub use atsams70j19  as target_device;
#[cfg(any(feature = "sams70j19b"))] pub use atsams70j19b as target_device;
#[cfg(any(feature = "sams70j20"))]  pub use atsams70j20  as target_device;
#[cfg(any(feature = "sams70j20b"))] pub use atsams70j20b as target_device;
#[cfg(any(feature = "sams70j21"))]  pub use atsams70j21  as target_device;
#[cfg(any(feature = "sams70j21b"))] pub use atsams70j21b as target_device;
#[cfg(any(feature = "sams70n19"))]  pub use atsams70n19  as target_device;
#[cfg(any(feature = "sams70n19b"))] pub use atsams70n19b as target_device;
#[cfg(any(feature = "sams70n20"))]  pub use atsams70n20  as target_device;
#[cfg(any(feature = "sams70n20b"))] pub use atsams70n20b as target_device;
#[cfg(any(feature = "sams70n21 "))] pub use atsams70n21  as target_device;
#[cfg(any(feature = "sams70n21b"))] pub use atsams70n21b as target_device;
#[cfg(any(feature = "sams70q19"))]  pub use atsams70q19  as target_device;
#[cfg(any(feature = "sams70q19b"))] pub use atsams70q19b as target_device;
#[cfg(any(feature = "sams70q20"))]  pub use atsams70q20  as target_device;
#[cfg(any(feature = "sams70q20b"))] pub use atsams70q20b as target_device;
#[cfg(any(feature = "sams70q21"))]  pub use atsams70q21  as target_device;
#[cfg(any(feature = "sams70q21b"))] pub use atsams70q21b as target_device;

pub mod serial;