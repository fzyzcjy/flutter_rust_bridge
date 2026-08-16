use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use crate::if_then_some;
use itertools::Itertools;
use std::collections::HashMap;
use syn::visit_mut::VisitMut;
use syn::{Attribute, ImplItem, ImplItemFn, ItemImpl, PathArguments, Type, TypePath};

pub(crate) fn parse_syn_item_impl(
    target: &mut HirFlatPack,
    item_impl: ItemImpl,
    meta: &HirNaiveFlatItemMeta,
) {
    let trait_name = parse_trait_name(&item_impl);

    if let Some(trait_name) = &trait_name {
        (target.trait_impls).push(parse_trait_impl(&item_impl, trait_name));
    }
    (target.functions).extend(parse_functions(item_impl, meta, &trait_name));
}

fn parse_trait_name(item_impl: &ItemImpl) -> Option<String> {
    (item_impl.trait_.as_ref()).map(|t| t.1.segments.last().unwrap().ident.to_string())
}

fn parse_functions(
    item_impl: ItemImpl,
    meta: &HirNaiveFlatItemMeta,
    trait_def_name: &Option<String>,
) -> Vec<HirFlatFunction> {
    let attrs_item_impl = item_impl.attrs;
    let associated_types = parse_associated_types(&item_impl.items);

    (item_impl.items.into_iter())
        .filter_map(|item| if_then_some!(let ImplItem::Fn(impl_item_fn) = item, impl_item_fn))
        .map(|mut impl_item_fn| {
            AssociatedTypeSubstitutor {
                associated_types: &associated_types,
            }
            .visit_signature_mut(&mut impl_item_fn.sig);
            HirFlatFunction {
                namespace: meta.namespace.clone(),
                owner: HirFlatFunctionOwner::StructOrEnum {
                    impl_ty: *item_impl.self_ty.clone(),
                    trait_def_name: trait_def_name.clone(),
                },
                item_fn: GeneralizedItemFn::ImplItemFn(add_attrs(impl_item_fn, &attrs_item_impl)),
                sources: meta.sources.clone(),
            }
        })
        .collect_vec()
}

fn parse_associated_types(items: &[ImplItem]) -> HashMap<String, Type> {
    items
        .iter()
        .filter_map(|item| {
            if let ImplItem::Type(item_type) = item {
                Some((item_type.ident.to_string(), item_type.ty.clone()))
            } else {
                None
            }
        })
        .collect()
}

struct AssociatedTypeSubstitutor<'a> {
    associated_types: &'a HashMap<String, Type>,
}

impl VisitMut for AssociatedTypeSubstitutor<'_> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        if let Type::Path(type_path) = node {
            if let Some(name) = self_associated_type_name(type_path) {
                if let Some(replacement) = self.associated_types.get(&name) {
                    *node = replacement.clone();
                    return;
                }
            }
        }

        syn::visit_mut::visit_type_mut(self, node);
    }
}

fn self_associated_type_name(type_path: &TypePath) -> Option<String> {
    if type_path.qself.is_none()
        && type_path.path.segments.len() == 2
        && type_path.path.segments[0].ident == "Self"
        && matches!(type_path.path.segments[1].arguments, PathArguments::None)
    {
        return Some(type_path.path.segments[1].ident.to_string());
    }

    let qself = type_path.qself.as_ref()?;
    let Type::Path(qself_type) = qself.ty.as_ref() else {
        return None;
    };
    if qself_type.qself.is_none()
        && qself_type.path.is_ident("Self")
        && type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| matches!(segment.arguments, PathArguments::None))
    {
        return type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
    }

    None
}

fn add_attrs(mut item: ImplItemFn, attrs: &[Attribute]) -> ImplItemFn {
    item.attrs.extend(attrs.to_owned());
    item
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn substitutes_impl_associated_types_in_signatures() {
        let item_impl: ItemImpl = parse_quote! {
            impl Add for Point {
                type Output = Point;

                fn add(self, rhs: Self) -> Self::Output {
                    self
                }
            }
        };
        let associated_types = parse_associated_types(&item_impl.items);
        let mut function: ImplItemFn = parse_quote! {
            fn add(self, rhs: Self) -> Self::Output {
                self
            }
        };

        AssociatedTypeSubstitutor {
            associated_types: &associated_types,
        }
        .visit_signature_mut(&mut function.sig);

        assert_eq!(
            quote!(#function).to_string(),
            "fn add (self , rhs : Self) -> Point { self }"
        );
    }

    #[test]
    fn substitutes_qualified_impl_associated_types() {
        let item_impl: ItemImpl = parse_quote! {
            impl Add for Point {
                type Output = Point;

                fn add(self, rhs: Self) -> <Self as Add>::Output {
                    self
                }
            }
        };
        let associated_types = parse_associated_types(&item_impl.items);
        let mut function: ImplItemFn = parse_quote! {
            fn add(self, rhs: Self) -> <Self as Add>::Output {
                self
            }
        };

        AssociatedTypeSubstitutor {
            associated_types: &associated_types,
        }
        .visit_signature_mut(&mut function.sig);

        assert_eq!(
            quote!(#function).to_string(),
            "fn add (self , rhs : Self) -> Point { self }"
        );
    }
}

fn parse_trait_impl(item_impl: &ItemImpl, trait_name: &str) -> HirFlatTraitImpl {
    HirFlatTraitImpl {
        trait_name: trait_name.to_owned(),
        impl_ty: *item_impl.self_ty.clone(),
    }
}
