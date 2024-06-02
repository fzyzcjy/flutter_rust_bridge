use syn::*;

pub(crate) trait SynItemStructOrEnum: Clone {
    fn attrs(&self) -> &[Attribute];
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

impl_trait!(ItemStruct);
impl_trait!(ItemEnum);
