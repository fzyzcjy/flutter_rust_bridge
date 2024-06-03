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
