use crate::{ir::*, target::Target};

impl IrTypeTrait for IrTypeUnencodable {
    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "Object".to_owned()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.string.clone()
    }
}
