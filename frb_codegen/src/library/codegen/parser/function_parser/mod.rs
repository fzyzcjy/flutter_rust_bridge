pub(crate) mod argument;
pub(crate) mod output;
mod transformer;

use crate::codegen::ir::field::IrField;
use crate::codegen::ir::func::{
    IrFunc, IrFuncMode, IrFuncOwnerInfo, IrFuncOwnerInfoMethod, IrFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::function_extractor::GeneralizedItemFn;
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::{TypeParser, TypeParserParsingContext};
use crate::library::codegen::ir::ty::IrTypeTrait;
use anyhow::{bail, Context};
use itertools::concat;
use log::debug;
use std::fmt::Debug;
use std::path::Path;
use syn::*;
use IrType::Primitive;

const STREAM_SINK_IDENT: &str = "StreamSink";

pub(crate) struct FunctionParser<'a, 'b> {
    type_parser: &'a mut TypeParser<'b>,
}

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(crate) fn new(type_parser: &'a mut TypeParser<'b>) -> Self {
        Self { type_parser }
    }

    pub(crate) fn parse_function(
        &mut self,
        func: &GeneralizedItemFn,
        file_path: &Path,
        rust_crate_dir: &Path,
    ) -> anyhow::Result<Option<IrFunc>> {
        self.parse_function_inner(func, file_path, rust_crate_dir)
            .with_context(|| format!("function={:?}", func.sig().ident))
    }

    fn parse_function_inner(
        &mut self,
        func: &GeneralizedItemFn,
        file_path: &Path,
        rust_crate_dir: &Path,
    ) -> anyhow::Result<Option<IrFunc>> {
        debug!("parse_function function name: {:?}", func.sig().ident);

        let sig = func.sig();
        let namespace = Namespace::new_from_rust_crate_path(file_path, rust_crate_dir)?;
        let src_lineno = func.span().start().line;
        let context = TypeParserParsingContext {
            initiated_namespace: namespace.clone(),
        };

        let owner = if let Some(owner) = self.parse_owner(func, &context)? {
            owner
        } else {
            return Ok(None);
        };

        let func_name = parse_name(sig, &owner);
        let attributes = FrbAttributes::parse(func.attrs())?;

        let mut info = FunctionPartialInfo::default();
        for (i, sig_input) in sig.inputs.iter().enumerate() {
            info = info.merge(self.parse_fn_arg(i, sig_input, &owner, &context)?)?;
        }
        info = info.merge(self.parse_fn_output(sig, &context)?)?;
        info = self.transform_fn_info(info, &context);

        let codec_mode_pack = attributes.codec_mode_pack();
        let mode = compute_func_mode(&attributes, &info);

        Ok(Some(IrFunc {
            name: NamespacedName::new(namespace, func_name),
            inputs: info.inputs,
            output: info.ok_output.unwrap_or(Primitive(IrTypePrimitive::Unit)),
            error_output: info.error_output,
            owner,
            mode,
            rust_async: sig.asyncness.is_some(),
            comments: parse_comments(func.attrs()),
            codec_mode_pack,
            src_lineno,
        }))
    }

    fn parse_owner(
        &mut self,
        item_fn: &GeneralizedItemFn,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<Option<IrFuncOwnerInfo>> {
        Ok(Some(match item_fn {
            GeneralizedItemFn::Function { .. } => IrFuncOwnerInfo::Function,
            GeneralizedItemFn::Method {
                item_impl,
                impl_item_fn,
            } => {
                let mode = if matches!(impl_item_fn.sig.inputs.first(), Some(FnArg::Receiver(..))) {
                    IrFuncOwnerInfoMethodMode::Instance
                } else {
                    IrFuncOwnerInfoMethodMode::Static
                };

                let enum_or_struct_name =
                    if let Some(x) = self.parse_enum_or_struct_name(item_impl, context)? {
                        x
                    } else {
                        return Ok(None);
                    };

                IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod {
                    enum_or_struct_name,
                    actual_method_name: impl_item_fn.sig.ident.to_string(),
                    mode,
                })
            }
        }))
    }

    fn parse_enum_or_struct_name(
        &mut self,
        item_impl: &ItemImpl,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<Option<NamespacedName>> {
        let self_ty_path = if let Type::Path(self_ty_path) = item_impl.self_ty.as_ref() {
            self_ty_path
        } else {
            return Ok(None);
        };

        let enum_or_struct_name = (self_ty_path.path.segments.first().unwrap().ident).to_string();
        let syn_ty: Type = parse_str(&enum_or_struct_name)?;
        let ty = self.type_parser.parse_type(&syn_ty, context)?;

        let namespace: Option<Namespace> = ty.self_namespace();
        Ok(namespace.map(|namespace| NamespacedName::new(namespace, enum_or_struct_name)))
    }
}

fn compute_func_mode(attributes: &FrbAttributes, info: &FunctionPartialInfo) -> IrFuncMode {
    info.mode.unwrap_or(if attributes.sync() {
        IrFuncMode::Sync
    } else {
        IrFuncMode::Normal
    })
}

fn parse_name(sig: &Signature, owner: &IrFuncOwnerInfo) -> String {
    match owner {
        IrFuncOwnerInfo::Function => sig.ident.to_string(),
        IrFuncOwnerInfo::Method(method) => {
            format!(
                "{}_{}",
                method.enum_or_struct_name.name, method.actual_method_name
            )
        }
    }
}

#[derive(Debug, Default)]
struct FunctionPartialInfo {
    inputs: Vec<IrField>,
    ok_output: Option<IrType>,
    error_output: Option<IrType>,
    mode: Option<IrFuncMode>,
}

impl FunctionPartialInfo {
    fn merge(self, other: Self) -> anyhow::Result<Self> {
        Ok(Self {
            inputs: concat([self.inputs, other.inputs]),
            ok_output: merge_option(self.ok_output, other.ok_output).context("ok_output type")?,
            error_output: merge_option(self.error_output, other.error_output)
                .context("error_output type")?,
            mode: merge_option(self.mode, other.mode).context("mode")?,
        })
    }
}

fn merge_option<T: Debug>(a: Option<T>, b: Option<T>) -> anyhow::Result<Option<T>> {
    if a.is_some() && b.is_some() {
        bail!("Function has conflicting arguments and/or outputs: {a:?} and {b:?}");
    }
    Ok(a.or(b))
}
