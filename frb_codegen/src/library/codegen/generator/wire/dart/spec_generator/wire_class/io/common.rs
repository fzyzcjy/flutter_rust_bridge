pub(super) fn generate_wire_class_header(wire_class_name: &str) -> String {
    format!(
        "class {wire_class_name} implements BaseWire {{

            factory {wire_class_name}.fromExternalLibrary(ExternalLibrary lib) =>
              {wire_class_name}(lib.ffiDynamicLibrary);
        "
    )
}
