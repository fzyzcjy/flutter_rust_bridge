use crate::simple_code_trait_impl;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use serde::Serialize;
use std::ops::AddAssign;

#[derive(Clone, Debug, Serialize)]
pub(crate) enum GeneralCode {
    Dart(GeneralDartCode),
    Rust(GeneralRustCode),
    C(GeneralCCode),
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

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct GeneralCCode {
    pub body: String,
}

impl GeneralCode {
    pub(crate) fn all_code(&self) -> String {
        match self {
            GeneralCode::Dart(inner) => inner.all_code(),
            GeneralCode::Rust(inner) => inner.all_code(),
            GeneralCode::C(inner) => inner.all_code(),
        }
    }

    pub(crate) fn new_rust(body: String) -> GeneralCode {
        GeneralCode::Rust(GeneralRustCode { body })
    }

    pub(crate) fn new_c(body: String) -> GeneralCode {
        GeneralCode::C(GeneralCCode { body })
    }
}

impl GeneralDartCode {
    pub(crate) fn all_code(&self) -> String {
        format!("{}\n{}", self.header.all_code(), self.body)
    }
}

impl GeneralRustCode {
    pub(crate) fn all_code(&self) -> String {
        self.body.clone()
    }
}

impl GeneralCCode {
    pub(crate) fn all_code(&self) -> String {
        self.body.clone()
    }
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

impl AddAssign for GeneralCCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.body += &rhs.body;
    }
}

simple_code_trait_impl!(GeneralDartCode);
simple_code_trait_impl!(GeneralRustCode);
simple_code_trait_impl!(GeneralCCode);
