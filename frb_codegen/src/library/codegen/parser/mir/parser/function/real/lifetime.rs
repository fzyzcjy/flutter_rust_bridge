use syn::Signature;

pub(crate) fn parse_function_lifetime(sig: &Signature) -> ParseFunctionLifetimeOutput {
    let inputs_lifetimes: Vec<Vec<TODO>> = TODO;
    let output_lifetimes: Vec<TODO> = TODO;
    todo!()
}

pub(crate) struct ParseFunctionLifetimeOutput {
    pub needs_extend_lifetime_per_arg: Vec<bool>,
}
