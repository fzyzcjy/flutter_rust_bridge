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
