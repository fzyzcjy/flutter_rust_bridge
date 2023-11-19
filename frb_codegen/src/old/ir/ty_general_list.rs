use crate::ir::*;
use crate::target::Target;

crate::derive_serde_inner_as_newtype!(IrTypeGeneralList);

impl IrTypeTrait for IrTypeGeneralList {
    fn dart_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            format!("ffi.Pointer<wire_{}>", self.safe_ident())
        }
    }
}
