use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::IrFuncMode;
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::function_parser::{
    type_to_string, FunctionParser, FunctionPartialInfo, STREAM_SINK_IDENT,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use anyhow::anyhow;
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_arg(
        &mut self,
        argument_index: usize,
        sig_input: &FnArg,
    ) -> anyhow::Result<FunctionPartialInfo> {
        if let FnArg::Typed(ref pat_type) = sig_input {
            let ty = pat_type.ty.as_ref();
            match &ty {
                Type::Path(TypePath { path, .. }) => {
                    if let Some(ans) = self.parse_fn_arg_type_stream_sink(path, pat_type)? {
                        Ok(ans)
                    } else {
                        partial_info_for_normal_type(
                            self.type_parser.parse_type(ty)?,
                            argument_index,
                        )
                    }
                }
                Type::Array(_) => {
                    partial_info_for_normal_type(self.type_parser.parse_type(ty)?, argument_index)
                }
                _ => Err(anyhow!(
                    "Failed to parse function argument type `{}`",
                    type_to_string(ty)
                )
                .into()),
            }
        } else {
            Err(super::super::error::Error::UnexpectedSigInput(
                quote::quote!(#sig_input).to_string().into(),
            ))
        }
    }

    fn parse_fn_arg_type_stream_sink(
        &mut self,
        path: &Path,
        pat_type: &PatType,
    ) -> anyhow::Result<Option<FunctionPartialInfo>> {
        let last_segment = path.segments.last().unwrap();
        Ok(if last_segment.ident == STREAM_SINK_IDENT {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
                    if args.len() == 1 =>
                {
                    // Unwrap is safe here because args.len() == 1
                    match args.last().unwrap() {
                        GenericArgument::Type(t) => Some(partial_info_for_stream_sink_type(
                            self.type_parser.parse_type(t)?,
                            pat_type,
                        )?),
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

fn partial_info_for_normal_type(
    ty: IrType,
    argument_index: usize,
) -> anyhow::Result<FunctionPartialInfo> {
    Ok(FunctionPartialInfo {
        ok_output: Some(ty),
        mode: Some(IrFuncMode::Stream { argument_index }),
        ..Default::default()
    })
}

fn partial_info_for_stream_sink_type(
    ty: IrType,
    pat_type: &PatType,
) -> anyhow::Result<FunctionPartialInfo> {
    let name = parse_name(pat_type)?;
    let attributes = FrbAttributes::parse(&pat_type.attrs)?;
    Ok(FunctionPartialInfo {
        inputs: vec![IrField {
            name: IrIdent::new(name),
            ty,
            is_final: true,
            comments: parse_comments(&pat_type.attrs),
            default: attributes.default_value(),
            settings: IrFieldSettings::default(),
        }],
        ..Default::default()
    })
}

fn parse_name(pat_type: &PatType) -> anyhow::Result<String> {
    if let Pat::Ident(ref pat_ident) = *pat_type.pat {
        Ok(format!("{}", pat_ident.ident))
    } else {
        Err(super::super::error::Error::UnexpectedPattern(
            quote::quote!(#pat_type).to_string().into(),
        ))
    }
}
