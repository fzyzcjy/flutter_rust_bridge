use crate::codegen::parser::ParserResult;
use crate::if_then_some;
use anyhow::Context;
use itertools::Itertools;
use syn::File;
use syn::*;

pub(crate) fn extract_generalized_functions_from_file(file: &File) -> ParserResult<Vec<ItemFn>> {
    let mut ans = extract_fns_from_file(&file);
    ans.extend(extract_methods_from_file(&file)?);
    Ok(ans)
}

fn extract_fns_from_file(file: &File) -> Vec<ItemFn> {
    file.items
        .iter()
        .filter_map(|item| if_then_some!(let Item::Fn(ref item_fn) = item, item_fn))
        .filter(|item_fn| matches!(item_fn.vis, Visibility::Public(_)))
        .cloned()
        .collect_vec()
}

fn extract_methods_from_file(file: &File) -> ParserResult<Vec<ItemFn>> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        if let Item::Impl(ref item_impl) = item {
            for item in &item_impl.items {
                if let ImplItem::Fn(item_method) = item {
                    if let Visibility::Public(_) = &item_method.vis {
                        let f = convert_item_method_to_function(item_impl, item_method)?
                            .context("Unsupported item implementation")?;
                        src_fns.push(f);
                    }
                }
            }
        }
    }

    Ok(src_fns)
}

/// Converts an item implementation (something like fn(&self, ...)) into a function
/// where `&self` is a named parameter to `&Self`
fn convert_item_method_to_function(
    item_impl: &ItemImpl,
    item_method: &ImplItemFn,
) -> ParserResult<Option<ItemFn>> {
    // TODO want to use other approaches, temp disable
    todo!()
    // if let Type::Path(p) = item_impl.self_ty.as_ref() {
    //     let struct_name = p.path.segments.first().unwrap().ident.to_string();
    //     let span = item_method.sig.ident.span();
    //     let is_static_method = {
    //         let Signature { inputs, .. } = &item_method.sig;
    //         !matches!(inputs.first(), Some(FnArg::Receiver(..)))
    //     };
    //     let method_name = if is_static_method {
    //         let self_type = {
    //             let ItemImpl { self_ty, .. } = item_impl;
    //             if let Type::Path(TypePath { qself: _, path }) = &**self_ty {
    //                 if let Some(PathSegment {
    //                     ident,
    //                     arguments: _,
    //                 }) = path.segments.first()
    //                 {
    //                     Some(ident.to_string())
    //                 } else {
    //                     None
    //                 }
    //             } else {
    //                 None
    //             }
    //         };
    //         Ident::new(
    //             &FunctionName::new(
    //                 &item_method.sig.ident.to_string(),
    //                 crate::utils::method::MethodInfo::Static {
    //                     struct_name: self_type.unwrap(),
    //                 },
    //             )
    //             .serialize(),
    //             span,
    //         )
    //     } else {
    //         Ident::new(
    //             &FunctionName::new(
    //                 &item_method.sig.ident.to_string(),
    //                 crate::utils::method::MethodInfo::NonStatic {
    //                     struct_name: struct_name.clone(),
    //                 },
    //             )
    //             .serialize(),
    //             span,
    //         )
    //     };
    //
    //     Ok(Some(ItemFn {
    //         attrs: item_method.attrs.clone(),
    //         vis: item_method.vis.clone(),
    //         sig: Signature {
    //             constness: None,
    //             asyncness: None,
    //             unsafety: None,
    //             abi: None,
    //             fn_token: item_method.sig.fn_token,
    //             ident: method_name,
    //             generics: item_method.sig.generics.clone(),
    //             paren_token: item_method.sig.paren_token,
    //             inputs: item_method
    //                 .sig
    //                 .inputs
    //                 .iter()
    //                 .map(|input| -> ParserResult<_> {
    //                     if let FnArg::Receiver(Receiver { mutability, .. }) = input {
    //                         let mut segments = Punctuated::new();
    //                         segments.push(PathSegment {
    //                             ident: Ident::new(struct_name.as_str(), span),
    //                             arguments: PathArguments::None,
    //                         });
    //                         if mutability.is_some() {
    //                             return Err(super::error::Error::NoMutSelf);
    //                         }
    //                         Ok(FnArg::Typed(PatType {
    //                             attrs: vec![],
    //                             pat: Box::new(Pat::Ident(PatIdent {
    //                                 attrs: vec![],
    //                                 by_ref: Some(syn::token::Ref { span }),
    //                                 mutability: *mutability,
    //                                 ident: Ident::new("that", span),
    //                                 subpat: None,
    //                             })),
    //                             colon_token: Colon { spans: [span] },
    //                             ty: Box::new(Type::Path(TypePath {
    //                                 qself: None,
    //                                 path: Path {
    //                                     leading_colon: None,
    //                                     segments,
    //                                 },
    //                             })),
    //                         }))
    //                     } else {
    //                         Ok(input.clone())
    //                     }
    //                 })
    //                 .collect::<ParserResult<Punctuated<_, _>>>()?,
    //             variadic: None,
    //             output: item_method.sig.output.clone(),
    //         },
    //         block: Box::new(item_method.block.clone()),
    //     }))
    // } else {
    //     Ok(None)
    // }
}
