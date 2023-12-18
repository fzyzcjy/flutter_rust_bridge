#[doc(hidden)]
#[macro_export]
macro_rules! enum_map {
    ($struct_name:ident, $enum_name:ident; $($enum_variants_pascal:ident),*; $($enum_variants_snake:ident),*;) => {
        #[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct $struct_name<T> {
            $(
                pub $enum_variants_snake: T,
            )*
        }

        impl<T> core::ops::Index<$enum_name> for $struct_name<T> {
            type Output = T;

            fn index(&self, index: $enum_name) -> &Self::Output {
                match index {
                    $(
                        $enum_name::$enum_variants_pascal => &self.$enum_variants_snake,
                    )*
                }
            }
        }

        impl<T> $struct_name<T> {
            #[allow(dead_code)]
            pub fn get(self, index: $enum_name) -> T {
                match index {
                    $(
                        $enum_name::$enum_variants_pascal => self.$enum_variants_snake,
                    )*
                }
            }

            #[allow(dead_code)]
            pub fn into_vec(self) -> Vec<T> {
                vec![
                    $(
                        self.$enum_variants_snake,
                    )*
                ]
            }
        }
    };
}
