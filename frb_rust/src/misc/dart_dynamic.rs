use crate::platform_types::DartAbi;

/// Use this type to represent Dart `dynamic` values
pub type DartDynamic = DartAbi;

#[cfg(test)]
mod tests {
    /// Confirms that dynamic values preserve the platform ABI type.
    #[test]
    fn test_dart_dynamic_is_platform_abi() {
        fn accepts_dynamic(_: super::DartDynamic) {}

        let _: fn(crate::platform_types::DartAbi) = accepts_dynamic;
    }
}
