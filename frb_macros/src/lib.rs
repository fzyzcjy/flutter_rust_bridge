use proc_macro::TokenStream;
use quote::quote;
use syn::{self, *};

fn is_marker_attr(attr: &Attribute) -> bool {
    matches!(
        attr.parse_meta(),
        Ok(Meta::Path(path)) if path.is_ident("frb")
    )
}

/// Attribute to guide code generation.
#[proc_macro_attribute]
pub fn frb(_: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(mut input) = syn::parse::<DeriveInput>(item.clone()) {
        input.data = match input.data {
            Data::Struct(mut st) => {
                st.fields = remove_marker_attr_from_fields(st.fields);
                Data::Struct(st)
            }
            Data::Enum(mut enu) => {
                enu.variants = enu
                    .variants
                    .into_iter()
                    .map(remove_marker_attr_from_variant)
                    .collect();
                Data::Enum(enu)
            }
            Data::Union(mut uni) => {
                uni.fields = remove_marker_attr_from_named_fields(uni.fields);
                Data::Union(uni)
            }
        };
        quote! { #input }.into()
    } else if let Ok(mut input) = syn::parse::<ItemStruct>(item.clone()) {
        input.fields = remove_marker_attr_from_fields(input.fields);
        quote! { #input }.into()
    } else if let Ok(mut input) = syn::parse::<ItemEnum>(item.clone()) {
        input.variants = input
            .variants
            .into_iter()
            .map(remove_marker_attr_from_variant)
            .collect();
        quote! { #input }.into()
    } else if let Ok(mut input) = syn::parse::<ItemUnion>(item.clone()) {
        input.fields = remove_marker_attr_from_named_fields(input.fields);
        quote! { #input }.into()
    } else {
        item
    }
}

fn remove_marker_attr_from_variant(mut variant: Variant) -> Variant {
    variant.fields = remove_marker_attr_from_fields(variant.fields);
    variant
}

fn remove_marker_attr_from_field(mut field: Field) -> Field {
    field.attrs = field.attrs.into_iter().filter(is_marker_attr).collect();
    field
}

fn remove_marker_attr_from_named_fields(mut fields: FieldsNamed) -> FieldsNamed {
    fields.named = fields
        .named
        .into_iter()
        .map(remove_marker_attr_from_field)
        .collect();
    fields
}

fn remove_marker_attr_from_fields(fields: Fields) -> Fields {
    match fields {
        Fields::Named(fields) => Fields::Named(remove_marker_attr_from_named_fields(fields)),
        Fields::Unnamed(mut fields) => {
            fields.unnamed = fields
                .unnamed
                .into_iter()
                .map(remove_marker_attr_from_field)
                .collect();
            Fields::Unnamed(fields)
        }
        _ => fields,
    }
}
