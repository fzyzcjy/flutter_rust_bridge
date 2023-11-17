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
    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
    }
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        format!("Vec<Option<{}>>", self.inner.intodart_type(ir_pack))
    }
}
