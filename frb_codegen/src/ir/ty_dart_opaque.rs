use crate::{ir::*, target::Target};

#[derive(Debug, Clone)]
pub struct IrTypeDartOpaque;

impl IrTypeTrait for IrTypeDartOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {
        // do nothing.
    }

    fn safe_ident(&self) -> String {
        "DartObject".to_owned()
    }

    fn dart_api_type(&self) -> String {
        "Object".to_owned()
    }

    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if target.is_wasm() {
            "int"
        } else {
            "ffi.Pointer<wire_DartOpaque>"
        }
        .to_owned()
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        !target.is_wasm()
    }

    fn rust_api_type(&self) -> String {
        "DartOpaque".to_owned()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "*mut JsValue".into()
        } else {
            "wire_DartOpaque".to_owned()
        }
    }
}
