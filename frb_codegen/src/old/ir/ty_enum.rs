use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeTrait for IrTypeEnumRef {
    fn dart_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }
}

impl IrEnum {
    pub fn is_struct(&self) -> bool {
        self.is_struct
    }
}
