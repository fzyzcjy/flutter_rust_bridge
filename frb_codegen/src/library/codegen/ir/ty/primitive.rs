use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use convert_case::{Case, Casing};

crate::ir! {
#[derive(strum_macros::Display)]
pub enum IrTypePrimitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
    Unit,
    Usize,
    Isize,
}
}

impl IrTypeTrait for IrTypePrimitive {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {}

    fn safe_ident(&self) -> String {
        self.to_string().to_case(Case::Snake)
    }

    fn rust_api_type(&self) -> String {
        self.rust_wire_type(Target::Io)
    }
}
