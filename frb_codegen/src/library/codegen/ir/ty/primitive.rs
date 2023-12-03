use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
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
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        _f: &mut F,
        _ir_context: &impl IrContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        self.to_string().to_case(Case::Snake)
    }

    fn rust_api_type(&self) -> String {
        match self {
            IrTypePrimitive::U8 => "u8",
            IrTypePrimitive::I8 => "i8",
            IrTypePrimitive::U16 => "u16",
            IrTypePrimitive::I16 => "i16",
            IrTypePrimitive::U32 => "u32",
            IrTypePrimitive::I32 => "i32",
            IrTypePrimitive::U64 => "u64",
            IrTypePrimitive::Unit => "unit",
            IrTypePrimitive::Usize => "usize",
            IrTypePrimitive::Isize => "isize",
            IrTypePrimitive::I64 => "i64",
            IrTypePrimitive::F32 => "f32",
            IrTypePrimitive::F64 => "f64",
            IrTypePrimitive::Bool => "bool",
        }
        .to_string()
    }

    fn as_primitive(&self) -> Option<&IrTypePrimitive> {
        Some(self)
    }
}
