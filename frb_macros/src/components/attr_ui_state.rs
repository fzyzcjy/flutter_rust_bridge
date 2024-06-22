use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, ItemStruct};

// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(crate) fn handle(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // ref: https://users.rust-lang.org/t/solved-derive-and-proc-macro-add-field-to-an-existing-struct/52307
    let mut ast = parse_macro_input!(item as ItemStruct);
    if let syn::Fields::Named(ref mut fields) = ast.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub(crate) base_state: crate::frb_generated::BaseRustState })
                .unwrap(),
        );
    }

    let struct_ident = &ast.ident;

    (quote! {
        #ast

        impl #struct_ident {
            pub fn set_base_state(&mut self, base_state: crate::frb_generated::BaseRustState) {
                self.base_state = base_state;
            }
        }
    })
    .into()
}
// frb-coverage:ignore-end
