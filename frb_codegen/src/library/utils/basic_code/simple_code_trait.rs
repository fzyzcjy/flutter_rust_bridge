pub(crate) trait SimpleCodeTrait {
    fn body(&self) -> &str;

    fn new_body(body: String) -> Self;
}

#[doc(hidden)]
#[macro_export]
macro_rules! simple_code_trait_impl {
    ($name:ident) => {
        $crate::impl_add_by_add_assign!($name);

        impl From<String> for $name {
            #[allow(clippy::needless_update)]
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

        impl $crate::utils::basic_code::simple_code_trait::SimpleCodeTrait for $name {
            fn body(&self) -> &str {
                &self.body
            }

            #[allow(clippy::needless_update)]
            fn new_body(body: String) -> Self {
                Self {
                    body,
                    ..Default::default()
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_add_by_add_assign {
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
