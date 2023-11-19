use crate::{ir::*, target::Target};

impl IrTypeTrait for IrTypeRecord {
    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm {
            "List<dynamic>".to_string()
        } else {
            self.rust_wire_type(target)
        }
    }
}
