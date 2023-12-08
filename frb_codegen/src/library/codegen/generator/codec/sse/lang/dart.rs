use crate::codegen::generator::codec::sse::lang::LangTrait;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) struct DartLang;

impl LangTrait for DartLang {
    fn call_encode(&self, var_ty: &IrType, var_name: &str) -> String {
        format!(
            "_sse_encode_{}(serializer, {})",
            var_ty.safe_ident(),
            var_name
        )
    }

    fn call_decode(&self, var_ty: &IrType) -> String {
        format!("_sse_decode_{}(serializer)", var_ty.safe_ident(),)
    }

    fn call_constructor(&self, class_name: &str, field_names: &[String]) -> String {
        format!(
            "{class_name}({})",
            field_names.iter().map(|x| format!("{x}: {x}")).join(", ")
        )
    }

    fn throw_unimplemented(&self) -> String {
        "throw UnimplementedError('')".into()
    }

    fn for_loop(&self, lhs: &str, rhs: &str, body: &str) -> String {
        format!("for (final {lhs} in {rhs}) {{ {body} }}")
    }

    fn null(&self) -> &'static str {
        "null"
    }

    fn var_decl(&self) -> &'static str {
        "var"
    }
}
