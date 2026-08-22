use crate::codegen::generator::codec::sse::lang::LangTrait;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::{multizip, Itertools};

#[derive(Clone, Copy, Debug)]
pub(crate) struct DartLang;

impl LangTrait for DartLang {
    fn call_encode(&self, var_ty: &MirType, var_name: &str) -> String {
        format!(
            "sse_encode_{}({}, serializer)",
            var_ty.safe_ident(),
            var_name
        )
    }

    fn call_decode(&self, var_ty: &MirType) -> String {
        format!("sse_decode_{}(deserializer)", var_ty.safe_ident(),)
    }

    fn call_constructor(
        &self,
        class_name: &str,
        ctor_postfix: &str,
        field_names: &[String],
        var_names: &[String],
        keyword_args: bool,
    ) -> String {
        format!(
            "{class_name}{ctor_postfix}({})",
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
        format!("throw UnimplementedError('Unreachable ({message})')")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

    /// Emits Dart type wrappers and error expressions for serialized values.
    #[test]
    fn wrappers_and_errors_match_dart_syntax() {
        let lang = DartLang;
        let ty = MirType::Primitive(MirTypePrimitive::I32);

        assert_eq!(
            lang.call_encode(&ty, "value"),
            "sse_encode_i_32(value, serializer)"
        );
        assert_eq!(lang.call_decode(&ty), "sse_decode_i_32(deserializer)");
        assert_eq!(
            lang.throw_unimplemented("missing"),
            "throw UnimplementedError('missing')"
        );
        assert_eq!(
            lang.throw_unreachable("invalid"),
            "throw UnimplementedError('Unreachable (invalid)')"
        );
    }

    /// Formats Dart constructors for positional and named field arguments.
    #[test]
    fn constructor_uses_requested_argument_style() {
        let lang = DartLang;
        let fields = ["first".to_owned(), "second".to_owned()];
        let values = ["one".to_owned(), "two".to_owned()];

        assert_eq!(
            lang.call_constructor("Pair", ".named", &fields, &values, false),
            "Pair.named(one, two)"
        );
        assert_eq!(
            lang.call_constructor("Pair", ".named", &fields, &values, true),
            "Pair.named(first: one, second: two)"
        );
    }

    /// Emits Dart control-flow tokens including an optional switch fallback.
    #[test]
    fn control_flow_tokens_match_dart_syntax() {
        let lang = DartLang;

        assert_eq!(
            lang.for_loop("item", "items", "use(item);"),
            "for (final item in items) { use(item); }"
        );
        assert_eq!(
            lang.for_range_loop("index", "count", "use(index);"),
            "for (var index = 0; index < count; ++index) { use(index); }"
        );
        assert_eq!(
            lang.switch_expr(
                "value",
                &[("0".to_owned(), "zero;".to_owned())],
                Some("fallback;".to_owned()),
            ),
            "switch (value) { case 0: zero; default: fallback; }"
        );
        assert_eq!(
            lang.switch_expr("value", &[("0".to_owned(), "zero;".to_owned())], None),
            "switch (value) { case 0: zero;  }"
        );
        assert_eq!(lang.var_decl(), "var");
    }
}
