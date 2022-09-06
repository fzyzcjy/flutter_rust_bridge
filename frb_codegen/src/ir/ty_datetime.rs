use crate::ir::*;

#[derive(Debug, Clone)]
pub struct IrTypeDateTime(i64);

impl IrTypeTrait for IrTypeDateTime {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {}

    fn safe_ident(&self) -> String {
        self.rust_api_type()
    }

    fn dart_api_type(&self) -> String {
        "DateTime".to_string()
    }

    fn dart_wire_type(&self) -> String {
        "int".to_string()
    }

    fn rust_api_type(&self) -> String {
        "chrono::DateTime<chrono::Utc>".to_string()
    }

    fn rust_wire_type(&self) -> String {
        "i64".to_string()
    }
}

impl IrTypeDateTime {
    /// Representations of primitives within Dart's pointers, e.g. `ffi.Pointer<ffi.Uint8>`.
    /// This is enforced on Dart's side, and should be used instead of `dart_wire_type`
    /// whenever primitives are put behind a pointer.
    pub fn dart_native_type(&self) -> &'static str {
        "ffi.Int64"
    }
    pub fn try_from_rust_str(s: &str) -> Option<Self> {
        match s {
            "i64" => s.parse().map(|x| IrTypeDateTime(x)).ok(),
            _ => None,
        }
    }
}
