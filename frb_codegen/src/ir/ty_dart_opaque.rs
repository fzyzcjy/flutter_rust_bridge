use crate::{ir::*, target::Target};

#[derive(Debug, Clone)]
pub struct IrTypeDartOpaque;

impl IrTypeTrait for IrTypeDartOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {
        // do nothing.
    }

    fn safe_ident(&self) -> String {
        "".to_owned()
    }

    fn dart_api_type(&self) -> String {
        "Object".to_owned()
    }

    fn dart_wire_type(&self, _target: crate::target::Target) -> String {
        "Object".to_owned()
    }

    fn rust_api_type(&self) -> String {
        "DartOpaque".to_owned()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            "Dart_Handle".to_owned()
        }
    }

}