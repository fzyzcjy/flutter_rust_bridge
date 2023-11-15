pub(crate) mod markers;
pub(crate) mod source_graph;
pub(crate) mod ty;

use std::borrow::Cow;
use std::collections::HashMap;
use std::default::Default as _;
use std::string::String;

use anyhow::Context;
use log::debug;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::*;
use topological_sort::TopologicalSort;

use crate::ir::*;

use crate::generator::rust::HANDLER_NAME;
use crate::parser::source_graph::Crate;
use crate::parser::ty::TypeParser;
use crate::parser::IrType::{EnumRef, StructRef};
use crate::utils::method::FunctionName;

use self::ty::convert_ident_str;

const STREAM_SINK_IDENT: &str = "StreamSink";

mod error;
pub use error::Error;

impl<'a> Parser<'a> {
    fn parse(mut self, source_rust_content: &str, src_fns: Vec<ItemFn>) -> ParserResult<IrPack> {
        let funcs = src_fns
            .iter()
            .map(|f| self.parse_function(f))
            .collect::<ParserResult<Vec<_>>>()?;

        let has_executor = source_rust_content.contains(HANDLER_NAME);

        let (struct_pool, enum_pool) = self.type_parser.consume();

        Ok(IrPack {
            funcs,
            struct_pool,
            enum_pool,
            has_executor,
        })
    }
}

fn extract_comments(attrs: &[Attribute]) -> Vec<IrComment> {
    attrs
        .iter()
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            }) if path.is_ident("doc") => Some(IrComment::from(lit.value().as_ref())),
            _ => None,
        })
        .collect()
}

pub mod frb_keyword {
    syn::custom_keyword!(mirror);
    syn::custom_keyword!(non_final);
    syn::custom_keyword!(dart_metadata);
    syn::custom_keyword!(import);
}

#[derive(Clone, Debug)]
pub struct NamedOption<K, V> {
    pub name: K,
    pub value: V,
}

impl<K: Parse + std::fmt::Debug, V: Parse> Parse for NamedOption<K, V> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name: K = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(Self { name, value })
    }
}

#[derive(Clone, Debug)]
pub struct MirrorOption(Path);

impl Parse for MirrorOption {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let path: Path = content.parse()?;
        Ok(Self(path))
    }
}

#[derive(Clone, Debug)]
pub struct MetadataAnnotations(Vec<IrDartAnnotation>);

impl Parse for IrDartAnnotation {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
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
impl Parse for MetadataAnnotations {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let annotations =
            Punctuated::<IrDartAnnotation, syn::Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect();
        Ok(Self(annotations))
    }
}

#[derive(Clone, Debug)]
pub struct DartImports(Vec<IrDartImport>);

impl Parse for IrDartImport {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
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
impl Parse for DartImports {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let imports = Punctuated::<IrDartImport, syn::Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(imports))
    }
}

enum FrbOption {
    Mirror(MirrorOption),
    NonFinal,
    Metadata(NamedOption<frb_keyword::dart_metadata, MetadataAnnotations>),
    Default(DefaultValues),
}

impl Parse for FrbOption {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(frb_keyword::mirror) {
            input.parse().map(FrbOption::Mirror)
        } else if lookahead.peek(frb_keyword::non_final) {
            input
                .parse::<frb_keyword::non_final>()
                .map(|_| FrbOption::NonFinal)
        } else if lookahead.peek(frb_keyword::dart_metadata) {
            input.parse().map(FrbOption::Metadata)
        } else if lookahead.peek(Token![default]) {
            input.parse::<Token![default]>()?;
            input.parse::<Token![=]>()?;
            input.parse().map(FrbOption::Default)
        } else {
            Err(lookahead.error())
        }
    }
}

impl DefaultValues {
    pub(crate) fn extract(attrs: &[Attribute]) -> Option<Self> {
        let defaults = attrs
            .iter()
            .filter(|attr| attr.path().is_ident("frb"))
            .map(|attr| attr.parse_args::<FrbOption>())
            .filter_map(|attr| {
                if let Ok(FrbOption::Default(default)) = attr {
                    Some(default)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        match &defaults[..] {
            [] => None,
            [single] => Some(single.clone()),
            [.., last] => {
                log::warn!("Only one `default = ..` attribute is expected; taking the last one");
                Some(last.clone())
            }
        }
    }
    pub(crate) fn to_dart(&self) -> Cow<str> {
        match self {
            Self::Bool(lit) => if lit.value { "true" } else { "false" }.into(),
            Self::Str(lit) => format!("r\"{}\"", lit.value()).into(),
            Self::Int(lit) => lit.base10_digits().into(),
            Self::Float(lit) => lit.base10_digits().into(),
            Self::Vec(lit) => format!(
                "const [{}]",
                lit.iter().map(Self::to_dart).collect::<Vec<_>>().join(",")
            )
            .into(),
        }
    }
}

impl Parse for DefaultValues {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

/// syn -> string https://github.com/dtolnay/syn/issues/294
fn type_to_string(ty: &Type) -> String {
    quote!(#ty).to_string().replace(' ', "")
}
