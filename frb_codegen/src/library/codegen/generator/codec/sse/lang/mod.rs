use crate::codegen::generator::codec::sse::lang::dart::DartLang;
use crate::codegen::generator::codec::sse::lang::rust::RustLang;
use crate::codegen::ir::ty::IrType;
use enum_dispatch::enum_dispatch;

pub(crate) mod dart;
pub(crate) mod rust;

#[enum_dispatch]
pub(crate) enum Lang {
    DartLang,
    RustLang,
}

#[enum_dispatch(Lang)]
pub(crate) trait LangTrait {
    fn call_encode(&self, var_ty: &IrType, var_name: &str) -> String;

    fn call_decode(&self, var_ty: &IrType) -> String;

    fn call_constructor(&self, class_name: &str, field_names: &[String]) -> String;

    fn for_loop(&self, lhs: &str, rhs: &str, body: &str) -> String;

    fn null(&self) -> String;

    fn var_decl(&self) -> String;
}
