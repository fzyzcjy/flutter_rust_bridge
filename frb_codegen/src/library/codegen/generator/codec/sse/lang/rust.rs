use crate::codegen::generator::codec::sse::lang::LangTrait;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

pub(crate) struct RustLang;

impl LangTrait for RustLang {
    fn call_encode(&self, var_ty: &IrType, var_name: &str) -> String {
        format!(
            "_sse_encode_{}({}, serializer)",
            var_ty.safe_ident(),
            var_name
        )
    }

    fn call_decode(&self, var_ty: &IrType) -> String {
        format!("_sse_decode_{}(serializer)", var_ty.safe_ident(),)
    }

    fn call_constructor(&self, class_name: &str, field_names: &[String]) -> String {
        format!(
            "{class_name} {{ {} }}",
            field_names.iter().map(|x| format!("{x}: {x}")).join(", ")
        )
    }

    fn throw_unimplemented(&self) -> String {
        "unimplemented!()".into()
    }

    fn for_loop(&self, lhs: &str, rhs: &str, body: &str) -> String {
        format!("for {lhs} in {rhs} {{ {body} }}")
    }

    fn null(&self) -> &'static str {
        "None"
    }

    fn var_decl(&self) -> &'static str {
        "let mut"
    }
}
