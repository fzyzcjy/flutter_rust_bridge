use crate::ir::*;

#[derive(Debug, Clone)]
pub struct IrTypeGeneralList {
    pub inner: Box<IrType>,
}

impl ApiTypeTrait for IrTypeGeneralList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, api_file: &IrFile) {
        self.inner.visit_types(f, api_file);
    }

    fn safe_ident(&self) -> String {
        format!("list_{}", self.inner.safe_ident())
    }

    fn dart_api_type(&self) -> String {
        format!("List<{}>", self.inner.dart_api_type())
    }

    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<wire_{}>", self.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.inner.rust_api_type())
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}
