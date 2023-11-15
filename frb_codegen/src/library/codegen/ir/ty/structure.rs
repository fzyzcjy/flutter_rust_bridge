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

pub struct IrStruct {
    pub name: String,
    pub wrapper_name: Option<String>,
    pub path: Option<Vec<String>>,
    pub fields: Vec<IrField>,
    pub is_fields_named: bool,
    pub dart_metadata: Vec<IrDartAnnotation>,
    pub comments: Vec<IrComment>,
}
}

impl IrTypeTrait for IrTypeStructRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        for field in &self.get(ir_pack).fields {
            field.ty.visit_types(f, ir_pack);
        }
    }

    fn safe_ident(&self) -> String {
        self.name.to_case(Case::Snake)
    }
}
