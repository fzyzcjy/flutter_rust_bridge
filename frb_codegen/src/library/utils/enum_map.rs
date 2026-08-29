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

#[cfg(test)]
mod tests {
    #[derive(Clone, Copy)]
    pub enum Flavor {
        Vanilla,
        Chocolate,
    }

    crate::enum_map!(FlavorMap, Flavor; Vanilla, Chocolate; vanilla, chocolate;);

    /// Indexes, extracts, and vectorizes values in declaration order.
    #[test]
    fn test_enum_map_indexes_extracts_and_preserves_declared_order() {
        let map = FlavorMap {
            vanilla: "vanilla",
            chocolate: "chocolate",
        };

        assert_eq!(map[Flavor::Vanilla], "vanilla");
        assert_eq!(map[Flavor::Chocolate], "chocolate");
        assert_eq!(map.clone().get(Flavor::Chocolate), "chocolate");
        assert_eq!(map.into_vec(), vec!["vanilla", "chocolate"]);
    }

    /// Uses the value type default for every generated field.
    #[test]
    fn test_enum_map_default_initializes_every_declared_field() {
        assert_eq!(FlavorMap::<u8>::default().into_vec(), vec![0, 0]);
    }
}
