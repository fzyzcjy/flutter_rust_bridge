use crate::codegen::parser::mir::parser::lifetime_extractor::Lifetime;
use syn::Signature;

pub(crate) fn parse_function_lifetime(sig: &Signature) -> ParseFunctionLifetimeOutput {
    let inputs_lifetimes: Vec<Vec<Lifetime>> = etod;
    let output_lifetimes: Vec<Lifetime> = TODO;
    todo!()
}

pub(crate) struct ParseFunctionLifetimeOutput {
    pub needs_extend_lifetime_per_arg: Vec<bool>,
}
