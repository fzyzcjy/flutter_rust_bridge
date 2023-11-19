use crate::ir::*;
use crate::target::Target;

crate::derive_serde_inner_as_newtype!(IrTypeOptionalList);

impl IrTypeTrait for IrTypeOptionalList {
    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Wasm => "List<dynamic>".into(),
            Target::Io => format!("ffi.Pointer<wire_{}>", self.safe_ident()),
            Target::Common => unreachable!(),
        }
    }
}
