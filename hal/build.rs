use std::env;

fn main() -> Result<(), &'static str> {
    // Refer to
    // <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts>
    fn feat(f: &str) -> bool {
        env::var(format!(
            "CARGO_FEATURE_{}",
            f.to_ascii_uppercase().replace('-', "_")
        ))
        .is_ok()
    }

    if !feat("device-selected") || ["e70", "s70", "v70", "v71"].iter().all(|&f| !feat(f)) {
        return Err(
            "The HAL is built for a specific target device selected using a feature, but no such a feature was selected."
        );
    }

    Ok(())
}
