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

    if !feat("__device-selected")
        || ["__e70", "__s70", "__v70", "__v71"]
            .iter()
            .all(|&f| !feat(f))
    {
        return Err(
            "The HAL is built for a specific target device selected using a feature, but no such a feature was selected."
        );
    }

    Ok(())
}
