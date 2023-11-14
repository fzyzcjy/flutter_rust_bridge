use crate::ir::*;

impl IrTypeTrait for IrTypeDartOpaque {
    fn dart_api_type(&self) -> String {
        "Object".to_owned()
    }

    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if target.is_wasm() {
            "Object"
        } else {
            "wire_DartOpaque"
        }
        .to_owned()
    }

    fn rust_api_type(&self) -> String {
        "DartOpaque".to_owned()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if target.is_wasm() {
            "JsValue"
        } else {
            "wire_DartOpaque"
        }
        .to_owned()
    }
}
