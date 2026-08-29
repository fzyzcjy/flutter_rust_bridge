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
    /// Produces the complete text for every code language variant.
    fn produces_complete_text_for_every_language_variant() {
        let dart = GeneralCode::Dart(GeneralDartCode {
            header: DartHeaderCode {
                file_top: "// top".to_owned(),
                import: "import 'b.dart';\nimport 'a.dart';".to_owned(),
                part: "part 'generated.dart';".to_owned(),
            },
            body: "void main() {}".to_owned(),
        });

        assert_eq!(
            dart.all_code(),
            "// top\nimport 'a.dart';\nimport 'b.dart';\npart 'generated.dart';\nvoid main() {}"
        );
        assert_eq!(
            GeneralCode::new_rust("fn main() {}".to_owned()).all_code(),
            "fn main() {}"
        );
        assert_eq!(
            GeneralCode::new_c("int main(void) {}".to_owned()).all_code(),
            "int main(void) {}"
        );
    }

    #[test]
    /// Extracts each matching language variant without changing its body.
    fn extracts_matching_language_variants() {
        assert_eq!(
            GeneralCode::Dart(GeneralDartCode::default()).dart().body,
            ""
        );
        assert_eq!(GeneralCode::new_rust("rust".to_owned()).rust().body, "rust");
        assert_eq!(GeneralCode::new_c("c".to_owned()).c().body, "c");
    }

    #[test]
    /// Appends Dart headers and bodies in their original order.
    fn appends_dart_headers_and_bodies() {
        let mut left = GeneralDartCode {
            header: DartHeaderCode {
                file_top: "first-top".to_owned(),
                import: "import 'first.dart';".to_owned(),
                part: "first-part".to_owned(),
            },
            body: "first-body".to_owned(),
        };
        left += GeneralDartCode {
            header: DartHeaderCode {
                file_top: "second-top".to_owned(),
                import: "import 'second.dart';".to_owned(),
                part: "second-part".to_owned(),
            },
            body: "second-body".to_owned(),
        };

        assert_eq!(left.header.file_top, "first-topsecond-top");
        assert_eq!(
            left.header.import,
            "import 'first.dart';import 'second.dart';"
        );
        assert_eq!(left.header.part, "first-partsecond-part");
        assert_eq!(left.body, "first-bodysecond-body");
    }

    #[test]
    /// Appends Dart headers and bodies through the enum-level addition dispatch.
    fn appends_dart_code_through_the_enum_wrapper() {
        let dart = GeneralCode::Dart(GeneralDartCode {
            header: DartHeaderCode {
                file_top: "first-top".to_owned(),
                import: "import 'first.dart';".to_owned(),
                part: "first-part".to_owned(),
            },
            body: "first-body".to_owned(),
        }) + GeneralCode::Dart(GeneralDartCode {
            header: DartHeaderCode {
                file_top: "second-top".to_owned(),
                import: "import 'second.dart';".to_owned(),
                part: "second-part".to_owned(),
            },
            body: "second-body".to_owned(),
        });

        assert_eq!(
            dart.all_code(),
            "first-topsecond-top\nimport 'first.dart';\nimport 'second.dart';\nfirst-partsecond-part\nfirst-bodysecond-body"
        );
    }

    #[test]
    /// Appends same-language Rust and C code through the enum wrapper.
    fn appends_same_language_code_through_the_enum_wrapper() {
        let rust =
            GeneralCode::new_rust("left".to_owned()) + GeneralCode::new_rust("right".to_owned());
        let c = GeneralCode::new_c("left".to_owned()) + GeneralCode::new_c("right".to_owned());

        assert_eq!(rust.all_code(), "leftright");
        assert_eq!(c.all_code(), "leftright");
    }
}
