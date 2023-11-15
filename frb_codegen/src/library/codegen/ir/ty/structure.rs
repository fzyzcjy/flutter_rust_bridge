// Name "structure" not "struct", since the latter is a keyword

use convert_case::{Case, Casing};
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeStructRef {
    pub name: String,
    pub freezed: bool,
    pub empty: bool,
    pub is_exception: bool,
}
}

impl IrTypeTrait for IrTypeStructRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        for field in &self.get(ir_file).fields {
            field.ty.visit_types(f, ir_file);
        }
    }

    fn safe_ident(&self) -> String {
        self.name.to_case(Case::Snake)
    }
}
