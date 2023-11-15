// Name "enumeration" not "enum", since the latter is a keyword

use convert_case::{Case, Casing};
use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::codegen::ir::ty::structure::IrStruct;

crate::ir! {
pub struct IrTypeEnumRef {
    pub name: String,
}

pub struct IrEnum {
    pub name: String,
    pub wrapper_name: Option<String>,
    pub path: Vec<String>,
    pub comments: Vec<IrComment>,
    pub is_exception: bool,
    variants: Vec<IrVariant>,
    is_struct: bool,
}

pub struct IrVariant {
    pub name: IrIdent,
    pub wrapper_name: IrIdent,
    pub comments: Vec<IrComment>,
    pub kind: IrVariantKind,
}

pub enum IrVariantKind {
    Value,
    Struct(IrStruct),
}
}

impl IrTypeTrait for IrTypeEnumRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        let enu = self.get(ir_pack);
        for variant in enu.variants() {
            if let IrVariantKind::Struct(st) = &variant.kind {
                st.fields
                    .iter()
                    .for_each(|field| field.ty.visit_types(f, ir_pack));
            }
        }
    }

    fn safe_ident(&self) -> String {
        self.name.to_case(Case::Snake)
    }
}
