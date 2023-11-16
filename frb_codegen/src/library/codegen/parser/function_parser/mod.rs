pub(crate) mod argument;
pub(crate) mod output;

use crate::codegen::ir::field::IrField;
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::parser::ParserResult;
use anyhow::Context;
use itertools::concat;
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

        let mut info = FunctionPartialInfo::default();

        for (i, sig_input) in sig.inputs.iter().enumerate() {
            info = info.merge(self.parse_fn_arg(i, sig_input)?);
        }

        info = info.merge(self.parse_fn_output()?);

        Ok(IrFunc {
            name: func_name,
            inputs: info.inputs,
            output: info.output.context("Unsupported output")?,
            fallible: info.fallible.unwrap_or(true),
            mode: info.mode.context("Missing mode")?,
            comments: parse_comments(&func.attrs),
            error_output: output_err,
        })
    }
}

#[derive(Debug, Default)]
struct FunctionPartialInfo {
    inputs: Vec<IrField>,
    output: Option<IrType>,
    mode: Option<IrFuncMode>,
    fallible: Option<bool>,
}

impl FunctionPartialInfo {
    fn merge(self, other: Self) -> Self {
        Self {
            inputs: concat([self.inputs, other.inputs]),
            output: other.output.or(self.output),
            mode: other.mode.or(self.mode),
            fallible: other.fallible.or(self.fallible),
        }
    }
}

/// syn -> string https://github.com/dtolnay/syn/issues/294
fn type_to_string(ty: &Type) -> String {
    quote!(#ty).to_string().replace(' ', "")
}
