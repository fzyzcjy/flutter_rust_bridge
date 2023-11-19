use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeTrait for IrTypeStructRef {
    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }
}
