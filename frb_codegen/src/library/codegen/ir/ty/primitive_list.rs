use convert_case::Case;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

crate::ir! {
pub struct IrTypePrimitiveList {
    pub primitive: IrTypePrimitive,
}
}

impl IrTypeTrait for IrTypePrimitiveList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        IrType::Primitive(self.primitive.clone()).visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        format!("list_prim_{}", self.inner.safe_ident())
    }
}
