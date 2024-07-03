use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::mir::annotation::MirDartAnnotation;
use crate::codegen::ir::mir::default::MirDefaultValue;
use crate::codegen::ir::mir::func::MirFuncAccessorMode;
use crate::codegen::ir::mir::import::MirDartImport;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::if_then_some;
use anyhow::Context;
use itertools::Itertools;
use serde::{Serialize, Serializer};
use syn::parse::{Lookahead1, Parse, ParseStream, Peek};
use syn::punctuated::Punctuated;
use syn::*;

const METADATA_IDENT: &str = "frb";

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct FrbAttributes(Vec<FrbAttribute>);

impl FrbAttributes {
    pub(crate) fn parse(attrs: &[Attribute]) -> anyhow::Result<Self> {
        Ok(Self(
            attrs
                .iter()
                .map(transform_doc_comment)
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .filter(|attr| attr.path().segments.last().unwrap().ident == METADATA_IDENT)
                .map(|attr| {
                    if matches!(attr.meta, Meta::Path(_)) {
                        Ok(FrbAttributesInner(vec![FrbAttribute::Noop]))
                    } else {
                        attr.parse_args::<FrbAttributesInner>()
                            .with_context(|| format!("attr={:?}", quote::quote!(#attr).to_string()))
                    }
                })
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .flat_map(|x| x.0)
                .collect(),
        ))
    }

    pub(crate) fn default_value(&self) -> Option<MirDefaultValue> {
        let candidates = self
            .0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Default(default) = item, default.clone()),
            )
            .collect_vec();
        if candidates.len() > 1 {
            // We do not care about details of this warning message
            // frb-coverage:ignore-start
            log::warn!("Only one `default = ..` attribute is expected; taking the last one");
            // frb-coverage:ignore-end
        }
        candidates.last().map(|item| item.to_mir_default_value())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn non_final(&self) -> bool {
        self.any_eq(&FrbAttribute::NonFinal)
    }

    pub(crate) fn dart_async(&self) -> Option<bool> {
        if self.any_eq(&FrbAttribute::Sync) {
            Some(false)
        } else if self.any_eq(&FrbAttribute::DartAsync) {
            Some(true)
        } else {
            None
        }
    }

    pub(crate) fn stream_dart_await(&self) -> bool {
        self.any_eq(&FrbAttribute::StreamDartAwait)
    }

    pub(crate) fn accessor(&self) -> Option<MirFuncAccessorMode> {
        if self.any_eq(&FrbAttribute::Getter) {
            Some(MirFuncAccessorMode::Getter)
        } else if self.any_eq(&FrbAttribute::Setter) {
            Some(MirFuncAccessorMode::Setter)
        } else {
            None
        }
    }

    pub(crate) fn init(&self) -> bool {
        self.any_eq(&FrbAttribute::Init)
    }

    pub(crate) fn ignore(&self) -> bool {
        self.any_eq(&FrbAttribute::Ignore)
    }

    pub(crate) fn opaque(&self) -> Option<bool> {
        if self.any_eq(&FrbAttribute::Opaque) {
            Some(true)
        } else if self.any_eq(&FrbAttribute::NonOpaque) {
            Some(false)
        } else if self.ui_state() {
            // When `#[frb(ui_state)]`, auto infer `#[frb(opaque)]`
            Some(true)
        } else {
            None
        }
    }

    pub(crate) fn generate_hash(&self) -> bool {
        !self.any_eq(&FrbAttribute::NonHash)
    }

    pub(crate) fn generate_eq(&self) -> bool {
        !self.any_eq(&FrbAttribute::NonEq)
    }

    pub(crate) fn positional(&self) -> bool {
        self.any_eq(&FrbAttribute::Positional)
    }

    pub(crate) fn proxy(&self) -> bool {
        self.any_eq(&FrbAttribute::Proxy)
    }

    pub(crate) fn external(&self) -> bool {
        self.any_eq(&FrbAttribute::External)
    }

    pub(crate) fn type_64bit_int(&self) -> bool {
        self.any_eq(&FrbAttribute::Type64bitInt)
    }

    // pub(crate) fn generate_implementor_enum(&self) -> bool {
    //     self.any_eq(&FrbAttribute::GenerateImplEnum)
    // }

    pub(crate) fn rust_opaque_codec(&self) -> Option<RustOpaqueCodecMode> {
        if self.any_eq(&FrbAttribute::RustOpaqueCodecMoi) {
            Some(RustOpaqueCodecMode::Moi)
        } else {
            None
        }
    }

    pub(crate) fn codec_mode_pack(&self) -> Option<CodecModePack> {
        if self.any_eq(&FrbAttribute::Serialize) {
            Some(CodecModePack {
                dart2rust: CodecMode::Sse,
                rust2dart: CodecMode::Sse,
            })
        } else if self.any_eq(&FrbAttribute::SemiSerialize) {
            Some(CodecModePack {
                dart2rust: CodecMode::Cst,
                rust2dart: CodecMode::Sse,
            })
        } else {
            None
        }
    }

    fn any_eq(&self, target: &FrbAttribute) -> bool {
        self.0.iter().any(|item| item == target)
    }

    pub(crate) fn mirror(&self) -> Vec<Path> {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Mirror(mirror) = item, mirror.0.clone()),
            )
            .flatten()
            .collect()
    }

    pub(crate) fn dart_metadata(&self) -> Vec<MirDartAnnotation> {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::DartMetadata(metadata) = item, metadata.value.0.clone()),
            )
            .flatten()
            .collect()
    }

    pub(crate) fn dart_code(&self) -> String {
        self.0
            .iter()
            .filter_map(
                |item| if_then_some!(let FrbAttribute::DartCode(inner) = item, inner.0.clone()),
            )
            .join("\n\n")
    }

    pub(crate) fn name(&self) -> Option<String> {
        self.0
            .iter()
            .filter_map(|item| if_then_some!(let FrbAttribute::Name(inner) = item, inner.0.clone()))
            .next()
    }

    pub(crate) fn dart2rust(&self) -> Option<FrbAttributeSerDes> {
        (self.0.iter())
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Dart2Rust(inner) = item, inner.clone()),
            )
            .next()
    }

    pub(crate) fn rust2dart(&self) -> Option<FrbAttributeSerDes> {
        (self.0.iter())
            .filter_map(
                |item| if_then_some!(let FrbAttribute::Rust2Dart(inner) = item, inner.clone()),
            )
            .next()
    }

    pub(crate) fn ui_state(&self) -> bool {
        self.any_eq(&FrbAttribute::UiState)
    }

    pub(crate) fn ui_mutation(&self) -> bool {
        self.any_eq(&FrbAttribute::UiMutation)
    }
}

fn transform_doc_comment(attr: &Attribute) -> anyhow::Result<Attribute> {
    if let Some(doc_comment) = extract_doc_comment(attr) {
        if let Some(inner) = doc_comment.trim().strip_prefix("flutter_rust_bridge:") {
            return parse_syn_attribute(&format!("#[frb({inner})]"));
        }
    }
    Ok(attr.to_owned())
}

fn extract_doc_comment(attr: &Attribute) -> Option<String> {
    if let Meta::NameValue(MetaNameValue {
        path: Path { segments, .. },
        value: Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }),
        ..
    }) = &attr.meta
    {
        if segments.len() == 1 && segments.first().unwrap().ident == "doc" {
            return Some(lit_str.value());
        }
    }
    None
}

fn parse_syn_attribute(raw: &str) -> anyhow::Result<Attribute> {
    let code = format!("{raw} fn f() {{}}");
    let fn_ast: ItemFn = parse_str(&code)?;
    Ok(fn_ast.attrs[0].to_owned())
}

mod frb_keyword {
    syn::custom_keyword!(mirror);
    syn::custom_keyword!(non_final);
    syn::custom_keyword!(sync);
    syn::custom_keyword!(dart_async);
    syn::custom_keyword!(stream_dart_await);
    syn::custom_keyword!(getter);
    syn::custom_keyword!(setter);
    syn::custom_keyword!(init);
    syn::custom_keyword!(ignore);
    syn::custom_keyword!(opaque);
    syn::custom_keyword!(non_opaque);
    syn::custom_keyword!(non_hash);
    syn::custom_keyword!(non_eq);
    syn::custom_keyword!(positional);
    syn::custom_keyword!(proxy);
    syn::custom_keyword!(external);
    syn::custom_keyword!(type_64bit_int);
    syn::custom_keyword!(generate_implementor_enum);
    syn::custom_keyword!(rust_opaque_codec_moi);
    syn::custom_keyword!(serialize);
    syn::custom_keyword!(semi_serialize);
    syn::custom_keyword!(dart_metadata);
    syn::custom_keyword!(import);
    syn::custom_keyword!(default);
    syn::custom_keyword!(dart_code);
    syn::custom_keyword!(name);
    syn::custom_keyword!(rust2dart);
    syn::custom_keyword!(dart2rust);
    syn::custom_keyword!(dart_type);
    syn::custom_keyword!(ui_state);
    syn::custom_keyword!(ui_mutation);
}

struct FrbAttributesInner(Vec<FrbAttribute>);

impl Parse for FrbAttributesInner {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Punctuated::<FrbAttribute, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect_vec();
        if items.is_empty() {
            items = vec![FrbAttribute::Noop];
        }
        Ok(Self(items))
    }
}

// Alphabetical order
#[derive(Eq, PartialEq, Debug, Clone)]
enum FrbAttribute {
    Dart2Rust(FrbAttributeSerDes),
    DartCode(FrbAttributeDartCode),
    Default(FrbAttributeDefaultValue),
    External,
    Getter,
    Ignore,
    Init,
    Mirror(FrbAttributeMirror),
    Name(FrbAttributeName),
    NonEq,
    NonFinal,
    NonHash,
    NonOpaque,
    Opaque,
    Positional,
    Proxy,
    Rust2Dart(FrbAttributeSerDes),
    Setter,
    Serialize,
    StreamDartAwait,
    Sync,
    DartAsync,
    Type64bitInt,

    // === Mainly undocumented since may subject to change ===

    // Maybe we can unify this with `DartCode`
    DartMetadata(NamedOption<frb_keyword::dart_metadata, FrbAttributeDartMetadata>),
    Noop,
    RustOpaqueCodecMoi,
    // NOTE: Undocumented, since this name may be suboptimal and is subject to change
    SemiSerialize,
    UiState,
    UiMutation,
}

impl Parse for FrbAttribute {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        use frb_keyword::*;
        use FrbAttribute::*;

        let lookahead = input.lookahead1();

        let keyword_output = parse_keyword::<non_final, _>(input, &lookahead, non_final, NonFinal)
            .or_else(|| parse_keyword::<sync, _>(input, &lookahead, sync, Sync))
            .or_else(|| parse_keyword::<dart_async, _>(input, &lookahead, dart_async, DartAsync))
            .or_else(|| {
                parse_keyword::<stream_dart_await, _>(
                    input,
                    &lookahead,
                    stream_dart_await,
                    StreamDartAwait,
                )
            })
            .or_else(|| parse_keyword::<getter, _>(input, &lookahead, getter, Getter))
            .or_else(|| parse_keyword::<setter, _>(input, &lookahead, setter, Setter))
            .or_else(|| parse_keyword::<init, _>(input, &lookahead, init, Init))
            .or_else(|| parse_keyword::<ignore, _>(input, &lookahead, ignore, Ignore))
            .or_else(|| parse_keyword::<opaque, _>(input, &lookahead, opaque, Opaque))
            .or_else(|| parse_keyword::<non_opaque, _>(input, &lookahead, non_opaque, NonOpaque))
            .or_else(|| parse_keyword::<non_hash, _>(input, &lookahead, non_hash, NonHash))
            .or_else(|| parse_keyword::<non_eq, _>(input, &lookahead, non_eq, NonEq))
            .or_else(|| parse_keyword::<positional, _>(input, &lookahead, positional, Positional))
            .or_else(|| parse_keyword::<proxy, _>(input, &lookahead, proxy, Proxy))
            .or_else(|| parse_keyword::<external, _>(input, &lookahead, external, External))
            .or_else(|| {
                parse_keyword::<type_64bit_int, _>(input, &lookahead, type_64bit_int, Type64bitInt)
            })
            // .or_else(|| {
            //     parse_keyword::<generate_implementor_enum, _>(
            //         input,
            //         &lookahead,
            //         generate_implementor_enum,
            //         GenerateImplEnum,
            //     )
            // })
            .or_else(|| {
                parse_keyword::<rust_opaque_codec_moi, _>(
                    input,
                    &lookahead,
                    rust_opaque_codec_moi,
                    RustOpaqueCodecMoi,
                )
            })
            .or_else(|| parse_keyword::<serialize, _>(input, &lookahead, serialize, Serialize))
            .or_else(|| {
                parse_keyword::<semi_serialize, _>(input, &lookahead, semi_serialize, SemiSerialize)
            })
            .or_else(|| parse_keyword::<ui_state, _>(input, &lookahead, ui_state, UiState))
            .or_else(|| {
                parse_keyword::<ui_mutation, _>(input, &lookahead, ui_mutation, UiMutation)
            });
        if let Some(keyword_output) = keyword_output {
            return keyword_output;
        }

        Ok(if lookahead.peek(frb_keyword::mirror) {
            input.parse::<frb_keyword::mirror>()?;
            input.parse().map(Mirror)?
        } else if lookahead.peek(frb_keyword::dart_metadata) {
            input.parse().map(DartMetadata)?
        } else if lookahead.peek(default) {
            input.parse::<default>()?;
            input.parse::<Token![=]>()?;
            input.parse().map(Default)?
        } else if lookahead.peek(dart_code) {
            input.parse::<dart_code>()?;
            input.parse::<Token![=]>()?;
            input.parse().map(DartCode)?
        } else if lookahead.peek(name) {
            input.parse::<name>()?;
            input.parse::<Token![=]>()?;
            input.parse().map(Name)?
        } else if lookahead.peek(frb_keyword::dart2rust) {
            input.parse::<frb_keyword::dart2rust>()?;
            input.parse().map(Dart2Rust)?
        } else if lookahead.peek(frb_keyword::rust2dart) {
            input.parse::<frb_keyword::rust2dart>()?;
            input.parse().map(Rust2Dart)?
        } else {
            return Err(lookahead.error());
        })
    }
}

fn parse_keyword<T: Parse, S: Peek>(
    input: ParseStream,
    lookahead: &Lookahead1,
    token: S,
    attribute: FrbAttribute,
) -> Option<Result<FrbAttribute>> {
    lookahead
        .peek(token)
        .then(|| input.parse::<T>().map(|_| attribute))
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct FrbAttributeMirror(Vec<Path>);

impl Parse for FrbAttributeMirror {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let paths = Punctuated::<Path, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(paths))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FrbAttributeDartMetadata(Vec<MirDartAnnotation>);

impl Parse for FrbAttributeDartMetadata {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let annotations = Punctuated::<MirDartAnnotation, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Self(annotations))
    }
}

// TODO unused, rm?
// #[derive(Clone, Debug)]
// struct DartImports(Vec<MirDartImport>);
//
// impl Parse for DartImports {
//     fn parse(input: ParseStream<'_>) -> Result<Self> {
//         let content;
//         parenthesized!(content in input);
//         let imports = Punctuated::<MirDartImport, Token![,]>::parse_terminated(&content)?
//             .into_iter()
//             .collect();
//         Ok(Self(imports))
//     }
// }

impl Parse for MirDartImport {
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

impl Parse for MirDartAnnotation {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let annotation: LitStr = input.parse()?;
        let library = if input.peek(frb_keyword::import) {
            let _ = input.parse::<frb_keyword::import>()?;
            let library: MirDartImport = input.parse()?;
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

#[derive(Clone, Serialize, Eq, PartialEq, Debug)]
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
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            Err(lh.error())
            // frb-coverage:ignore-end
        }
    }
}

impl FrbAttributeDefaultValue {
    fn to_mir_default_value(&self) -> MirDefaultValue {
        match self {
            Self::Str(lit) => MirDefaultValue::String {
                content: lit.value(),
            },

            // other types
            Self::Bool(lit) => MirDefaultValue::Others {
                dart_literal: (if lit.value { "true" } else { "false" }).to_owned(),
            },
            Self::Int(lit) => MirDefaultValue::Others {
                dart_literal: lit.base10_digits().into(),
            },
            Self::Float(lit) => MirDefaultValue::Others {
                dart_literal: lit.base10_digits().into(),
            },
            Self::Vec(lit) => MirDefaultValue::Others {
                dart_literal: format!(
                    "const [{}]",
                    lit.iter()
                        .map(|item| item.to_mir_default_value().to_dart_literal().to_string())
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

#[derive(Clone, Serialize, Eq, PartialEq, Debug)]
struct FrbAttributeDartCode(String);

impl Parse for FrbAttributeDartCode {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::LitStr>().map(|x| Self(x.value()))
    }
}

#[derive(Clone, Serialize, Eq, PartialEq, Debug)]
struct FrbAttributeName(String);

impl Parse for FrbAttributeName {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::LitStr>().map(|x| Self(x.value()))
    }
}

#[derive(Clone, Serialize, Eq, PartialEq, Debug)]
pub(crate) struct FrbAttributeSerDes {
    pub dart_type: String,
    pub dart_code: String,
}

impl Parse for FrbAttributeSerDes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);

        content.parse::<frb_keyword::dart_type>()?;
        content.parse::<Token![=]>()?;
        let dart_type = content.parse::<syn::LitStr>()?.value();

        content.parse::<Token![,]>()?;

        content.parse::<frb_keyword::dart_code>()?;
        content.parse::<Token![=]>()?;
        let dart_code = content.parse::<syn::LitStr>()?.value();

        Ok(Self {
            dart_type,
            dart_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::ir::mir::default::MirDefaultValue;
    use crate::codegen::parser::mir::parser::attribute::{
        FrbAttribute, FrbAttributeDartCode, FrbAttributeDefaultValue, FrbAttributeMirror,
        FrbAttributeName, FrbAttributeSerDes, FrbAttributes, NamedOption,
    };
    use crate::if_then_some;
    use quote::quote;
    use syn::ItemFn;

    #[test]
    fn test_error() -> anyhow::Result<()> {
        let result = parse("#[frb(what_is_this)]");
        assert!(result.err().is_some());
        Ok(())
    }

    #[test]
    fn test_double_colon() -> anyhow::Result<()> {
        let parsed = parse("#[flutter_rust_bridge::frb(sync)]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Sync]);
        Ok(())
    }

    #[test]
    fn test_multiple_via_comma() -> anyhow::Result<()> {
        let parsed = parse("#[frb(sync, non_final)]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Sync, FrbAttribute::NonFinal]);
        Ok(())
    }

    #[test]
    fn test_multiple_via_hash() -> anyhow::Result<()> {
        let parsed = parse("#[frb(sync)]\n#[frb(non_final)]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Sync, FrbAttribute::NonFinal]);
        Ok(())
    }

    #[test]
    fn test_empty() -> anyhow::Result<()> {
        let parsed = parse("#[frb]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Noop]);
        Ok(())
    }

    #[test]
    fn test_empty_bracket() -> anyhow::Result<()> {
        let parsed = parse("#[frb()]")?;
        assert_eq!(parsed.0, vec![FrbAttribute::Noop]);
        Ok(())
    }

    #[test]
    fn test_comments() -> anyhow::Result<()> {
        let actual = parse("/// flutter_rust_bridge:ignore\n")?;
        assert_eq!(actual, FrbAttributes(vec![FrbAttribute::Ignore]));
        Ok(())
    }

    #[test]
    fn test_unrelated_comments() -> anyhow::Result<()> {
        let actual = parse("/// whatever_comment\n")?;
        assert_eq!(actual, FrbAttributes(vec![]));
        Ok(())
    }

    #[test]
    fn test_mirror() -> anyhow::Result<()> {
        let parsed = parse("#[frb(mirror(Apple, Orange))]")?;
        let paths = if_then_some!(let FrbAttribute::Mirror(FrbAttributeMirror(paths)) = &parsed.0[0], paths);
        let path = &paths.unwrap()[0];
        assert_eq!(quote!(#path).to_string(), "Apple");
        Ok(())
    }

    fn simple_keyword_tester(keyword_name: &str, attribute: FrbAttribute) {
        let parsed = parse(&format!("#[frb({keyword_name})]")).unwrap();
        assert_eq!(parsed, FrbAttributes(vec![attribute]));
    }

    #[test]
    fn test_non_final() {
        simple_keyword_tester("non_final", FrbAttribute::NonFinal);
    }

    #[test]
    fn test_sync() {
        simple_keyword_tester("sync", FrbAttribute::Sync);
    }

    #[test]
    fn test_dart_async() {
        simple_keyword_tester("dart_async", FrbAttribute::DartAsync);
    }

    #[test]
    fn test_stream_dart_await() {
        simple_keyword_tester("stream_dart_await", FrbAttribute::StreamDartAwait);
    }

    #[test]
    fn test_getter() {
        simple_keyword_tester("getter", FrbAttribute::Getter);
    }

    #[test]
    fn test_setter() {
        simple_keyword_tester("setter", FrbAttribute::Setter);
    }

    #[test]
    fn test_init() {
        simple_keyword_tester("init", FrbAttribute::Init);
    }

    #[test]
    fn test_ignore() {
        simple_keyword_tester("ignore", FrbAttribute::Ignore);
    }

    #[test]
    fn test_opaque() {
        simple_keyword_tester("opaque", FrbAttribute::Opaque);
    }

    #[test]
    fn test_non_opaque() {
        simple_keyword_tester("non_opaque", FrbAttribute::NonOpaque);
    }

    #[test]
    fn test_non_hash() {
        simple_keyword_tester("non_hash", FrbAttribute::NonHash);
    }

    #[test]
    fn test_non_eq() {
        simple_keyword_tester("non_eq", FrbAttribute::NonEq);
    }

    #[test]
    fn test_positional() {
        simple_keyword_tester("positional", FrbAttribute::Positional);
    }

    #[test]
    fn test_proxy() {
        simple_keyword_tester("proxy", FrbAttribute::Proxy);
    }

    #[test]
    fn test_external() {
        simple_keyword_tester("external", FrbAttribute::External);
    }

    #[test]
    fn test_type_64bit_int() {
        simple_keyword_tester("type_64bit_int", FrbAttribute::Type64bitInt);
    }

    // #[test]
    // fn test_generate_implementor_enum() {
    //     simple_keyword_tester("generate_implementor_enum", FrbAttribute::GenerateImplEnum);
    // }

    #[test]
    fn test_rust_opaque_codec_moi() {
        simple_keyword_tester("rust_opaque_codec_moi", FrbAttribute::RustOpaqueCodecMoi);
    }

    #[test]
    fn test_dart_code() -> anyhow::Result<()> {
        let parsed = parse(r###"#[frb(dart_code="a\nb\nc")]"###)?;
        assert_eq!(
            parsed,
            FrbAttributes(vec![FrbAttribute::DartCode(FrbAttributeDartCode(
                "a\nb\nc".to_owned()
            ))])
        );
        Ok(())
    }

    #[test]
    fn test_name() -> anyhow::Result<()> {
        let parsed = parse(r###"#[frb(name="operator <")]"###)?;
        assert_eq!(
            parsed,
            FrbAttributes(vec![FrbAttribute::Name(FrbAttributeName(
                "operator <".to_owned()
            ))])
        );
        Ok(())
    }

    #[test]
    fn test_rust2dart() -> anyhow::Result<()> {
        let parsed =
            parse(r###"#[frb(rust2dart(dart_type = "my_type", dart_code = "my_code"))]"###)?;
        assert_eq!(
            parsed,
            FrbAttributes(vec![FrbAttribute::Rust2Dart(FrbAttributeSerDes {
                dart_type: "my_type".to_owned(),
                dart_code: "my_code".to_owned(),
            })])
        );
        Ok(())
    }

    #[test]
    fn test_dart2rust() -> anyhow::Result<()> {
        let parsed =
            parse(r###"#[frb(dart2rust(dart_type = "my_type", dart_code = "my_code"))]"###)?;
        assert_eq!(
            parsed,
            FrbAttributes(vec![FrbAttribute::Dart2Rust(FrbAttributeSerDes {
                dart_type: "my_type".to_owned(),
                dart_code: "my_code".to_owned(),
            })])
        );
        Ok(())
    }

    #[test]
    fn test_metadata() -> anyhow::Result<()> {
        let parsed = parse(
            r#"#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]"#,
        )?;
        let value = if_then_some!(let FrbAttribute::DartMetadata(NamedOption { value, .. }) = &parsed.0[0], value).unwrap();
        assert_eq!(value.0[0].content, "freezed");
        assert_eq!(value.0[1].content, "immutable");
        Ok(())
    }

    #[test]
    fn test_default() -> anyhow::Result<()> {
        let parsed = parse(r#"#[frb(default = "Weekdays.Sunday")]"#)?;
        let value = if_then_some!(let FrbAttribute::Default(value) = &parsed.0[0], value).unwrap();
        assert!(matches!(value, FrbAttributeDefaultValue::Str(_)));
        Ok(())
    }

    #[test]
    fn test_ui_state() {
        simple_keyword_tester("ui_state", FrbAttribute::UiState);
    }

    #[test]
    fn test_ui_mutation() {
        simple_keyword_tester("ui_mutation", FrbAttribute::UiMutation);
    }

    // Mirror(FrbAttributeMirror),
    // NonFinal,
    // Sync,
    // Metadata(NamedOption<frb_keyword::dart_metadata, FrbAttributeDartMetadata>),
    // Default(FrbAttributeDefaultValue),

    fn parse(raw: &str) -> anyhow::Result<FrbAttributes> {
        let code = raw.to_owned() + " fn f() {}";
        let fn_ast: ItemFn = syn::parse_str(&code)?;
        FrbAttributes::parse(&fn_ast.attrs)
    }

    #[test]
    fn test_frb_attribute_default_value() -> anyhow::Result<()> {
        for (text, expect_mir_default_value) in vec![
            (
                "\"Hello\"",
                MirDefaultValue::String {
                    content: "Hello".to_string(),
                },
            ),
            (
                "true",
                MirDefaultValue::Others {
                    dart_literal: "true".to_string(),
                },
            ),
            (
                "100",
                MirDefaultValue::Others {
                    dart_literal: "100".to_string(),
                },
            ),
            (
                "1.5",
                MirDefaultValue::Others {
                    dart_literal: "1.5".to_string(),
                },
            ),
            (
                "[100,200]",
                MirDefaultValue::Others {
                    dart_literal: "const [100,200]".to_string(),
                },
            ),
        ] {
            let value: FrbAttributeDefaultValue = syn::parse_str(text)?;
            assert_eq!(value.to_mir_default_value(), expect_mir_default_value);
            assert!(!serde_json::to_string(&value)?.is_empty());
        }
        Ok(())
    }
}
