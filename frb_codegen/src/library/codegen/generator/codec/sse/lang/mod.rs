use crate::codegen::ir::ty::IrType;

pub(crate) mod dart;
pub(crate) mod rust;

pub(crate) trait Lang {
    fn call_encode(&self, ty: &IrType) -> String;

    fn call_decode(&self, ty: &IrType) -> String;
}
