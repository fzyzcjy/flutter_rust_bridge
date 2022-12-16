use delegate::delegate;

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
    delegate! {
        to self.0 {
            fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile);
            fn safe_ident(&self) -> String;
            fn dart_api_type(&self) -> String;
            fn dart_wire_type(&self, target: Target) -> String;
            fn rust_api_type(&self) -> String;
            fn rust_wire_type(&self, target: Target) -> String;
        }
    }
}
