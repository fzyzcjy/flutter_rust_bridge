use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::unencodable::{ArgsRefs, Splayable};
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::parser::ParserResult;
use anyhow::Context;
use log::debug;
use quote::quote;
use syn::*;

const STREAM_SINK_IDENT: &str = "StreamSink";

pub(crate) struct FunctionParser<'a, 'b> {
    type_parser: &'a mut TypeParser<'b>,
}

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(crate) fn new(type_parser: &'a mut TypeParser<'b>) -> Self {
        Self { type_parser }
    }

    pub(crate) fn parse_function(&mut self, func: &ItemFn) -> ParserResult<IrFunc> {
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
                    return Err(super::error::Error::UnexpectedPattern(
                        quote::quote!(#pat_type).to_string().into(),
                    ));
                };
                let arg_type = self.parse_fn_arg_type(&pat_type.ty)?.with_context(|| {
                    format!(
                        "Failed to parse function argument type `{}`",
                        type_to_string(&pat_type.ty)
                    )
                })?;
                match arg_type {
                    FuncArg::StreamSinkType(ty) => {
                        output = Some(ty);
                        mode = Some(IrFuncMode::Stream { argument_index: i });
                        fallible = match &sig.output {
                            ReturnType::Default => false,
                            ReturnType::Type(_, ty) => {
                                !matches!(self.parse_fn_output_type(ty)?, Some(FuncOutput::Type(_)))
                            }
                        }
                    }
                    FuncArg::Type(ty) => {
                        let attributes = FrbAttributes::parse(&pat_type.attrs)?;
                        inputs.push(IrField {
                            name: IrIdent::new(name),
                            ty,
                            is_final: true,
                            comments: parse_comments(&pat_type.attrs),
                            default: attributes.default_value(),
                            settings: IrFieldSettings::default(),
                        });
                    }
                }
            } else {
                return Err(super::error::Error::UnexpectedSigInput(
                    quote::quote!(#sig_input).to_string().into(),
                ));
            }
        }

        let (output_ok, output_err) = match &sig.output {
            ReturnType::Type(_, ty) => {
                let output_type = self.parse_fn_output_type(ty)?.with_context(|| {
                    format!(
                        "Failed to parse function output type `{}`",
                        type_to_string(ty)
                    )
                })?;
                match output_type {
                    FuncOutput::ResultType { ok: ty, error: err } => (ty, err),
                    FuncOutput::Type(ty) => {
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

        if matches!(mode, Some(IrFuncMode::Stream { argument_index: _ }) if output_ok != IrType::Primitive(IrTypePrimitive::Unit))
        {
            return Err(super::error::Error::NoStreamSinkAndOutput(func_name.into()));
        }

        if output.is_none() {
            // TODO handle SyncReturn as a marker
            // mode = Some(if let IrType::SyncReturn(_) = output_ok {
            //     IrFuncMode::Sync
            // } else {
            //     IrFuncMode::Normal
            // });
            mode = Some(IrFuncMode::Normal);
            output = Some(output_ok);
        }

        Ok(IrFunc {
            name: func_name,
            inputs,
            output: output.context("Unsupported output")?,
            fallible,
            mode: mode.context("Missing mode")?,
            comments: parse_comments(&func.attrs),
            error_output: output_err,
        })
    }

    /// Attempts to parse the type from an argument of a function signature. There is a special
    /// case for top-level `StreamSink` types.
    fn parse_fn_arg_type(&mut self, ty: &Type) -> anyhow::Result<Option<FuncArg>> {
        Ok(match ty {
            Type::Path(TypePath { path, .. }) => {
                let last_segment = path.segments.last().unwrap();
                if last_segment.ident == STREAM_SINK_IDENT {
                    match &last_segment.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) if args.len() == 1 => {
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
                    Some(FuncArg::Type(self.type_parser.parse_type(ty)?))
                }
            }
            Type::Array(_) => Some(FuncArg::Type(self.type_parser.parse_type(ty)?)),
            _ => None,
        })
    }

    /// Attempts to parse the type from the return part of a function signature. There is a special
    /// case for top-level `Result` types.
    fn parse_fn_output_type(&mut self, ty: &Type) -> anyhow::Result<Option<FuncOutput>> {
        let ty = &self.type_parser.resolve_alias(ty).clone();

        Ok(if let Type::Path(type_path) = ty {
            match self.type_parser.parse_type_path(&type_path) {
                Ok(IrType::Unencodable(IrTypeUnencodable { segments, .. })) => {
                    match segments.splay()[..] {
                        [("anyhow", None), ("Result", Some(ArgsRefs::Generic([args])))] => {
                            Some(FuncOutput::ResultType {
                                ok: args.clone(),
                                error: None,
                            })
                        }
                        // TODO deal with std::result::Result and anyhow::Result
                        [("Result", Some(ArgsRefs::Generic(args)))] => {
                            let ok = args.first().unwrap();

                            let is_anyhow = args.len() == 1
                                || args.iter().any(|x| match x {
                                    IrType::Unencodable(IrTypeUnencodable { string, .. }) => {
                                        string == "anyhow :: Error"
                                    }
                                    _ => false,
                                });
                            let error = if is_anyhow {
                                Some(IrType::Delegate(IrTypeDelegate::Anyhow))
                            } else {
                                args.last().cloned()
                            };

                            let error = if let Some(StructRef(mut struct_ref)) = error {
                                struct_ref.is_exception = true;
                                Some(StructRef(struct_ref))
                            } else if let Some(EnumRef(mut enum_ref)) = error {
                                enum_ref.is_exception = true;
                                Some(EnumRef(enum_ref))
                            } else {
                                error
                            };

                            Some(FuncOutput::ResultType {
                                ok: ok.clone(),
                                error,
                            })
                        }
                        _ => None, // unencodable types not implemented
                    }
                }
                Ok(result) => Some(FuncOutput::Type(result)),
                Err(..) => None,
            }
        } else {
            let ir_ty = self.type_parser.parse_type(ty)?;
            Some(FuncOutput::Type(ir_ty))
        })
    }
}

/// Represents a function's output type
#[derive(Debug, Clone)]
enum FuncOutput {
    ResultType { ok: IrType, error: Option<IrType> },
    Type(IrType),
}

/// Represents the type of an argument to a function
#[derive(Debug, Clone)]
enum FuncArg {
    StreamSinkType(IrType),
    Type(IrType),
}

/// syn -> string https://github.com/dtolnay/syn/issues/294
fn type_to_string(ty: &Type) -> String {
    quote!(#ty).to_string().replace(' ', "")
}
