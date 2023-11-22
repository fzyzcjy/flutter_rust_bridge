pub(crate) trait BasicCode {
    fn body(&self) -> &str;

    fn new_body(body: String) -> Self;
}

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

        impl BasicCode for $name {
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
