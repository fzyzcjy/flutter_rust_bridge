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
use syn::token::{Colon, Comma};
use syn::*;
use topological_sort::TopologicalSort;

use crate::ir::*;
use crate::utils::misc::{filter_type_content, read_rust_file, BlockIndex};

use crate::generator::rust::HANDLER_NAME;
use crate::parser::source_graph::Crate;
use crate::parser::ty::TypeParser;
use crate::parser::IrType::{EnumRef, StructRef};
use crate::utils::method::FunctionName;

use self::ty::convert_ident_str;

const STREAM_SINK_IDENT: &str = "StreamSink";

mod error;
pub use error::Error;
pub(crate) type ParserResult<T = (), E = Error> = core::result::Result<T, E>;

pub(crate) fn topo_resolve(src: HashMap<String, Type>) -> HashMap<String, Type> {
    // Some types that cannot be Handled.
    // Filter something like `BareFn( TypeBareFn...`
    // Filter something like `Ptr( TypePtr { star_token: Star,`
    let mut ret: HashMap<String, Type> = src
        .iter()
        .filter_map(|(k, v)| match convert_ident_str(v) {
            Some(_) => None,
            None => Some((k.to_owned(), v.to_owned())),
        })
        .collect();

    let string_src = src
        .iter()
        // Filter some types that cannot be Handled.
        .filter_map(|(k, v)| convert_ident_str(v).map(|v| (k, v)));

    let mut ts = TopologicalSort::<String>::new();

    string_src.for_each(|(k, v)| {
        // k and v switch orders here
        ts.add_dependency(v, k);
    });

    // remove base type, like i32 in `type Id = i32`.
    // You might worry about the type, which cannot be handled and isn't in ts.
    // Case:
    // ```
    // type UnsafeAlias = unsafe{};
    // type NestAlias = UnsafeAlias;
    // ```
    // 1. pop_base: ret = [{UnsafeAlias, unsafe}]
    // 2. init_condition:
    //    2.1. ("UnsafeAlias","NestAlias") in ts,
    //    2.2. do pop_all, pop UnsafeAlias, only "NestAlias" in ts.
    //    2.3. do pop, and then handle NestAlias.
    //    src.get("NestAlias") => UnsafeAlias,
    //    ret.insert("NestAlias") = ret.get("UnsafeAlias")
    ts.pop_all();
    // build init_condition
    ts.pop_all().into_iter().for_each(|k| {
        let v_src = src.get(&k).unwrap().to_owned();
        let v_str = convert_ident_str(&v_src).unwrap();

        ret.insert(
            k,
            if ret.contains_key(&v_str) {
                // only happen if v_src cannot handle
                ret.get(&v_str).unwrap().clone()
            } else {
                v_src
            },
        );
    });

    while let Some(k) = ts.pop() {
        let v_src = src.get(&k).unwrap().to_owned();
        let v_str = convert_ident_str(&v_src).unwrap();
        let v_ret = ret
            .get(&v_str)
            .unwrap_or_else(|| panic!("{:?},\n{:?},\n{:?},\n{}", src, ts, ret, k))
            .to_owned();
        ret.insert(k, v_ret);
    }
    ret
}

/// Read a single rust file, and parse it into an IrFile.
pub fn parse_a_rust_file(
    manifest_path: &str,
    rust_file_path: &str,
    block_index: BlockIndex,
    only_parse_these_types_names: Option<&[&str]>,
) -> ParserResult<IrFile> {
    log::debug!("the rust path is:{rust_file_path}"); // TODO: delete
    let mut source_rust_content = read_rust_file(rust_file_path);
    if let Some(types_names) = only_parse_these_types_names {
        source_rust_content = filter_type_content(&source_rust_content, types_names);
    };
    let file_ast = syn::parse_file(&source_rust_content).unwrap();

    let mut src_fns = extract_fns_from_file(&file_ast);
    src_fns.extend(extract_methods_from_file(&file_ast)?);
    let crate_map = Crate::new(manifest_path)?;
    let src_structs = crate_map.root_module.collect_structs_to_vec();
    let src_enums = crate_map.root_module.collect_enums_to_vec();
    let src_types = crate_map.root_module.collect_types_to_pool();
    let src_types = topo_resolve(src_types);

    // TODO: what is inside `src_types`,
    // TODO: what if directly use `src_enums` and `src_structs` inside `parser.parse`,
    let parser = Parser::new(TypeParser::new(src_structs, src_enums, src_types));
    parser.parse(
        manifest_path,
        rust_file_path,
        &source_rust_content,
        src_fns,
        block_index,
    )
}

struct Parser<'a> {
    type_parser: TypeParser<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(type_parser: TypeParser<'a>) -> Self {
        Parser { type_parser }
    }
}

impl<'a> Parser<'a> {
    fn parse(
        mut self,
        manifest_path: &str,
        rust_file_path: &str,
        source_rust_content: &str,
        src_fns: Vec<ItemFn>,
        block_index: BlockIndex,
    ) -> ParserResult<IrFile> {
        let funcs = src_fns
            .iter()
            .map(|f| self.parse_function(f))
            .collect::<ParserResult<Vec<_>>>()?;
        let has_executor = source_rust_content.contains(HANDLER_NAME);
        // TODO: does the 2 contains fields of struct/enum? if fields are also struct/enum, should we add them?
        // TODO: what if also `consume` type_parser.types?
        let (struct_pool, enum_pool) = self.type_parser.consume(); // TODO: delete
        Ok(IrFile::new(
            manifest_path,
            rust_file_path,
            funcs,
            None,
            struct_pool,
            enum_pool,
            has_executor,
            block_index,
        ))
    }

    /// Attempts to parse the type from the return part of a function signature. There is a special
    /// case for top-level `Result` types.
    pub fn try_parse_fn_output_type(&mut self, ty: &syn::Type) -> Option<IrFuncOutput> {
        let ty = &self.type_parser.resolve_alias(ty).clone();

        if let Type::Path(type_path) = ty {
            match self.type_parser.convert_path_to_ir_type(type_path) {
                Ok(IrType::Unencodable(IrTypeUnencodable { segments, .. })) => {
                    match if cfg!(feature = "qualified_names") {
                        segments.splay()
                    } else {
                        // Emulate old behavior by discarding any name qualifiers
                        vec![segments.splay().pop().unwrap()]
                    }[..]
                    {
                        #[cfg(feature = "qualified_names")]
                        [("anyhow", None), ("Result", Some(ArgsRefs::Generic([args])))] => {
                            Some(IrFuncOutput::ResultType {
                                ok: args.clone(),
                                error: None,
                            })
                        }
                        [("Result", Some(ArgsRefs::Generic(args)))] => {
                            let ok = args.first().unwrap();

                            let is_anyhow = args.len() == 1
                                || args.iter().any(|x| match x {
                                    IrType::Unencodable(IrTypeUnencodable { string, .. }) => {
                                        string == "anyhow :: Error"
                                    }
                                    _ => false,
                                });
                            let error = if is_anyhow {
                                Some(IrType::Delegate(IrTypeDelegate::Anyhow))
                            } else {
                                args.last().cloned()
                            };

                            let error = if let Some(StructRef(mut struct_ref)) = error {
                                struct_ref.is_exception = true;
                                Some(StructRef(struct_ref))
                            } else if let Some(EnumRef(mut enum_ref)) = error {
                                enum_ref.is_exception = true;
                                Some(EnumRef(enum_ref))
                            } else {
                                error
                            };

                            Some(IrFuncOutput::ResultType {
                                ok: ok.clone(),
                                error,
                            })
                        }
                        _ => None, // unencodable types not implemented
                    }
                }
                Ok(result) => Some(IrFuncOutput::Type(result)),
                Err(..) => None,
            }
        } else {
            let ir_ty = self.type_parser.parse_type(ty);
            Some(IrFuncOutput::Type(ir_ty))
        }
    }

    /// Attempts to parse the type from an argument of a function signature. There is a special
    /// case for top-level `StreamSink` types.
    pub fn try_parse_fn_arg_type(&mut self, ty: &syn::Type) -> Option<IrFuncArg> {
        match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let last_segment = path.segments.last().unwrap();
                if last_segment.ident == STREAM_SINK_IDENT {
                    match &last_segment.arguments {
                        syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments { args, .. },
                        ) if args.len() == 1 => {
                            // Unwrap is safe here because args.len() == 1
                            match args.last().unwrap() {
                                syn::GenericArgument::Type(t) => {
                                    Some(IrFuncArg::StreamSinkType(self.type_parser.parse_type(t)))
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                } else {
                    Some(IrFuncArg::Type(self.type_parser.parse_type(ty)))
                }
            }
            syn::Type::Array(_) => Some(IrFuncArg::Type(self.type_parser.parse_type(ty))),
            _ => None,
        }
    }

    fn parse_function(&mut self, func: &ItemFn) -> ParserResult<IrFunc> {
        debug!("parse_function function name: {:?}", func.sig.ident);

        let sig = &func.sig;
        let func_name = sig.ident.to_string();

        let mut inputs = Vec::new();
        let mut output = None;
        let mut mode: Option<IrFuncMode> = None;
        let mut fallible = true;

        for (i, sig_input) in sig.inputs.iter().enumerate() {
            if let FnArg::Typed(ref pat_type) = sig_input {
                let name = if let Pat::Ident(ref pat_ident) = *pat_type.pat {
                    format!("{}", pat_ident.ident)
                } else {
                    return Err(Error::UnexpectedPattern(
                        quote::quote!(#pat_type).to_string().into(),
                    ));
                };
                let arg_type = self.try_parse_fn_arg_type(&pat_type.ty).with_context(|| {
                    format!(
                        "Failed to parse function argument type `{}` in function `{}`",
                        type_to_string(&pat_type.ty),
                        func_name
                    )
                })?;
                match arg_type {
                    IrFuncArg::StreamSinkType(ty) => {
                        output = Some(ty);
                        mode = Some(IrFuncMode::Stream { argument_index: i });
                        fallible = match &sig.output {
                            ReturnType::Default => false,
                            ReturnType::Type(_, ty) => !matches!(
                                self.try_parse_fn_output_type(ty),
                                Some(IrFuncOutput::Type(_))
                            ),
                        }
                    }
                    IrFuncArg::Type(ty) => {
                        inputs.push(IrField {
                            name: IrIdent::new(name),
                            ty,
                            is_final: true,
                            comments: extract_comments(&pat_type.attrs),
                            default: DefaultValues::extract(&pat_type.attrs),
                            settings: IrFieldSettings::default(),
                        });
                    }
                }
            } else {
                return Err(Error::UnexpectedSigInput(
                    quote::quote!(#sig_input).to_string().into(),
                ));
            }
        }

        let result = match &sig.output {
            ReturnType::Type(_, ty) => {
                let output_type = self.try_parse_fn_output_type(ty).with_context(|| {
                    format!(
                        "Failed to parse function output type `{}` in function `{}`",
                        type_to_string(ty),
                        func_name
                    )
                })?;
                match output_type {
                    IrFuncOutput::ResultType { ok: ty, error: err } => (ty, err),
                    IrFuncOutput::Type(ty) => {
                        fallible = false;
                        (ty, None)
                    }
                }
            }
            ReturnType::Default => {
                fallible = false;
                (IrType::Primitive(IrTypePrimitive::Unit), None)
            }
        };

        if matches!(mode, Some(IrFuncMode::Stream { argument_index: _ }) if result.0 != IrType::Primitive(IrTypePrimitive::Unit))
        {
            return Err(Error::NoStreamSinkAndOutput(func_name.into()));
        }

        if output.is_none() {
            mode = Some(if let IrType::SyncReturn(_) = result.0 {
                IrFuncMode::Sync
            } else {
                IrFuncMode::Normal
            });
            output = Some(result.0);
        }

        Ok(IrFunc {
            name: func_name,
            inputs,
            output: output.context("Unsupported output")?,
            fallible,
            mode: mode.context("Missing mode")?,
            comments: extract_comments(&func.attrs),
            error_output: result.1,
        })
    }
}

fn extract_fns_from_file(file: &File) -> Vec<ItemFn> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        if let Item::Fn(ref item_fn) = item {
            if let Visibility::Public(_) = &item_fn.vis {
                src_fns.push(item_fn.clone());
            }
        }
    }

    src_fns
}

fn extract_methods_from_file(file: &File) -> ParserResult<Vec<ItemFn>> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Fn(item_method) = item {
                    if let Visibility::Public(_) = &item_method.vis {
                        let f = item_method_to_function(item_impl, item_method);
                        if f.is_err() {
                            log::warn!(
                                "For `{:?}`, the item implementation is unsupported",
                                item_method
                            );
                            continue; // continue, don't break
                        }

                        src_fns.push(f.unwrap());
                    }
                }
            }
        }
    }

    Ok(src_fns.into_iter().flatten().collect())
}

// Converts an item implementation (something like fn(&self, ...)) into a function where `&self` is a named parameter to `&Self`
fn item_method_to_function(
    item_impl: &ItemImpl,
    item_method: &ImplItemFn,
) -> ParserResult<Option<ItemFn>> {
    if let Type::Path(p) = item_impl.self_ty.as_ref() {
        let struct_name = p.path.segments.first().unwrap().ident.to_string();
        let span = item_method.sig.ident.span();

        // get/check inputs for mutability first
        let mut is_mut = false;
        let inputs: Punctuated<FnArg, Comma> = item_method
            .sig
            .inputs
            .iter()
            .enumerate()
            .map(|(i, input)| -> ParserResult<_> {
                let mut segments = Punctuated::new();
                segments.push(PathSegment {
                    ident: Ident::new(struct_name.as_str(), span),
                    arguments: PathArguments::None,
                });
                if let FnArg::Receiver(Receiver { mutability, .. }) = input {
                    if i == 0 && mutability.is_some() {
                        is_mut = true;
                    }
                    Ok(FnArg::Typed(PatType {
                        attrs: vec![],
                        pat: Box::new(Pat::Ident(PatIdent {
                            attrs: vec![],
                            by_ref: Some(syn::token::Ref { span }),
                            mutability: *mutability,
                            ident: Ident::new("that", span),
                            subpat: None,
                        })),
                        colon_token: Colon { spans: [span] },
                        ty: Box::new(Type::Path(TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments,
                            },
                        })),
                    }))
                } else {
                    Ok(input.clone())
                }
            })
            .collect::<ParserResult<Punctuated<_, _>>>()?;

        if is_mut {
            log::warn!(
                "Mutable method detected: `{}::{}`, which won't be generated",
                struct_name,
                item_method.sig.ident
            );
            return Ok(None);
        }

        let is_static_method = {
            let Signature { inputs, .. } = &item_method.sig;
            {
                !matches!(inputs.first(), Some(FnArg::Receiver(..)))
            }
        };
        let method_name = if is_static_method {
            let self_type = {
                let ItemImpl { self_ty, .. } = item_impl;
                if let Type::Path(TypePath { qself: _, path }) = &**self_ty {
                    if let Some(PathSegment {
                        ident,
                        arguments: _,
                    }) = path.segments.first()
                    {
                        Some(ident.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            Ident::new(
                &FunctionName::new(
                    &item_method.sig.ident.to_string(),
                    crate::utils::method::MethodInfo::Static {
                        struct_name: self_type.unwrap(),
                    },
                )
                .serialize(),
                span,
            )
        } else {
            Ident::new(
                &FunctionName::new(
                    &item_method.sig.ident.to_string(),
                    crate::utils::method::MethodInfo::NonStatic { struct_name },
                )
                .serialize(),
                span,
            )
        };

        Ok(Some(ItemFn {
            attrs: item_method.attrs.clone(),
            vis: item_method.vis.clone(),
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: item_method.sig.fn_token,
                ident: method_name,
                generics: item_method.sig.generics.clone(),
                paren_token: item_method.sig.paren_token,
                inputs,
                variadic: None,
                output: item_method.sig.output.clone(),
            },
            block: Box::new(item_method.block.clone()),
        }))
    } else {
        Ok(None)
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
fn extract_metadata(attrs: &[Attribute]) -> Vec<IrDartAnnotation> {
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

crate::ir! {
pub enum DefaultValues {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_litstr"))]
    Str(syn::LitStr),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_litbool"))]
    Bool(syn::LitBool),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_litint"))]
    Int(syn::LitInt),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_litfloat"))]
    Float(syn::LitFloat),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_punctuated"))]
    Vec(Punctuated<DefaultValues, Token![,]>),
}
}

#[cfg(feature = "serde")]
use _serde::*;

#[cfg(feature = "serde")]
mod _serde {
    use serde::{Serialize, Serializer};
    use syn::{punctuated::Punctuated, Token};

    use super::DefaultValues;

    pub fn serialize_litstr<S: Serializer>(lit: &syn::LitStr, s: S) -> Result<S::Ok, S::Error> {
        lit.value().serialize(s)
    }
    pub fn serialize_litbool<S: Serializer>(lit: &syn::LitBool, s: S) -> Result<S::Ok, S::Error> {
        lit.value().serialize(s)
    }
    pub fn serialize_litint<S: Serializer>(lit: &syn::LitInt, s: S) -> Result<S::Ok, S::Error> {
        lit.base10_parse::<i64>().unwrap().serialize(s)
    }
    pub fn serialize_litfloat<S: Serializer>(lit: &syn::LitFloat, s: S) -> Result<S::Ok, S::Error> {
        lit.base10_parse::<f64>().unwrap().serialize(s)
    }
    pub fn serialize_punctuated<S: Serializer>(
        lit: &Punctuated<DefaultValues, Token![,]>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        lit.into_iter().collect::<Vec<_>>().serialize(s)
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use syn::{parse_str, Type};

    use crate::parser::topo_resolve;

    #[test]
    fn test_topo_resolve_primary_type_with_nest() {
        let input = HashMap::from([
            ("id".to_string(), parse_str::<Type>("i32").unwrap()),
            ("UserId".to_string(), parse_str::<Type>("id").unwrap()),
        ]);
        let expect = HashMap::from([
            ("id".to_string(), parse_str::<Type>("i32").unwrap()),
            ("UserId".to_string(), parse_str::<Type>("i32").unwrap()),
        ]);
        let output = topo_resolve(input);
        assert_eq!(output, expect);
    }
    #[test]
    fn test_topo_resolve_primary_type() {
        let input = HashMap::from([("id".to_string(), parse_str::<Type>("i32").unwrap())]);
        let expect = HashMap::from([("id".to_string(), parse_str::<Type>("i32").unwrap())]);

        let output = topo_resolve(input);
        assert_eq!(output, expect);
    }
    #[test]
    fn test_topo_resolve3_unhandle_case() {
        let input = HashMap::from([("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap())]);
        let expect = HashMap::from([("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap())]);
        let output = topo_resolve(input);
        assert_eq!(output, expect);
    }
    #[test]
    fn test_topo_resolve_unhandle_case_with_nest() {
        let input = HashMap::from([
            ("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap()),
            (
                "DartPostCObjectFnTypeAlias".to_string(),
                parse_str::<Type>("DartPostCObjectFnType").unwrap(),
            )
        ]);
        let expect = HashMap::from([
            ("DartPostCObjectFnType".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap()),
            ("DartPostCObjectFnTypeAlias".to_string(), parse_str::<Type>(r#"unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool"#).unwrap()),
            ]);
        let output = topo_resolve(input);
        assert_eq!(&output, &expect);
    }
}
