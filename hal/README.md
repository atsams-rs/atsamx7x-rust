ATSAMx7x Rust HAL and PACs
---

This crate provides a type-safe API for working with Microchip SAM S70/E70/V70/V71-based devices.

## Usage

For example, if you are using an ATSAMV71Q21B. Add the following to your `Cargo.toml`:
```toml
[dependencies]
atsamx7x-hal = { version = "0.4.1", features = [ "samv71q21b-rt", "unproven" ] }
```

The `-rt` suffix adds the [`cortex-m-rt`](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/) run-time. `unproven` enables unproven [`embedded-hal`](https://docs.rs/embedded-hal/0.2.7/embedded_hal/) features.

License
---
All source code (including code snippets) is licensed under either of

Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
MIT license ([LICENSE-MIT](./LICENSE-MIT) or https://opensource.org/licenses/MIT)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
