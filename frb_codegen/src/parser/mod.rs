pub(crate) mod ty;

use std::collections::HashMap;
use std::string::String;

use log::debug;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::*;
use topological_sort::TopologicalSort;

use crate::ir::*;

use crate::generator::rust::HANDLER_NAME;
use crate::method_utils::FunctionName;
use crate::parser::ty::TypeParser;
use crate::source_graph::Crate;

use self::ty::convert_ident_str;

const STREAM_SINK_IDENT: &str = "StreamSink";
const RESULT_IDENT: &str = "Result";

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

pub fn parse(source_rust_content: &str, file: File, manifest_path: &str) -> IrFile {
    let crate_map = Crate::new(manifest_path);

    let mut src_fns = extract_fns_from_file(&file);
    src_fns.extend(extract_methods_from_file(&file));
    let src_structs = crate_map.root_module.collect_structs_to_vec();
    let src_enums = crate_map.root_module.collect_enums_to_vec();
    let src_types = crate_map.root_module.collect_types_to_pool();
    let src_types = topo_resolve(src_types);

    let parser = Parser::new(TypeParser::new(src_structs, src_enums, src_types));
    parser.parse(source_rust_content, src_fns)
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
    fn parse(mut self, source_rust_content: &str, src_fns: Vec<ItemFn>) -> IrFile {
        let funcs = src_fns.iter().map(|f| self.parse_function(f)).collect();

        let has_executor = source_rust_content.contains(HANDLER_NAME);

        let (struct_pool, enum_pool) = self.type_parser.consume();

        IrFile {
            funcs,
            struct_pool,
            enum_pool,
            has_executor,
        }
    }

    /// Attempts to parse the type from the return part of a function signature. There is a special
    /// case for top-level `Result` types.
    pub fn try_parse_fn_output_type(&mut self, ty: &syn::Type) -> Option<IrFuncOutput> {
        let ty = &self.type_parser.resolve_alias(ty);
        let inner = ty::SupportedInnerType::try_from_syn_type(ty)?;
        match inner {
            ty::SupportedInnerType::Path(ty::SupportedPathType {
                ident,
                generic: Some(generic),
            }) if ident == RESULT_IDENT => Some(IrFuncOutput::ResultType(
                self.type_parser.convert_to_ir_type(*generic)?,
            )),
            _ => Some(IrFuncOutput::Type(
                self.type_parser.convert_to_ir_type(inner)?,
            )),
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

    fn parse_function(&mut self, func: &ItemFn) -> IrFunc {
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
                    panic!("unexpected pat_type={:?}", pat_type)
                };
                match self.try_parse_fn_arg_type(&pat_type.ty).unwrap_or_else(|| {
                    panic!(
                        "Failed to parse function argument type `{}`",
                        type_to_string(&pat_type.ty)
                    )
                }) {
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
                        });
                    }
                }
            } else {
                panic!("unexpected sig_input={:?}", sig_input);
            }
        }

        if output.is_none() {
            output = Some(match &sig.output {
                ReturnType::Type(_, ty) => {
                    match self.try_parse_fn_output_type(ty).unwrap_or_else(|| {
                        panic!(
                            "Failed to parse function output type `{}`",
                            type_to_string(ty)
                        )
                    }) {
                        IrFuncOutput::ResultType(ty) => ty,
                        IrFuncOutput::Type(ty) => {
                            fallible = false;
                            ty
                        }
                    }
                }
                ReturnType::Default => {
                    fallible = false;
                    IrType::Primitive(IrTypePrimitive::Unit)
                }
            });
            mode = Some(if let Some(IrType::SyncReturn(_)) = output {
                IrFuncMode::Sync
            } else {
                IrFuncMode::Normal
            });
        }

        IrFunc {
            name: func_name,
            inputs,
            output: output.expect("unsupported output"),
            fallible,
            mode: mode.expect("missing mode"),
            comments: extract_comments(&func.attrs),
        }
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

fn extract_methods_from_file(file: &File) -> Vec<ItemFn> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Method(item_method) = item {
                    if let Visibility::Public(_) = &item_method.vis {
                        let f = item_method_to_function(item_impl, item_method)
                            .expect("item implementation is unsupported");
                        src_fns.push(f);
                    }
                }
            }
        }
    }

    src_fns
}

// Converts an item implementation (something like fn(&self, ...)) into a function where `&self` is a named parameter to `&Self`
fn item_method_to_function(item_impl: &ItemImpl, item_method: &ImplItemMethod) -> Option<ItemFn> {
    if let Type::Path(p) = item_impl.self_ty.as_ref() {
        let struct_name = p.path.segments.first().unwrap().ident.to_string();
        let span = item_method.sig.ident.span();
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
                    crate::method_utils::MethodInfo::Static {
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
                    crate::method_utils::MethodInfo::NonStatic {
                        struct_name: struct_name.clone(),
                    },
                )
                .serialize(),
                span,
            )
        };

        Some(ItemFn {
            attrs: vec![],
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
                inputs: item_method
                    .sig
                    .inputs
                    .iter()
                    .map(|input| {
                        if let FnArg::Receiver(Receiver { mutability, .. }) = input {
                            let mut segments = Punctuated::new();
                            segments.push(PathSegment {
                                ident: Ident::new(struct_name.as_str(), span),
                                arguments: PathArguments::None,
                            });
                            if mutability.is_some() {
                                panic!("mutable methods are unsupported for safety reasons");
                            }
                            FnArg::Typed(PatType {
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
                            })
                        } else {
                            input.clone()
                        }
                    })
                    .collect::<Punctuated<_, _>>(),
                variadic: None,
                output: item_method.sig.output.clone(),
            },
            block: Box::new(item_method.block.clone()),
        })
    } else {
        None
    }
}

fn extract_comments(attrs: &[Attribute]) -> Vec<IrComment> {
    attrs
        .iter()
        .filter_map(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            })) if path.is_ident("doc") => Some(IrComment::from(lit.value().as_ref())),
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
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name: K = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(Self { name, value })
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
impl Parse for MetadataAnnotations {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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
impl Parse for DartImports {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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
        } else {
            Err(lookahead.error())
        }
    }
}
fn extract_metadata(attrs: &[Attribute]) -> Vec<IrDartAnnotation> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("frb"))
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
