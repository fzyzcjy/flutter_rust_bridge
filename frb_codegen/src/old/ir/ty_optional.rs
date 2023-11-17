use crate::ir::IrType::*;
use crate::ir::*;
use crate::target::Target;

crate::derive_serde_inner_as_newtype!(IrTypeOptional);

impl IrTypeOptional {
    pub fn is_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if ! boxed.exist_in_real_api && boxed.inner.is_primitive())
    }

    pub fn is_boxed_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if boxed.exist_in_real_api && boxed.inner.is_primitive())
    }
}

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
