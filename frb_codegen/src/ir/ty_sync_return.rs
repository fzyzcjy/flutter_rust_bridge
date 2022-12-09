use crate::{ir::*, target::Target};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrTypeSyncReturn(Box<IrType>);

impl IrTypeSyncReturn {
    pub fn new(ir: IrType) -> Self {
        Self(Box::new(ir))
    }

    pub fn into_inner(self) -> IrType {
        *self.0
    }
}

impl IrTypeTrait for IrTypeSyncReturn {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.0.visit_children_types(f, ir_file)
    }

    fn safe_ident(&self) -> String {
        self.0.safe_ident()
    }

    fn dart_api_type(&self) -> String {
        self.0.dart_api_type()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        self.0.dart_wire_type(target)
    }

    fn rust_api_type(&self) -> String {
        self.0.rust_api_type()
    }

    fn rust_wire_type(&self, target: Target) -> String {
        self.0.rust_wire_type(target)
    }
}
