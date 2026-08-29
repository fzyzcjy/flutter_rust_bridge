use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::parser::mir::parser::function::real::argument::parse_argument_ty_and_name;
use crate::codegen::parser::mir::parser::lifetime_extractor::{Lifetime, LifetimeExtractor};
use anyhow::ensure;
use itertools::Itertools;
use std::collections::HashSet;
use syn::{ReturnType, Signature, Type};

pub(crate) fn parse_function_lifetime(
    sig: &Signature,
    owner: &MirFuncOwnerInfo,
) -> anyhow::Result<ParseFunctionLifetimeOutput> {
    let inputs_lifetimes = (sig.inputs.iter())
        .map(|x| {
            Ok(extract_lifetime_skipping_static_and_anonymous(
                &parse_argument_ty_and_name(x, owner)?.0,
            ))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let output_lifetimes = match &sig.output {
        ReturnType::Type(_, ty) => extract_lifetime_skipping_static_and_anonymous(ty),
        ReturnType::Default => vec![],
    };

    ensure_one_lifetime(&inputs_lifetimes, &output_lifetimes)?;

    let ans = ParseFunctionLifetimeOutput {
        needs_extend_lifetime_per_arg: (inputs_lifetimes.iter())
            .map(|input_lifetimes| {
                output_lifetimes
                    .iter()
                    .any(|output_lifetime| input_lifetimes.contains(output_lifetime))
            })
            .collect_vec(),
    };
    log::debug!("parse_function_lifetime name={name} inputs_lifetimes={inputs_lifetimes:?} output_lifetimes={output_lifetimes:?} ans={ans:?}", name = sig.ident);
    Ok(ans)
}

fn extract_lifetime_skipping_static_and_anonymous(ty: &Type) -> Vec<Lifetime> {
    (LifetimeExtractor::extract_skipping_static(ty).into_iter())
        .filter(|x| !x.is_anonymous())
        .collect_vec()
}

fn ensure_one_lifetime(
    inputs_lifetimes: &[Vec<Lifetime>],
    output_lifetimes: &[Lifetime],
) -> anyhow::Result<()> {
    let all_lifetimes = (inputs_lifetimes.iter().flatten())
        .chain(output_lifetimes.iter())
        .collect::<HashSet<_>>();
    ensure!(
        all_lifetimes.len() <= 1,
        "Only support <=1 lifetime specifiers yet, but see {:?}",
        all_lifetimes
    );
    Ok(())
}

#[derive(Debug)]
pub(crate) struct ParseFunctionLifetimeOutput {
    pub needs_extend_lifetime_per_arg: Vec<bool>,
}

#[cfg(test)]
mod tests {
    use super::parse_function_lifetime;
    use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
    use syn::ItemFn;

    /// Extends only inputs whose named lifetime appears in the returned type.
    #[test]
    fn marks_only_inputs_sharing_the_output_lifetime() -> anyhow::Result<()> {
        let function: ItemFn = syn::parse_str(
            "fn example<'a>(first: &'a u8, second: &'static u8) -> &'a u8 { first }",
        )?;

        let output = parse_function_lifetime(&function.sig, &MirFuncOwnerInfo::Function)?;

        assert_eq!(output.needs_extend_lifetime_per_arg, vec![true, false]);
        Ok(())
    }

    /// Rejects signatures that require more than one distinct named lifetime.
    #[test]
    fn rejects_multiple_distinct_lifetimes() -> anyhow::Result<()> {
        let function: ItemFn = syn::parse_str(
            "fn example<'a, 'b>(first: &'a u8, second: &'b u8) { let _ = (first, second); }",
        )?;

        assert!(parse_function_lifetime(&function.sig, &MirFuncOwnerInfo::Function).is_err());
        Ok(())
    }
}
