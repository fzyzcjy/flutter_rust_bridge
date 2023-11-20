use std::ops::{Add, AddAssign};

#[doc(hidden)]
#[macro_export]
macro_rules! basic_code_impl {
    ($name:ident) => {
        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                value.to_owned().into()
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }

        impl std::iter::FromIterator<$name> for $name {
            fn from_iter<A: IntoIterator<Item = $name>>(iter: A) -> Self {
                iter.into_iter().fold(Default::default(), |a, b| a + b)
            }
        }
    };
}
