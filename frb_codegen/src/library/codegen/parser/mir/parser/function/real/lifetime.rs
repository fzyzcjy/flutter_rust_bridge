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
