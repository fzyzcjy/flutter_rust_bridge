use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypePrimitiveList {
    pub primitive: IrTypePrimitive,
}
}

impl IrTypeTrait for IrTypePrimitiveList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, _ir_file: &IrFile) {
        f(&IrType::Primitive(self.primitive.clone()));
    }

    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }
}
