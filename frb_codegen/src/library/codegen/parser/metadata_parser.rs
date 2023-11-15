use crate::codegen::ir::annotation::IrDartAnnotation;
use crate::codegen::ir::default::IrDefaultValue;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::*;

fn parse_metadata(attrs: &[Attribute]) -> Vec<IrDartAnnotation> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("frb"))
        .map(|attr| attr.parse_args::<FrbOption>())
        .flat_map(|frb_option| match frb_option {
            Ok(FrbOption::Metadata(NamedOption {
                name: _,
                value: MetadataAnnotations(annotations),
            })) => annotations,
            _ => vec![],
        })
        .collect()
}

mod frb_keyword {
    syn::custom_keyword!(mirror);
    syn::custom_keyword!(non_final);
    syn::custom_keyword!(dart_metadata);
    syn::custom_keyword!(import);
}

enum FrbOption {
    Mirror(MirrorOption),
    NonFinal,
    Metadata(NamedOption<frb_keyword::dart_metadata, MetadataAnnotations>),
    Default(IrDefaultValue),
}

impl Parse for FrbOption {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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

#[derive(Clone, Debug)]
pub struct MirrorOption(Path);

impl Parse for MirrorOption {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let path: Path = content.parse()?;
        Ok(Self(path))
    }
}

#[derive(Clone, Debug)]
pub struct MetadataAnnotations(Vec<IrDartAnnotation>);

impl Parse for MetadataAnnotations {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let annotations = Punctuated::<IrDartAnnotation, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(annotations))
    }
}

impl Parse for IrDefaultValue {
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
