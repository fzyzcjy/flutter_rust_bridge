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

    let all_lifetimes = (inputs_lifetimes.iter().flatten())
        .chain(output_lifetimes.iter())
        .collect::<HashSet<_>>();
    ensure!(
        all_lifetimes.len() <= 1,
        "Only support <=1 lifetime specifiers yet, but see {:?}",
        all_lifetimes
    );

    todo!()
}

pub(crate) struct ParseFunctionLifetimeOutput {
    pub needs_extend_lifetime_per_arg: Vec<bool>,
}
