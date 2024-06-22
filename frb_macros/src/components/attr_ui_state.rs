use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::parse::Parser;

// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(crate) fn handle(attribute: TokenStream, item: proc_macro::TokenStream) -> TokenStream {
    let mut ast: DeriveInput = parse_macro_input!(item as DeriveInput);

    // ref: https://users.rust-lang.org/t/solved-derive-and-proc-macro-add-field-to-an-existing-struct/52307
    match &mut ast.data {
        syn::Data::Struct(ref mut data_struct) => {
            match &mut data_struct.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named.parse2(quote! { pub a: String }).unwrap(),
                    );
                }
                _ => {}
            }
        }
        _ => panic!("`add_field` has to be used with structs "),
    }

    quote! { #ast }
}
// frb-coverage:ignore-end
