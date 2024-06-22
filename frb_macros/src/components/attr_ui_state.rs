use proc_macro::quote;
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{parse_macro_input, DeriveInput};

// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(crate) fn handle(attribute: TokenStream, item: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_name = format_ident!("my_new_field");
    let field_ty = syn::parse_str("i32").unwrap();

    quote! {
        #(#input.attrs)*
        pub struct #name #ty_generics #where_clause {
            #(#input.fields),*
            pub #field_name: #field_ty,
        }
    }
}
// frb-coverage:ignore-end
