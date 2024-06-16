use crate::codegen::parser::mir::parser::lifetime_extractor::Lifetime;
use anyhow::ensure;
use itertools::{concat, Itertools};
use std::collections::HashSet;
use syn::Signature;

pub(crate) fn parse_function_lifetime(
    sig: &Signature,
) -> anyhow::Result<ParseFunctionLifetimeOutput> {
    let inputs_lifetimes: Vec<Vec<Lifetime>> = TODO;
    let output_lifetimes: Vec<Lifetime> = TODO;

    ensure_one_lifetime(&inputs_lifetimes, &output_lifetimes)?;

    todo!()
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
