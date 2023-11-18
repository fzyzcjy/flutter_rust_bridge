use crate::codegen::generator::wire::rust::extern_func::ExternFunc;
use std::iter::FromIterator;
use std::ops::{Add, AddAssign};

#[derive(Default)]
pub(crate) struct WireRustCode {
    pub(crate) direct_code: String,
    pub(crate) extern_funcs: Vec<ExternFunc>,
}

impl Add for WireRustCode {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for WireRustCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.direct_code += &rhs.direct_code;
        self.extern_funcs.extend(rhs.extern_funcs);
    }
}

impl From<String> for WireRustCode {
    fn from(code: String) -> Self {
        Self {
            direct_code: code,
            extern_funcs: vec![],
        }
    }
}

impl From<&str> for WireRustCode {
    fn from(code: &str) -> Self {
        code.to_owned().into()
    }
}

impl From<ExternFunc> for WireRustCode {
    fn from(value: ExternFunc) -> Self {
        vec![value].into()
    }
}

impl From<Vec<ExternFunc>> for WireRustCode {
    fn from(extern_funcs: Vec<ExternFunc>) -> Self {
        Self {
            direct_code: "".to_string(),
            extern_funcs,
        }
    }
}

impl FromIterator<WireRustCode> for WireRustCode {
    fn from_iter<A: IntoIterator<Item = WireRustCode>>(iter: A) -> Self {
        iter.into_iter().fold(Default::default(), |a, b| a + b)
    }
}
