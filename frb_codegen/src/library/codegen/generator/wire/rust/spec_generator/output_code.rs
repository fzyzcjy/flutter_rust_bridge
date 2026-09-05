use crate::codegen::generator::wire::rust::spec_generator::extern_func::{ExternClass, ExternFunc};
use crate::simple_code_trait_impl;
use itertools::Itertools;
use serde::Serialize;
use std::ops::AddAssign;

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct WireRustOutputCode {
    pub(crate) body: String,
    pub(crate) extern_funcs: Vec<ExternFunc>,
    pub(crate) extern_classes: Vec<ExternClass>,
}

simple_code_trait_impl!(WireRustOutputCode);

impl WireRustOutputCode {
    pub(crate) fn all_code(&self, c_symbol_prefix: &str) -> String {
        format!(
            "{}\n{}\n{}",
            self.body,
            (self.extern_funcs.iter())
                .map(|func| func.generate(c_symbol_prefix))
                .join("\n"),
            (self.extern_classes.iter().map(|cls| cls.generate())).join("\n"),
        )
    }
}

impl AddAssign for WireRustOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.body += &rhs.body;
        self.extern_funcs.extend(rhs.extern_funcs);
        self.extern_classes.extend(rhs.extern_classes);
    }
}

impl From<ExternFunc> for WireRustOutputCode {
    fn from(value: ExternFunc) -> Self {
        vec![value].into()
    }
}

impl From<Vec<ExternFunc>> for WireRustOutputCode {
    fn from(extern_funcs: Vec<ExternFunc>) -> Self {
        Self {
            extern_funcs,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::misc::target::Target;
    use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
        ExternClassMode, ExternFuncParam,
    };
    use itertools::Itertools;

    fn extern_func(name: &str) -> ExternFunc {
        ExternFunc {
            partial_func_name: name.into(),
            params: vec![ExternFuncParam {
                name: "value".into(),
                rust_type: "i32".into(),
                dart_type: "int".into(),
            }],
            return_type: None,
            body: format!("{name}_impl(value)"),
            target: Target::Io,
            needs_ffigen: false,
        }
    }

    /// Concatenates bodies and renders functions and classes in their stable order.
    #[test]
    fn combines_output_code_and_renders_every_section() {
        let mut output = WireRustOutputCode {
            body: "first\n".into(),
            extern_funcs: vec![extern_func("first")],
            extern_classes: vec![],
        };
        output += WireRustOutputCode {
            body: "second".into(),
            extern_funcs: vec![extern_func("second")],
            extern_classes: vec![ExternClass {
                name: "NativeValue".into(),
                mode: ExternClassMode::Struct,
                body: "value: i32,".into(),
                needs_ffigen: false,
            }],
        };

        assert_eq!(output.body, "first\nsecond");
        assert_eq!(
            output
                .extern_funcs
                .iter()
                .map(|func| func.partial_func_name.as_str())
                .collect_vec(),
            ["first", "second"]
        );
        let rendered = output.all_code("frb_");
        assert!(rendered.contains("pub extern \"C\" fn frb_first(value: i32)"));
        assert!(rendered.contains("pub extern \"C\" fn frb_second(value: i32)"));
        assert!(rendered.contains("pub struct NativeValue { value: i32, }"));
        assert!(rendered.find("frb_first").unwrap() < rendered.find("frb_second").unwrap());
        assert!(rendered.find("frb_second").unwrap() < rendered.find("NativeValue").unwrap());
    }

    /// Converts one or many external functions without adding unrelated output.
    #[test]
    fn converts_external_functions_into_function_only_output() {
        let one: WireRustOutputCode = extern_func("one").into();
        let many: WireRustOutputCode = vec![extern_func("two"), extern_func("three")].into();

        assert!(one.body.is_empty());
        assert!(one.extern_classes.is_empty());
        assert_eq!(one.extern_funcs.len(), 1);
        assert!(many.body.is_empty());
        assert!(many.extern_classes.is_empty());
        assert_eq!(
            many.extern_funcs
                .iter()
                .map(|func| func.partial_func_name.as_str())
                .collect_vec(),
            ["two", "three"]
        );
    }
}
