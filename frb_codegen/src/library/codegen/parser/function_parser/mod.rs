pub(crate) mod argument;
pub(crate) mod output;

use crate::codegen::ir::field::IrField;
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParser;
use anyhow::{bail, Context};
use itertools::concat;
use log::debug;
use quote::quote;
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

    pub(crate) fn parse_function(&mut self, func: &ItemFn) -> anyhow::Result<IrFunc> {
        debug!("parse_function function name: {:?}", func.sig.ident);

        let sig = &func.sig;
        let func_name = sig.ident.to_string();

        let mut info = FunctionPartialInfo::default();

        for (i, sig_input) in sig.inputs.iter().enumerate() {
            info = info.merge(self.parse_fn_arg(i, sig_input)?)?;
        }

        info = info.merge(self.parse_fn_output(sig)?)?;

        Ok(IrFunc {
            name: func_name,
            inputs: info.inputs,
            output: info.ok_output.unwrap_or(Primitive(IrTypePrimitive::Unit)),
            error_output: info.error_output,
            mode: info.mode.unwrap_or(IrFuncMode::Normal),
            comments: parse_comments(&func.attrs),
        })
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
            ok_output: merge_option(self.ok_output, other.ok_output)?,
            error_output: merge_option(self.error_output, other.error_output)?,
            mode: merge_option(self.mode, other.mode)?,
        })
    }
}

fn merge_option<T>(a: Option<T>, b: Option<T>) -> anyhow::Result<Option<T>> {
    if a.is_some() && b.is_some() {
        bail!("Function has conflicting arguments and/or outputs");
    }
    Ok(a.or(b))
}

/// syn -> string https://github.com/dtolnay/syn/issues/294
fn type_to_string(ty: &Type) -> String {
    quote!(#ty).to_string().replace(' ', "")
}
