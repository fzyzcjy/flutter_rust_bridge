use crate::codegen::generator::codec::sse::lang::LangTrait;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::{multizip, Itertools};

#[derive(Clone, Copy, Debug)]
pub(crate) struct DartLang;

impl LangTrait for DartLang {
    fn call_encode(&self, var_ty: &IrType, var_name: &str) -> String {
        format!(
            "sse_encode_{}({}, serializer)",
            var_ty.safe_ident(),
            var_name
        )
    }

    fn call_decode(&self, var_ty: &IrType) -> String {
        format!("sse_decode_{}(deserializer)", var_ty.safe_ident(),)
    }

    fn call_constructor(
        &self,
        class_name: &str,
        ctor_name: Option<&str>,
        field_names: &[String],
        var_names: &[String],
        keyword_args: bool,
    ) -> String {
        let dotted_ctor_name = ctor_name.map_or_else(|| "".to_owned(), |x| format!(".{x}"));
        format!(
            "{class_name}{dotted_ctor_name}({})",
            multizip((field_names, var_names))
                .map(|(x, y)| if keyword_args {
                    format!("{x}: {y}")
                } else {
                    y.to_string()
                })
                .join(", ")
        )
    }

    fn throw_unimplemented(&self, message: &str) -> String {
        format!("throw UnimplementedError('{message}')")
    }

    fn throw_unreachable(&self, message: &str) -> String {
        format!("throw UnimplementedError('Unreachable ({message}(')")
    }

    fn for_loop(&self, lhs: &str, rhs: &str, body: &str) -> String {
        format!("for (final {lhs} in {rhs}) {{ {body} }}")
    }

    fn for_range_loop(&self, var: &str, limit: &str, body: &str) -> String {
        format!("for (var {var} = 0; {var} < {limit}; ++{var}) {{ {body} }}")
    }

    fn switch_expr(
        &self,
        value: &str,
        variants: &[(String, String)],
        fallback: Option<String>,
    ) -> String {
        let body = (variants.iter())
            .map(|(lhs, rhs)| format!("case {lhs}: {rhs}"))
            .join("");
        let fallback = fallback
            .map(|expr| format!("default: {expr}"))
            .unwrap_or_default();
        format!("switch ({value}) {{ {body} {fallback} }}")
    }

    fn var_decl(&self) -> &'static str {
        "var"
    }
}
