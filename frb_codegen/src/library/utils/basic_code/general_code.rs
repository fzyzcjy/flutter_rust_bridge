use crate::simple_code_trait_impl;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use serde::Serialize;
use std::ops::AddAssign;

#[derive(Clone, Debug, Serialize)]
pub(crate) enum GeneralCode {
    Dart(GeneralDartCode),
    Rust(GeneralRustCode),
}

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct GeneralDartCode {
    pub header: DartHeaderCode,
    pub body: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct GeneralRustCode {
    pub body: String,
}

impl AddAssign for GeneralDartCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.header += rhs.header;
        self.body += &rhs.body;
    }
}

impl AddAssign for GeneralRustCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.body += &rhs.body;
    }
}

simple_code_trait_impl!(GeneralDartCode);
simple_code_trait_impl!(GeneralRustCode);
