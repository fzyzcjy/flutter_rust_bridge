use crate::basic_code_impl;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use itertools::Itertools;
use std::ops::{Add, AddAssign};

#[derive(Default, Clone)]
pub(crate) struct WireRustOutputCode {
    pub(crate) direct_code: String,
    pub(crate) extern_funcs: Vec<ExternFunc>,
}

basic_code_impl!(WireRustOutputCode);

impl WireRustOutputCode {
    pub(crate) fn all_code(&self) -> String {
        format!(
            "{}\n{}",
            self.direct_code,
            self.extern_funcs
                .iter()
                .map(|func| func.generate())
                .join("\n")
        )
    }
}

impl AddAssign for WireRustOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.direct_code += &rhs.direct_code;
        self.extern_funcs.extend(rhs.extern_funcs);
    }
}

impl From<String> for WireRustOutputCode {
    fn from(direct_code: String) -> Self {
        Self {
            direct_code,
            ..Default::default()
        }
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
            direct_code: "".to_string(),
            extern_funcs,
        }
    }
}
