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
