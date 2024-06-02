pub(crate) trait SynItemStructOrEnum: Clone {
    fn attrs(&self) -> &[syn::Attribute];
}

macro_rules! impl_trait {
    ($name:ident) => {
        impl SynItemStructOrEnum for $name {
            fn attrs(&self) -> &[syn::Attribute] {
                &self.attrs
            }
        }
    };
}

impl_trait!(syn::ItemStruct);
impl_trait!(syn::ItemEnum);
