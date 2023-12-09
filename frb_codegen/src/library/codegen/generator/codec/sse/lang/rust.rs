use crate::codegen::generator::codec::sse::lang::LangTrait;
use crate::codegen::ir::ty::structure::rust_brackets_pair;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::{multizip, Itertools};
use std::env::var;

#[derive(Clone, Copy, Debug)]
pub(crate) struct RustLang;

impl LangTrait for RustLang {
    fn call_encode(&self, var_ty: &IrType, var_name: &str) -> String {
        format!(
            "<{}>::sse_encode({}, serializer)",
            var_ty.rust_api_type(),
            var_name
        )
    }

    fn call_decode(&self, var_ty: &IrType) -> String {
        format!("<{}>::sse_decode(deserializer)", var_ty.rust_api_type())
    }

    fn call_constructor(
        &self,
        class_name: &str,
        field_names: &[String],
        var_names: &[String],
        keyword_args: bool,
    ) -> String {
        let (left, right) = rust_brackets_pair(keyword_args);
        format!(
            "{class_name}{left}{}{right}",
            multizip((field_names, var_names))
                .map(|(x, y)| if keyword_args && x != y {
                    format!("{x}: {y}")
                } else {
                    format!("{y}")
                })
                .join(", ")
        )
    }

    fn throw_unimplemented(&self) -> String {
        "unimplemented!()".into()
    }

    fn for_loop(&self, lhs: &str, rhs: &str, body: &str) -> String {
        format!("for {lhs} in {rhs} {{ {body} }}")
    }

    fn switch_expr(
        &self,
        value: &str,
        variants: &[(String, String)],
        fallback: Option<String>,
    ) -> String {
        let body = (variants.iter())
            .map(|(lhs, rhs)| format!("{lhs} => {{ {rhs} }}\n"))
            .join("");
        let fallback = fallback
            .map(|expr| format!("_ => {{ {expr} }}"))
            .unwrap_or_default();
        format!(
            "
            match {value} {{
                {body}
                {fallback}
            }}
            "
        )
    }

    fn null(&self) -> &'static str {
        "None"
    }

    fn var_decl(&self) -> &'static str {
        "let mut"
    }
}
