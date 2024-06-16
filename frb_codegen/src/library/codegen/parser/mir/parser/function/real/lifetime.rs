use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::parser::mir::parser::function::real::argument::parse_argument_ty_and_name;
use crate::codegen::parser::mir::parser::lifetime_extractor::{Lifetime, LifetimeExtractor};
use anyhow::ensure;
use itertools::{concat, Itertools};
use std::collections::HashSet;
use syn::{ReturnType, Signature};

pub(crate) fn parse_function_lifetime(
    sig: &Signature,
    owner: &MirFuncOwnerInfo,
) -> anyhow::Result<ParseFunctionLifetimeOutput> {
    let inputs_lifetimes: Vec<Vec<Lifetime>> = (sig.inputs.iter())
        .map(|x| {
            LifetimeExtractor::extract_skipping_static(&parse_argument_ty_and_name(x, owner).0)
        })
        .collect_vec();
    let output_lifetimes: Vec<Lifetime> = match &sig.output {
        ReturnType::Type(_, ty) => LifetimeExtractor::extract_skipping_static(&*ty),
        ReturnType::Default => vec![],
    };

    ensure_one_lifetime(&inputs_lifetimes, &output_lifetimes)?;

    Ok(ParseFunctionLifetimeOutput {
        needs_extend_lifetime_per_arg: (inputs_lifetimes.iter())
            .map(|input_lifetimes| !input_lifetimes.is_empty())
            .collect_vec(),
    })
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

pub(crate) struct ParseFunctionLifetimeOutput {
    pub needs_extend_lifetime_per_arg: Vec<bool>,
}
