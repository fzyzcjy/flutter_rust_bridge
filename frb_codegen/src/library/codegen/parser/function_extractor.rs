use crate::if_then_some;
use itertools::Itertools;
use quote::__private::Span;
use std::path::PathBuf;
use syn::spanned::Spanned;
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
    TraitMethod {
        impl_item_fn: ImplItemFn,
    },
}

impl GeneralizedItemFn {
    pub(crate) fn sig(&self) -> &Signature {
        match self {
            GeneralizedItemFn::Function { item_fn } => &item_fn.sig,
            GeneralizedItemFn::Method { impl_item_fn, .. } => &impl_item_fn.sig,
            GeneralizedItemFn::TraitMethod { impl_item_fn } => &impl_item_fn.sig,
        }
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            GeneralizedItemFn::Function { item_fn } => &item_fn.attrs,
            GeneralizedItemFn::Method { impl_item_fn, .. } => &impl_item_fn.attrs,
            GeneralizedItemFn::TraitMethod { impl_item_fn } => &impl_item_fn.attrs,
        }
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            GeneralizedItemFn::Function { item_fn } => item_fn.span(),
            GeneralizedItemFn::Method { impl_item_fn, .. } => impl_item_fn.span(),
            GeneralizedItemFn::TraitMethod { impl_item_fn } => impl_item_fn.span(),
        }
    }
}

pub(super) fn extract_generalized_functions_from_file(
    file: &File,
    path: &std::path::Path,
) -> anyhow::Result<Vec<PathAndItemFn>> {
    let item_fns = [
        extract_fns_from_file(file),
        extract_methods_from_file(file)?,
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
                    } else if let Visibility::Inherited = &impl_item_fn.vis {
                        // `ìmpl ... for ...` blocks are also parsed as
                        // inherited. We need to do further checks to ensure no
                        // private function is extracted.
                        if let Some(_) = &item_impl.trait_ {
                            src_fns.push(GeneralizedItemFn::TraitMethod {
                                impl_item_fn: impl_item_fn.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(src_fns)
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_methods_from_impl_for() {
        let data = syn::parse_str::<syn::File>(
            "pub struct A {
                val: u32
            }
            impl Default for A {
                fn default() -> Self {
                    A {val: 0}
                }
            }",
        )
            .unwrap();

        use crate::codegen::parser::function_extractor::extract_methods_from_file;
        let methods = extract_methods_from_file(&data);
        assert!(methods.is_ok());
        assert_eq!(methods.unwrap().len(), 1);
    }
    #[test]
    fn dont_extract_private_restricted_methods() {
        let data = syn::parse_str::<syn::File>(
            "pub struct A {}

            impl A {
                fn private() {}
                pub(crate) fn restricted() {}
                pub fn public() {}
            }",
        )
            .unwrap();

        use crate::codegen::parser::function_extractor::extract_methods_from_file;
        let methods = extract_methods_from_file(&data);
        assert!(methods.is_ok());
        assert_eq!(methods.unwrap().len(), 1);
    }
}
