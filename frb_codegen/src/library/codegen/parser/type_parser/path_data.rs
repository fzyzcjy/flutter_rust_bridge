use crate::codegen::ir::ty::unencodable::{Args, NameComponent};
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::if_then_some;
use anyhow::Result;
use anyhow::{anyhow, Context};
use quote::ToTokens;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, Path,
    PathArguments, PathSegment,
};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn extract_path_data(&mut self, path: &Path) -> Result<Vec<NameComponent>> {
        path.segments
            .iter()
            .map(|segment| self.parse_path_segment(path, segment))
            .collect()
    }

    fn parse_path_segment(&mut self, path: &Path, segment: &PathSegment) -> Result<NameComponent> {
        let ident = segment.ident.to_string();
        let args = match &segment.arguments {
            PathArguments::None => None,
            PathArguments::AngleBracketed(args) => {
                let ir_types = self
                    .parse_angle_bracketed_generic_arguments(args)
                    .with_context(|| {
                        anyhow!("\"{ident}\" of \"{}\" is not valid", path.to_token_stream())
                    })?;
                Some(Args::Generic(ir_types))
            }
            // not used yet (detected by codecov)
            // syn doc says "The `(A, B) -> C` in `Fn(A, B) -> C`",
            // thus it seems we will not use it here.
            //
            // PathArguments::Parenthesized(args) => Some(Args::Signature(
            //     self.parse_parenthesized_generic_arguments(args)?,
            // )),
        };
        Ok(NameComponent { ident, args })
    }

    fn parse_angle_bracketed_generic_arguments(
        &mut self,
        args: &AngleBracketedGenericArguments,
    ) -> Result<Vec<IrType>> {
        args.args
            .iter()
            .filter_map(|arg| if_then_some!(let GenericArgument::Type(ty) = arg, ty))
            .map(|ty| self.parse_type(ty))
            .collect()
    }

    // not used yet
    // fn parse_parenthesized_generic_arguments(
    //     &mut self,
    //     args: &ParenthesizedGenericArguments,
    // ) -> Result<Vec<IrType>> {
    //     let input_types = args
    //         .inputs
    //         .iter()
    //         .map(|ty| self.parse_type(ty))
    //         .collect::<Result<Vec<_>>>()?;
    //
    //     let output_type = self.parse_return_type(&args.output)?;
    //
    //     Ok({
    //         let mut ans = vec![output_type];
    //         ans.extend(input_types);
    //         ans
    //     })
    // }
}
