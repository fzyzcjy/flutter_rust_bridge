use crate::codegen::ir::ty::dart_fn::IrTypeDartFn;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::unencodable::{ArgsRefs, SplayedSegment};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::if_then_some;
use anyhow::{bail, ensure, Context};
use enum_iterator::next;
use itertools::{unfold, Itertools};
use quote::__private::ext::RepToTokensExt;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, ReturnType,
    Type, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath,
};
use ArgsRefs::Generic;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_impl_trait_dart_fn(
        &mut self,
        type_impl_trait: &TypeImplTrait,
    ) -> anyhow::Result<IrType> {
        let trait_bound = (type_impl_trait.bounds.iter())
            .filter_map(
                |x| if_then_some!(let TypeParamBound::Trait(trait_bound) = x, trait_bound.clone()),
            )
            .next()
            .context("cannot find trait_bound")?;

        let segment = (trait_bound.path.segments.last()).context("cannot get segment")?;

        let segment_ident = segment.ident.to_string();
        match &segment_ident[..] {
            // TODO Currently, we treat `FnOnce` same as `Fn`,
            //      but in the future we can optimize this because we no longer need Arc or Clone for this case.
            "FnOnce" | "Fn" => {} // Ok
            _ => bail!("Unknown ident: {segment_ident}"),
        }

        if let PathArguments::Parenthesized(arguments) = &segment.arguments {
            let inputs = (arguments.inputs.iter())
                .map(|x| self.parse_type(x))
                .collect::<anyhow::Result<Vec<_>>>()?;

            let output = Box::new(self.parse_dart_fn_output(&arguments.output)?);

            return Ok(IrType::DartFn(IrTypeDartFn { inputs, output }));
        }

        bail!("Fail to parse DartFn")
    }

    fn parse_dart_fn_output(&mut self, return_type: &ReturnType) -> anyhow::Result<IrType> {
        if let ReturnType::Type(_, ret_ty) = return_type {
            if let Type::Path(TypePath { ref path, .. }) = **ret_ty {
                if let Some(PathSegment {
                    ident,
                    arguments:
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
                }) = path.segments.first()
                {
                    if &ident.to_string() == "DartFnFuture" {
                        if let GenericArgument::Type(inner_ty) = (args.iter())
                            .filter(|arg| matches!(arg, GenericArgument::Type(_)))
                            .next()
                            .unwrap()
                        {
                            return self.parse_type(inner_ty);
                        }
                    }
                }
            }
        }

        bail!("DartFn does not support return types except `DartFnFuture<T>` yet")
    }
}

// // Use this unit "test" to see how a type will be parsed into a tree
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_syn_parse_example() {
//         let ans: syn::Type =
//             syn::parse_str("impl Fn(Apple) -> DartFnFuture<Orange> + UnwindSafe").unwrap();
//         println!("{ans:#?}");
//     }
// }
