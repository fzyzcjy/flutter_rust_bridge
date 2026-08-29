pub const FLUTTER_RUST_BRIDGE_RUNTIME_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    /// Matches the runtime version constant to the package metadata.
    #[test]
    fn test_runtime_version_matches_package_version() {
        assert_eq!(
            super::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION,
            env!("CARGO_PKG_VERSION")
        );
    }
}
