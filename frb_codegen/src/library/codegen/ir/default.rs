use serde::{Serialize, Serializer};
use syn::punctuated::Punctuated;
use syn::Token;

// TODO maybe should not have `syn` dependency in this `ir` mod?
crate::ir! {
pub enum IrDefaultValue {
    #[serde(serialize_with = "serialize_litstr")]
    Str(syn::LitStr),
    #[serde(serialize_with = "serialize_litbool")]
    Bool(syn::LitBool),
    #[serde(serialize_with = "serialize_litint")]
    Int(syn::LitInt),
    #[serde(serialize_with = "serialize_litfloat")]
    Float(syn::LitFloat),
    #[serde(serialize_with = "serialize_punctuated")]
    Vec(Punctuated<IrDefaultValue, Token![,]>),
}
}

fn serialize_litstr<S: Serializer>(lit: &syn::LitStr, s: S) -> Result<S::Ok, S::Error> {
    lit.value().serialize(s)
}

fn serialize_litbool<S: Serializer>(lit: &syn::LitBool, s: S) -> Result<S::Ok, S::Error> {
    lit.value().serialize(s)
}

fn serialize_litint<S: Serializer>(lit: &syn::LitInt, s: S) -> Result<S::Ok, S::Error> {
    lit.base10_parse::<i64>().unwrap().serialize(s)
}

fn serialize_litfloat<S: Serializer>(lit: &syn::LitFloat, s: S) -> Result<S::Ok, S::Error> {
    lit.base10_parse::<f64>().unwrap().serialize(s)
}

fn serialize_punctuated<S: Serializer>(
    lit: &Punctuated<IrDefaultValue, Token![,]>,
    s: S,
) -> Result<S::Ok, S::Error> {
    lit.into_iter().collect::<Vec<_>>().serialize(s)
}
