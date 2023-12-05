use crate::codegen::ir::ty::dart_fn::IrTypeDartFn;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::unencodable::{ArgsRefs, SplayedSegment};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use itertools::Itertools;
use quote::__private::ext::RepToTokensExt;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, ReturnType,
    Type, TypeBareFn, TypePath,
};
use ArgsRefs::Generic;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_dart_fn(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        match last_segment {
            ("DartFn", Some(Generic([IrType::Unencodable(IrTypeUnencodable { string, .. })]))) => {
                self.parse_dart_fn(string)
            }
            _ => Ok(None),
        }
    }

    fn parse_dart_fn(&mut self, raw: &str) -> anyhow::Result<Option<IrType>> {
        let ty: syn::Type = syn::parse_str(raw)?;

        if let Type::BareFn(bare_fn) = ty {
            let inputs = (bare_fn.inputs.iter())
                .map(|x| self.parse_type(&x.ty))
                .collect::<anyhow::Result<Vec<_>>>()?;

            let output = Box::new(self.parse_dart_fn_output(&bare_fn.output)?);

            return Ok(Some(IrType::DartFn(IrTypeDartFn { inputs, output })));
        }

        Ok(None)
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
                    if &ident.to_string() == "BoxFuture" {
                        if let GenericArgument::Type(inner_ty) = args.first().unwrap() {
                            return self.parse_type(inner_ty);
                        }
                    }
                }
            }
        }

        bail!("DartFn does not support return types except `BoxFuture<T>` yet")
    }
}

// Use this unit "test" to see how a type will be parsed into a tree
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_syn_parse_example() {
//         let ans: syn::Type = syn::parse_str("fn(DartOpaque) -> BoxFuture<String>").unwrap();
//         println!("{ans:#?}");
//     }
// }
