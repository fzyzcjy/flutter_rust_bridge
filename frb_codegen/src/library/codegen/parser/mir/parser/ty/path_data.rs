use crate::codegen::ir::mir::ty::rust_opaque::NameComponent;
use anyhow::{bail, Result};
use syn::{Path, PathArguments, PathSegment};

pub(crate) fn extract_path_data(path: &Path) -> Result<Vec<NameComponent>> {
    path.segments.iter().map(parse_path_segment).collect()
}

fn parse_path_segment(segment: &PathSegment) -> Result<NameComponent> {
    let ident = segment.ident.to_string();
    let args = match &segment.arguments {
        PathArguments::None => vec![],
        PathArguments::AngleBracketed(args) => {
            args.args.iter().map(|arg| arg.to_owned()).collect()
            // .with_context(|| {
            //     // This will stop the whole generator and tell the users, so we do not care about testing it
            //     // frb-coverage:ignore-start
            //     anyhow!("\"{ident}\" of \"{}\" is not valid", path.to_token_stream())
            //     // frb-coverage:ignore-end
            // })?
        },
        // frb-coverage:ignore-start
        PathArguments::Parenthesized(_) => bail!("parsing parenthesized path arguments is not implemented yet"),
        // frb-coverage:ignore-end

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

// not used yet
// fn parse_parenthesized_generic_arguments(
//     &mut self,
//     args: &ParenthesizedGenericArguments,
// ) -> Result<Vec<MirType>> {
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
