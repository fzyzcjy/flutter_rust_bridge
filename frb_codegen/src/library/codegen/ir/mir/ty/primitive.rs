use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use convert_case::{Case, Casing};

crate::mir! {
#[derive(strum_macros::Display)]
pub enum MirTypePrimitive {
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
    // For i128/u128, see MirDelegate
}
}

impl MirTypeTrait for MirTypePrimitive {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        _f: &mut F,
        _mir_context: &impl MirContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        self.to_string().to_case(Case::Snake)
    }

    fn rust_api_type(&self) -> String {
        match self {
            MirTypePrimitive::U8 => "u8",
            MirTypePrimitive::I8 => "i8",
            MirTypePrimitive::U16 => "u16",
            MirTypePrimitive::I16 => "i16",
            MirTypePrimitive::U32 => "u32",
            MirTypePrimitive::I32 => "i32",
            MirTypePrimitive::U64 => "u64",
            MirTypePrimitive::Unit => "()",
            MirTypePrimitive::Usize => "usize",
            MirTypePrimitive::Isize => "isize",
            MirTypePrimitive::I64 => "i64",
            MirTypePrimitive::F32 => "f32",
            MirTypePrimitive::F64 => "f64",
            MirTypePrimitive::Bool => "bool",
        }
        .to_string()
    }

    fn as_primitive(&self) -> Option<&MirTypePrimitive> {
        Some(self)
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        true
    }
}
