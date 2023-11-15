use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::default::IrDefaultValue;
use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::ParserResult;
use anyhow::Context;
use log::debug;
use syn::*;

struct FunctionParser;

impl FunctionParser {
    fn parse_function(&self, func: &ItemFn) -> ParserResult<IrFunc> {
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
                            default: IrDefaultValue::extract(&pat_type.attrs),
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

        let (output_ok, output_err) = match &sig.output {
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

        if matches!(mode, Some(IrFuncMode::Stream { argument_index: _ }) if output_ok != IrType::Primitive(IrTypePrimitive::Unit))
        {
            return Err(Error::NoStreamSinkAndOutput(func_name.into()));
        }

        if output.is_none() {
            mode = Some(if let IrType::SyncReturn(_) = output_ok {
                IrFuncMode::Sync
            } else {
                IrFuncMode::Normal
            });
            output = Some(output_ok);
        }

        Ok(IrFunc {
            name: func_name,
            inputs,
            output: output.context("Unsupported output")?,
            fallible,
            mode: mode.context("Missing mode")?,
            comments: extract_comments(&func.attrs),
            error_output: output_err,
        })
    }

    /// Attempts to parse the type from an argument of a function signature. There is a special
    /// case for top-level `StreamSink` types.
    pub fn try_parse_fn_arg_type(&self, ty: &Type) -> Option<IrFuncArg> {
        match ty {
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
                                    Some(IrFuncArg::StreamSinkType(self.type_parser.parse_type(t)))
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                } else {
                    Some(IrFuncArg::Type(self.type_parser.parse_type(ty)))
                }
            }
            Type::Array(_) => Some(IrFuncArg::Type(self.type_parser.parse_type(ty))),
            _ => None,
        }
    }

    /// Attempts to parse the type from the return part of a function signature. There is a special
    /// case for top-level `Result` types.
    pub fn try_parse_fn_output_type(&self, ty: &Type) -> Option<IrFuncOutput> {
        let ty = &self.type_parser.resolve_alias(ty).clone();

        if let Type::Path(type_path) = ty {
            match self.type_parser.convert_path_to_ir_type(type_path) {
                Ok(IrType::Unencodable(IrTypeUnencodable { segments, .. })) => {
                    match if cfg!(feature = "qualified_names") {
                        segments.splay()
                    } else {
                        // Emulate old behavior by discarding any name qualifiers
                        vec![segments.splay().pop().unwrap()]
                    }[..]
                    {
                        #[cfg(feature = "qualified_names")]
                        [("anyhow", None), ("Result", Some(ArgsRefs::Generic([args])))] => {
                            Some(IrFuncOutput::ResultType {
                                ok: args.clone(),
                                error: None,
                            })
                        }
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

                            Some(IrFuncOutput::ResultType {
                                ok: ok.clone(),
                                error,
                            })
                        }
                        _ => None, // unencodable types not implemented
                    }
                }
                Ok(result) => Some(IrFuncOutput::Type(result)),
                Err(..) => None,
            }
        } else {
            let ir_ty = self.type_parser.parse_type(ty);
            Some(IrFuncOutput::Type(ir_ty))
        }
    }
}

fn extract_comments(attrs: &[Attribute]) -> Vec<IrComment> {
    attrs
        .iter()
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            }) if path.is_ident("doc") => Some(IrComment::from(lit.value().as_ref())),
            _ => None,
        })
        .collect()
}
