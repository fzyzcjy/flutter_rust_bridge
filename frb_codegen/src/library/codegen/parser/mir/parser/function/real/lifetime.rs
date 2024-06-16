use syn::Signature;

pub(crate) fn parse_function_lifetime(sig: &Signature) -> ParseFunctionLifetimeOutput {
    todo!()
}

pub(crate) struct ParseFunctionLifetimeOutput {
    pub needs_extend_lifetime_per_arg: Vec<bool>,
}
