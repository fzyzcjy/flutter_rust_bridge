use crate::ir::*;
use crate::target::Target;

crate::derive_serde_inner_as_newtype!(IrTypeOptionalList);

impl IrTypeTrait for IrTypeOptionalList {
    fn rust_wire_type(&self, target: Target) -> String {
        match target {
            Target::Wasm => "JsValue".into(),
            Target::Io => format!("wire_{}", self.safe_ident()),
            Target::Common => unreachable!(),
        }
    }
    fn rust_api_type(&self) -> String {
        format!("Vec<Option<{}>>", self.inner.rust_api_type())
    }
    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Wasm => "List<dynamic>".into(),
            Target::Io => format!("ffi.Pointer<wire_{}>", self.safe_ident()),
            Target::Common => unreachable!(),
        }
    }
    fn dart_api_type(&self) -> String {
        format!("List<{}?>", self.inner.dart_api_type())
    }
    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        !target.is_wasm()
    }
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        format!("Vec<Option<{}>>", self.inner.intodart_type(ir_pack))
    }
}
