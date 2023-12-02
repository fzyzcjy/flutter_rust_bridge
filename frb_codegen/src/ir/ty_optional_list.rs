use crate::ir::*;
use crate::target::Target;

crate::ir! {
    #[no_serde]
    pub struct IrTypeOptionalList {
        pub inner: Box<IrType>,
    }
}

crate::derive_serde_inner_as_newtype!(IrTypeOptionalList);

impl IrTypeTrait for IrTypeOptionalList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.inner.visit_self_types_recursively(f, ir_file)
    }
    fn safe_ident(&self) -> String {
        format!("list_opt_{}", self.inner.safe_ident())
    }
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
    fn intodart_type(&self, ir_file: &IrFile) -> String {
        format!("Vec<Option<{}>>", self.inner.intodart_type(ir_file))
    }
}
