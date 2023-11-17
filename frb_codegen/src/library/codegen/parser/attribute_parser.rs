use crate::codegen::ir::annotation::IrDartAnnotation;
use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::import::IrDartImport;
use crate::if_then_some;
use itertools::Itertools;
use serde::{Serialize, Serializer};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::*;

const METADATA_IDENT: &str = "frb";

pub(crate) struct FrbAttributes(Vec<FrbAttribute>);

impl FrbAttributes {
    pub(crate) fn parse(attrs: &[Attribute]) -> Result<Self> {
        Ok(Self(
            attrs
                .iter()
                .filter(|attr| attr.path().is_ident(METADATA_IDENT))
                .map(|attr| attr.parse_args::<FrbAttribute>())
                .collect::<Result<Vec<_>>>()?,
        ))
    }

    pub(crate) fn default_value(&self) -> Option<IrDefaultValue> {
        let candidates = self
            .0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Default(default) = item, default.clone()),
            )
            .collect_vec();
        if candidates.len() > 1 {
            log::warn!("Only one `default = ..` attribute is expected; taking the last one");
        }
        candidates.last().map(|item| item.to_ir_default_value())
    }

    pub(crate) fn non_final(&self) -> bool {
        self.0
            .iter()
            .any(|item| matches!(item, FrbAttribute::NonFinal))
    }

    pub(crate) fn mirror(&self) -> Vec<Path> {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Mirror(mirror) = item, mirror.0.clone()),
            )
            .collect()
    }

    pub(crate) fn dart_metadata(&self) -> Vec<IrDartAnnotation> {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Metadata(metadata) = item, metadata.value.0.clone()),
            )
            .flatten()
            .collect()
    }
}

mod frb_keyword {
    syn::custom_keyword!(mirror);
    syn::custom_keyword!(non_final);
    syn::custom_keyword!(dart_metadata);
    syn::custom_keyword!(import);
}

enum FrbAttribute {
    Mirror(FrbAttributeMirror),
    NonFinal,
    Metadata(NamedOption<frb_keyword::dart_metadata, FrbAttributeDartMetadata>),
    Default(FrbAttributeDefaultValue),
}

impl Parse for FrbAttribute {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(frb_keyword::mirror) {
            input.parse().map(FrbAttribute::Mirror)
        } else if lookahead.peek(frb_keyword::non_final) {
            input
                .parse::<frb_keyword::non_final>()
                .map(|_| FrbAttribute::NonFinal)
        } else if lookahead.peek(frb_keyword::dart_metadata) {
            input.parse().map(FrbAttribute::Metadata)
        } else if lookahead.peek(Token![default]) {
            input.parse::<Token![default]>()?;
            input.parse::<Token![=]>()?;
            input.parse().map(FrbAttribute::Default)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone, Debug)]
struct NamedOption<K, V> {
    name: K,
    value: V,
}

impl<K: Parse + std::fmt::Debug, V: Parse> Parse for NamedOption<K, V> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name: K = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(Self { name, value })
    }
}

#[derive(Clone, Debug)]
struct FrbAttributeMirror(Path);

impl Parse for FrbAttributeMirror {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let path: Path = content.parse()?;
        Ok(Self(path))
    }
}

#[derive(Clone, Debug)]
struct FrbAttributeDartMetadata(Vec<IrDartAnnotation>);

impl Parse for FrbAttributeDartMetadata {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let annotations = Punctuated::<IrDartAnnotation, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(annotations))
    }
}

#[derive(Clone, Debug)]
struct DartImports(Vec<IrDartImport>);

impl Parse for DartImports {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let imports = Punctuated::<IrDartImport, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(imports))
    }
}

impl Parse for IrDartImport {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let uri: LitStr = input.parse()?;
        let alias: Option<String> = if input.peek(token::As) {
            let _ = input.parse::<token::As>()?;
            let alias: Ident = input.parse()?;
            Some(alias.to_string())
        } else {
            None
        };
        Ok(Self {
            uri: uri.value(),
            alias,
        })
    }
}

impl Parse for IrDartAnnotation {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let annotation: LitStr = input.parse()?;
        let library = if input.peek(frb_keyword::import) {
            let _ = input.parse::<frb_keyword::import>()?;
            let library: IrDartImport = input.parse()?;
            Some(library)
        } else {
            None
        };
        Ok(Self {
            content: annotation.value(),
            library,
        })
    }
}

#[derive(Clone, Serialize)]
enum FrbAttributeDefaultValue {
    #[serde(serialize_with = "serialize_litstr")]
    Str(syn::LitStr),
    #[serde(serialize_with = "serialize_litbool")]
    Bool(syn::LitBool),
    #[serde(serialize_with = "serialize_litint")]
    Int(syn::LitInt),
    #[serde(serialize_with = "serialize_litfloat")]
    Float(syn::LitFloat),
    #[serde(serialize_with = "serialize_punctuated")]
    Vec(Punctuated<FrbAttributeDefaultValue, Token![,]>),
}

impl Parse for FrbAttributeDefaultValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let lh = input.lookahead1();
        if lh.peek(token::Bracket) {
            let inner;
            bracketed!(inner in input);
            Punctuated::parse_terminated(&inner).map(Self::Vec)
        } else if lh.peek(syn::LitStr) {
            input.parse().map(Self::Str)
        } else if lh.peek(syn::LitBool) {
            input.parse().map(Self::Bool)
        } else if lh.peek(syn::LitFloat) {
            input.parse().map(Self::Float)
        } else if lh.peek(syn::LitInt) {
            input.parse().map(Self::Int)
        } else {
            Err(lh.error())
        }
    }
}

impl FrbAttributeDefaultValue {
    fn to_ir_default_value(&self) -> IrDefaultValue {
        match self {
            Self::Str(lit) => IrDefaultValue::String {
                content: lit.value(),
            },

            // other types
            Self::Bool(lit) => IrDefaultValue::Others {
                dart_literal: (if lit.value { "true" } else { "false" }).to_owned(),
            },
            Self::Int(lit) => IrDefaultValue::Others {
                dart_literal: lit.base10_digits().into(),
            },
            Self::Float(lit) => IrDefaultValue::Others {
                dart_literal: lit.base10_digits().into(),
            },
            Self::Vec(lit) => IrDefaultValue::Others {
                dart_literal: format!(
                    "const [{}]",
                    lit.iter()
                        .map(|item| item.to_ir_default_value().to_dart_literal().to_string())
                        .collect_vec()
                        .join(",")
                ),
            },
        }
    }
}

fn serialize_litstr<S: Serializer>(
    lit: &syn::LitStr,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.value().serialize(s)
}

fn serialize_litbool<S: Serializer>(
    lit: &syn::LitBool,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.value().serialize(s)
}

fn serialize_litint<S: Serializer>(
    lit: &syn::LitInt,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.base10_parse::<i64>().unwrap().serialize(s)
}

fn serialize_litfloat<S: Serializer>(
    lit: &syn::LitFloat,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.base10_parse::<f64>().unwrap().serialize(s)
}

fn serialize_punctuated<S: Serializer>(
    lit: &Punctuated<FrbAttributeDefaultValue, Token![,]>,
    s: S,
) -> std::result::Result<S::Ok, S::Error> {
    lit.into_iter().collect_vec().serialize(s)
}
