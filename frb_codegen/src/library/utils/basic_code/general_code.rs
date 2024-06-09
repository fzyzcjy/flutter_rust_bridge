use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use crate::{impl_add_by_add_assign, simple_code_trait_impl};
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

    pub(crate) fn dart(self) -> GeneralDartCode {
        if let Self::Dart(inner) = self {
            inner
        } else {
            // frb-coverage:ignore-start
            panic!()
            // frb-coverage:ignore-end
        }
    }

    pub(crate) fn rust(self) -> GeneralRustCode {
        if let Self::Rust(inner) = self {
            inner
        } else {
            // frb-coverage:ignore-start
            panic!()
            // frb-coverage:ignore-end
        }
    }

    pub(crate) fn c(self) -> GeneralCCode {
        if let Self::C(inner) = self {
            inner
        } else {
            // frb-coverage:ignore-start
            panic!()
            // frb-coverage:ignore-end
        }
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

impl AddAssign for GeneralCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        match self {
            GeneralCode::Dart(inner) => inner.add_assign(rhs.dart()),
            GeneralCode::Rust(inner) => inner.add_assign(rhs.rust()),
            GeneralCode::C(inner) => inner.add_assign(rhs.c()),
        }
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

impl_add_by_add_assign!(GeneralCode);
simple_code_trait_impl!(GeneralDartCode);
simple_code_trait_impl!(GeneralRustCode);
simple_code_trait_impl!(GeneralCCode);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_simple() {
        let _ = GeneralCode::new_rust("a".to_owned()) + GeneralCode::new_rust("b".to_owned());
        let _ = GeneralCode::new_c("a".to_owned()) + GeneralCode::new_c("b".to_owned());
    }
}
