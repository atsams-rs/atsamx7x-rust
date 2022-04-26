use std::env;

fn main() -> Result<(), &'static str> {
    // Refer to
    // <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts>
    fn feat(f: &str) -> bool {
        env::var(format!(
            "CARGO_FEATURE_{}",
            f.to_ascii_uppercase().replace("-", "_")
        ))
        .is_ok()
    }

    if !feat("device-selected") {
        return Err(
            "The HAL is built for a specific target device selected using a feature, but no such a feature was selected."
        );
    }

    if feat("vddio-3v") && feat("vddio-1v") {
        return Err(
            "\"vddio-3v\" and \"vddio-1v\" are mutually exclusive features, try building with one, not both."
            );
    } else if !(feat("vddio-3v") || feat("vddio-1v")) {
        return Err(
            "The HAL is built for a specific voltage level using a feature, but no such feature was selected."
            );
    }

    Ok(())
}
