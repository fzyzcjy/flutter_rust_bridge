use anyhow::Context;
use log::debug;
use syn::*;
use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::parser::ParserResult;

struct FunctionParser;

impl FunctionParser {
    fn parse_function(&mut self, func: &ItemFn) -> ParserResult<IrFunc> {
        debug!("parse_function function name: {:?}", func.sig.ident);

        let sig = &func.sig;
        let func_name = sig.ident.to_string();

        let mut inputs = Vec::new();
        let mut output = None;
        let mut mode: Option<IrFuncMode> = None;
        let mut fallible = true;

        for (i, sig_input) in sig.inputs.iter().enumerate() {
            if let FnArg::Typed(ref pat_type) = sig_input {
                let name = if let Pat::Ident(ref pat_ident) = *pat_type.pat {
                    format!("{}", pat_ident.ident)
                } else {
                    return Err(Error::UnexpectedPattern(
                        quote::quote!(#pat_type).to_string().into(),
                    ));
                };
                let arg_type = self.try_parse_fn_arg_type(&pat_type.ty).with_context(|| {
                    format!(
                        "Failed to parse function argument type `{}`",
                        type_to_string(&pat_type.ty)
                    )
                })?;
                match arg_type {
                    IrFuncArg::StreamSinkType(ty) => {
                        output = Some(ty);
                        mode = Some(IrFuncMode::Stream { argument_index: i });
                        fallible = match &sig.output {
                            ReturnType::Default => false,
                            ReturnType::Type(_, ty) => !matches!(
                                self.try_parse_fn_output_type(ty),
                                Some(IrFuncOutput::Type(_))
                            ),
                        }
                    }
                    IrFuncArg::Type(ty) => {
                        inputs.push(IrField {
                            name: IrIdent::new(name),
                            ty,
                            is_final: true,
                            comments: extract_comments(&pat_type.attrs),
                            default: DefaultValues::extract(&pat_type.attrs),
                            settings: IrFieldSettings::default(),
                        });
                    }
                }
            } else {
                return Err(Error::UnexpectedSigInput(
                    quote::quote!(#sig_input).to_string().into(),
                ));
            }
        }

        let result = match &sig.output {
            ReturnType::Type(_, ty) => {
                let output_type = self.try_parse_fn_output_type(ty).with_context(|| {
                    format!(
                        "Failed to parse function output type `{}`",
                        type_to_string(ty)
                    )
                })?;
                match output_type {
                    IrFuncOutput::ResultType { ok: ty, error: err } => (ty, err),
                    IrFuncOutput::Type(ty) => {
                        fallible = false;
                        (ty, None)
                    }
                }
            }
            ReturnType::Default => {
                fallible = false;
                (IrType::Primitive(IrTypePrimitive::Unit), None)
            }
        };

        if matches!(mode, Some(IrFuncMode::Stream { argument_index: _ }) if result.0 != IrType::Primitive(IrTypePrimitive::Unit))
        {
            return Err(Error::NoStreamSinkAndOutput(func_name.into()));
        }

        if output.is_none() {
            mode = Some(if let IrType::SyncReturn(_) = result.0 {
                IrFuncMode::Sync
            } else {
                IrFuncMode::Normal
            });
            output = Some(result.0);
        }

        Ok(IrFunc {
            name: func_name,
            inputs,
            output: output.context("Unsupported output")?,
            fallible,
            mode: mode.context("Missing mode")?,
            comments: extract_comments(&func.attrs),
            error_output: result.1,
        })
    }
}
