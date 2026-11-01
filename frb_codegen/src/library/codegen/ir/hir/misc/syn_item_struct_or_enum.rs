use syn::*;

pub(crate) trait SynItemStructOrEnum: Clone {
    fn attrs(&self) -> &[Attribute];

    fn attrs_mut(&mut self) -> &mut Vec<Attribute>;

    fn generics(&self) -> &Generics;

    /// Whether `#[non_exhaustive]` forbids other crates from constructing (part of) this item.
    /// Whole-enum `#[non_exhaustive]` is excluded, since only matching needs a wildcard arm then.
    fn has_non_exhaustive_constructor(&self) -> bool;
}

macro_rules! impl_trait {
    ($name:ident, $has_non_exhaustive_constructor:ident) => {
        impl SynItemStructOrEnum for $name {
            fn attrs(&self) -> &[syn::Attribute] {
                &self.attrs
            }

            fn attrs_mut(&mut self) -> &mut Vec<syn::Attribute> {
                &mut self.attrs
            }

            fn generics(&self) -> &syn::Generics {
                &self.generics
            }

            fn has_non_exhaustive_constructor(&self) -> bool {
                $has_non_exhaustive_constructor(self)
            }
        }
    };
}

impl_trait!(ItemStruct, struct_has_non_exhaustive_constructor);
impl_trait!(ItemEnum, enum_has_non_exhaustive_constructor);

fn struct_has_non_exhaustive_constructor(item: &ItemStruct) -> bool {
    has_non_exhaustive_attr(&item.attrs)
}

fn enum_has_non_exhaustive_constructor(item: &ItemEnum) -> bool {
    (item.variants.iter()).any(|variant| has_non_exhaustive_attr(&variant.attrs))
}

fn has_non_exhaustive_attr(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("non_exhaustive"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_item_access<Item: SynItemStructOrEnum>(mut item: Item) {
        assert_eq!(item.attrs().len(), 1);
        assert_eq!(item.generics().params.len(), 1);
        item.attrs_mut().clear();
        assert!(item.attrs().is_empty());
    }

    /// Forwards attributes and generics for structs and enums.
    #[test]
    fn forwards_struct_and_enum_fields() {
        let item_struct: ItemStruct =
            syn::parse_str("#[derive(Clone)] struct Widget<T>(T);").unwrap();
        let item_enum: ItemEnum =
            syn::parse_str("#[derive(Clone)] enum Choice<T> { Value(T) }").unwrap();

        assert_item_access(item_struct);
        assert_item_access(item_enum);
    }

    /// Detects `#[non_exhaustive]` on structs and enum variants, but not on whole enums.
    #[test]
    fn detects_non_exhaustive() {
        let struct_has_non_exhaustive_constructor = |s| {
            syn::parse_str::<ItemStruct>(s)
                .unwrap()
                .has_non_exhaustive_constructor()
        };
        let enum_has_non_exhaustive_constructor = |s| {
            syn::parse_str::<ItemEnum>(s)
                .unwrap()
                .has_non_exhaustive_constructor()
        };

        assert!(struct_has_non_exhaustive_constructor(
            "#[non_exhaustive] struct A { pub x: i32 }"
        ));
        assert!(!struct_has_non_exhaustive_constructor(
            "#[derive(Clone)] struct A { pub x: i32 }"
        ));
        assert!(!enum_has_non_exhaustive_constructor(
            "#[non_exhaustive] enum B { X }"
        ));
        assert!(enum_has_non_exhaustive_constructor(
            "enum B { #[non_exhaustive] X { y: i32 }, Z }"
        ));
        assert!(!enum_has_non_exhaustive_constructor(
            "#[derive(Clone)] enum B { X, Z(i32) }"
        ));
    }
}
