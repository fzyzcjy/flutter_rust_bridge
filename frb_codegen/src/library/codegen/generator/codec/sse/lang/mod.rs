use crate::codegen::ir::ty::IrType;

pub(crate) mod dart;
pub(crate) mod rust;

pub(crate) trait Lang {
    fn call_encode(&self, var_ty: &IrType, var_name: &str) -> String;

    fn call_decode(&self, var_ty: &IrType) -> String;

    fn call_constructor(&self, class_name: &str, field_names: &[String]) -> String;

    fn for_loop(&self, lhs: &str, rhs: &str, body: &str) -> String;

    fn null(&self) -> String;

    fn var_decl(&self) -> String;
}
