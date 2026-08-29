use syn::*;

pub(crate) trait SynItemStructOrEnum: Clone {
    fn attrs(&self) -> &[Attribute];

    fn attrs_mut(&mut self) -> &mut Vec<Attribute>;

    fn generics(&self) -> &Generics;
}

macro_rules! impl_trait {
    ($name:ident) => {
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
        }
    };
}

impl_trait!(ItemStruct);
impl_trait!(ItemEnum);

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
}
