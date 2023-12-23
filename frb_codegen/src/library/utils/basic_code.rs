use itertools::Itertools;
use serde::Serialize;
use std::ops::AddAssign;

pub(crate) trait BasicCode {
    fn body(&self) -> &str;

    fn new_body(body: String) -> Self;
}

#[doc(hidden)]
#[macro_export]
macro_rules! basic_code_partial_impl {
    ($name:ident) => {
        impl std::ops::Add for $name {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }

        // unused now, thus uncomment only when needed
        // impl std::iter::FromIterator<$name> for $name {
        //     fn from_iter<A: IntoIterator<Item = $name>>(iter: A) -> Self {
        //         iter.into_iter().fold(Default::default(), |a, b| a + b)
        //     }
        // }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! basic_code_impl {
    ($name:ident) => {
        $crate::basic_code_partial_impl!($name);

        impl From<String> for $name {
            fn from(body: String) -> Self {
                Self {
                    body,
                    ..Default::default()
                }
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                value.to_owned().into()
            }
        }

        impl $crate::utils::basic_code::BasicCode for $name {
            fn body(&self) -> &str {
                &self.body
            }

            fn new_body(body: String) -> Self {
                Self {
                    body,
                    ..Default::default()
                }
            }
        }
    };
}

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct DartBasicHeaderCode {
    pub file_top: String,
    pub import: String,
    pub part: String,
}

basic_code_partial_impl!(DartBasicHeaderCode);

impl AddAssign for DartBasicHeaderCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.file_top += &rhs.file_top;
        self.import += &rhs.import;
        self.part += &rhs.part;
    }
}

impl DartBasicHeaderCode {
    pub(crate) fn all_code(&self) -> String {
        let import = optimize_imports(&self.import);
        format!("{}\n{}\n{}", self.file_top, import, self.part)
    }
}

fn optimize_imports(raw: &str) -> String {
    raw.split_inclusive(&['\n', ';'][..])
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .sorted()
        .dedup()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_optimize_imports_simple() {
        assert_eq!(
            optimize_imports(
                "
                import 'orange.dart';
                import 'apple.dart';
                import 'orange.dart';
                "
            ),
            "import 'apple.dart';\nimport 'orange.dart';"
        );
    }

    #[test]
    pub fn test_optimize_imports_missing_newline() {
        assert_eq!(
            optimize_imports(" import 'orange.dart'; import 'apple.dart';import 'orange.dart';"),
            "import 'apple.dart';\nimport 'orange.dart';"
        );
    }
}
