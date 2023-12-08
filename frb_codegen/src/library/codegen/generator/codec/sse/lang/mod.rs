use crate::codegen::ir::ty::IrType;

pub(crate) mod dart;
pub(crate) mod rust;

pub(crate) trait Lang {
    fn call_encode(&self, var_ty: &IrType, var_name: &str) -> String;

    fn call_decode(&self, var_ty: &IrType) -> String;

    fn null(&self) -> String;
}
