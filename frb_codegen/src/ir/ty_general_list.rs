use crate::ir::*;
use crate::target::Target;

crate::ir! {
#[no_serde]
pub struct IrTypeGeneralList {
    pub inner: Box<IrType>,
}
}

crate::derive_serde_inner_as_newtype!(IrTypeGeneralList);

impl IrTypeTrait for IrTypeGeneralList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.inner.visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        format!("list_{}", self.inner.safe_ident())
    }

    fn dart_api_type(&self) -> String {
        format!("List<{}>", self.inner.dart_api_type())
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            format!("ffi.Pointer<wire_{}>", self.safe_ident())
        }
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.inner.rust_api_type())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        !target.is_wasm()
    }
    fn intodart_type(&self, ir_file: &IrFile) -> String {
        format!("Vec<{}>", self.inner.intodart_type(ir_file))
    }
}
