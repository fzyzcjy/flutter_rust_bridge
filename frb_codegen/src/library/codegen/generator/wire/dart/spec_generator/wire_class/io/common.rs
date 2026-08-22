pub(super) fn generate_wire_class_header(wire_class_name: &str) -> String {
    format!(
        "class {wire_class_name} implements BaseWire {{

            factory {wire_class_name}.fromExternalLibrary(ExternalLibrary lib) =>
              {wire_class_name}(lib.ffiDynamicLibrary);
        "
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates an external-library factory for the requested wire class.
    #[test]
    fn wire_class_header_uses_the_given_class_name_consistently() {
        assert_eq!(
            generate_wire_class_header("RustLibWire"),
            "class RustLibWire implements BaseWire {\n\n            factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>\n              RustLibWire(lib.ffiDynamicLibrary);\n        "
        );
    }
}
