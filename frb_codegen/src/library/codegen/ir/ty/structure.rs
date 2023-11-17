// Name "structure" not "struct", since the latter is a keyword

use crate::codegen::ir::annotation::IrDartAnnotation;
use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use convert_case::{Case, Casing};

crate::ir! {
pub struct IrTypeStructRef {
    pub ident: IrStructIdent,
    pub is_exception: bool,
}

pub struct IrStructIdent(pub String);

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

impl IrTypeStructRef {
    pub fn get<'a>(&self, f: &'a IrPack) -> &'a IrStruct {
        &f.struct_pool[&self.ident]
    }
}

impl IrTypeTrait for IrTypeStructRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        for field in &self.get(ir_pack).fields {
            field.ty.visit_types(f, ir_pack);
        }
    }

    fn safe_ident(&self) -> String {
        self.ident.0.to_case(Case::Snake)
    }

    fn rust_api_type(&self) -> String {
        self.ident.0.to_string()
    }
}

impl IrStruct {
    pub fn using_freezed(&self) -> bool {
        self.dart_metadata.iter().any(|it| it.content == "freezed")
    }
}
