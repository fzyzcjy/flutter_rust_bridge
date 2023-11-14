use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeRecord {
    /// Refers to a virtual struct definition.
    pub inner: IrTypeStructRef,
    pub values: Box<[IrType]>,
}
}

impl IrTypeTrait for IrTypeRecord {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        for ty in self.values.iter() {
            ty.visit_types(f, ir_file)
        }
    }

    fn safe_ident(&self) -> String {
        self.inner.name.clone()
    }
}
