use crate::codegen::parser::function_parser::{FuncArg, FunctionParser, STREAM_SINK_IDENT};
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    /// Attempts to parse the type from an argument of a function signature. There is a special
    /// case for top-level `StreamSink` types.
    pub(super) fn parse_fn_arg_type(&mut self, ty: &Type) -> anyhow::Result<Option<FuncArg>> {
        Ok(match ty {
            Type::Path(TypePath { path, .. }) => {
                if let Some(ans) = self.parse_fn_arg_type_stream_sink(path)? {
                    Some(ans)
                } else {
                    Some(FuncArg::Type(self.type_parser.parse_type(ty)?))
                }
            }
            Type::Array(_) => Some(FuncArg::Type(self.type_parser.parse_type(ty)?)),
            _ => None,
        })
    }

    fn parse_fn_arg_type_stream_sink(&mut self, path: &Path) -> anyhow::Result<Option<FuncArg>> {
        let last_segment = path.segments.last().unwrap();
        Ok(if last_segment.ident == STREAM_SINK_IDENT {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
                    if args.len() == 1 =>
                {
                    // Unwrap is safe here because args.len() == 1
                    match args.last().unwrap() {
                        GenericArgument::Type(t) => {
                            Some(FuncArg::StreamSinkType(self.type_parser.parse_type(t)?))
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        })
    }
}
