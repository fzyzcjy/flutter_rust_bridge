use crate::if_then_some;
use anyhow::Context;
use itertools::Itertools;
use std::path::PathBuf;
use syn::File;
use syn::*;

pub(super) struct PathAndItemFn {
    pub(super) path: PathBuf,
    pub(super) generalized_item_fn: GeneralizedItemFn,
}

#[derive(Debug, Clone)]
pub(crate) enum GeneralizedItemFn {
    Function {
        item_fn: ItemFn,
    },
    Method {
        item_impl: ItemImpl,
        impl_item_fn: ImplItemFn,
    },
}

impl GeneralizedItemFn {
    pub(crate) fn sig(&self) -> &Signature {
        match self {
            GeneralizedItemFn::Function { item_fn } => &item_fn.sig,
            GeneralizedItemFn::Method { impl_item_fn, .. } => &impl_item_fn.sig,
        }
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            GeneralizedItemFn::Function { item_fn } => &item_fn.attrs,
            GeneralizedItemFn::Method { impl_item_fn, .. } => &impl_item_fn.attrs,
        }
    }
}

pub(super) fn extract_generalized_functions_from_file(
    file: &File,
    path: &std::path::Path,
) -> anyhow::Result<Vec<PathAndItemFn>> {
    let item_fns = [
        extract_fns_from_file(&file),
        extract_methods_from_file(&file)?,
    ]
    .concat();
    let ans = item_fns
        .into_iter()
        .map(|generalized_item_fn| PathAndItemFn {
            path: path.to_owned(),
            generalized_item_fn,
        })
        .collect_vec();
    Ok(ans)
}

fn extract_fns_from_file(file: &File) -> Vec<GeneralizedItemFn> {
    file.items
        .iter()
        .filter_map(|item| if_then_some!(let Item::Fn(ref item_fn) = item, item_fn))
        .filter(|item_fn| matches!(item_fn.vis, Visibility::Public(_)))
        .cloned()
        .map(|item_fn| GeneralizedItemFn::Function { item_fn })
        .collect_vec()
}

fn extract_methods_from_file(file: &File) -> anyhow::Result<Vec<GeneralizedItemFn>> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Fn(impl_item_fn) = item {
                    if let Visibility::Public(_) = &impl_item_fn.vis {
                        src_fns.push(GeneralizedItemFn::Method {
                            item_impl: item_impl.clone(),
                            impl_item_fn: impl_item_fn.clone(),
                        });
                    }
                }
            }
        }
    }

    Ok(src_fns)
}

// TODO temp put it here to copy useful parts
fn convert_item_method_to_function(
    _item_impl: &ItemImpl,
    _item_method: &ImplItemFn,
) -> anyhow::Result<Option<ItemFn>> {
    if let Type::Path(p) = item_impl.self_ty.as_ref() {
        let struct_name = p.path.segments.first().unwrap().ident.to_string();
        let span = item_method.sig.ident.span();
        let is_static_method = {
            let Signature { inputs, .. } = &item_method.sig;
            !matches!(inputs.first(), Some(FnArg::Receiver(..)))
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
                    crate::utils::method::MethodInfo::NonStatic {
                        struct_name: struct_name.clone(),
                    },
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
                inputs: item_method
                    .sig
                    .inputs
                    .iter()
                    .map(|input| -> anyhow::Result<_> {
                        if let FnArg::Receiver(Receiver { mutability, .. }) = input {
                            let mut segments = Punctuated::new();
                            segments.push(PathSegment {
                                ident: Ident::new(struct_name.as_str(), span),
                                arguments: PathArguments::None,
                            });
                            if mutability.is_some() {
                                return Err(super::error::Error::NoMutSelf);
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
                    .collect::<anyhow::Result<Punctuated<_, _>>>()?,
                variadic: None,
                output: item_method.sig.output.clone(),
            },
            block: Box::new(item_method.block.clone()),
        }))
    } else {
        Ok(None)
    }
}
