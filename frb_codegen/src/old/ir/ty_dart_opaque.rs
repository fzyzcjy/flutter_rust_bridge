use crate::ir::*;

impl IrTypeTrait for IrTypeDartOpaque {
    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if target == Target::Wasm {
            "Object"
        } else {
            "wire_DartOpaque"
        }
        .to_owned()
    }

    fn rust_api_type(&self) -> String {
        "DartOpaque".to_owned()
    }
}
