use crate::ir::IrType::*;
use crate::ir::*;
use crate::target::Target;

crate::derive_serde_inner_as_newtype!(IrTypeOptional);

impl IrTypeTrait for IrTypeOptional {
    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm {
            format!("{}?", self.inner.dart_wire_type(target))
        } else {
            self.inner.dart_wire_type(target)
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm || self.inner.rust_wire_is_pointer(target)
    }
}
