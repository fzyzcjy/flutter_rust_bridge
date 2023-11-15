pub(crate) mod markers;
pub(crate) mod source_graph;
pub(crate) mod ty;

use std::borrow::Cow;
use std::collections::HashMap;
use std::default::Default as _;
use std::string::String;

use anyhow::Context;
use log::debug;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::*;
use topological_sort::TopologicalSort;

use crate::ir::*;

use crate::generator::rust::HANDLER_NAME;
use crate::parser::source_graph::Crate;
use crate::parser::ty::TypeParser;
use crate::parser::IrType::{EnumRef, StructRef};
use crate::utils::method::FunctionName;

use self::ty::convert_ident_str;

mod error;
pub use error::Error;

impl<'a> Parser<'a> {
    fn parse(mut self, source_rust_content: &str, src_fns: Vec<ItemFn>) -> ParserResult<IrPack> {
        let funcs = src_fns
            .iter()
            .map(|f| self.parse_function(f))
            .collect::<ParserResult<Vec<_>>>()?;

        let has_executor = source_rust_content.contains(HANDLER_NAME);

        let (struct_pool, enum_pool) = self.type_parser.consume();

        Ok(IrPack {
            funcs,
            struct_pool,
            enum_pool,
            has_executor,
        })
    }
}

impl IrDefaultValue {
    // TODO use `parse_metadata`
    // pub(crate) fn extract(attrs: &[Attribute]) -> Option<Self> {
    //     let defaults = attrs
    //         .iter()
    //         .filter(|attr| attr.path().is_ident("frb"))
    //         .map(|attr| attr.parse_args::<FrbOption>())
    //         .filter_map(|attr| {
    //             if let Ok(FrbOption::Default(default)) = attr {
    //                 Some(default)
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect::<Vec<_>>();
    //     match &defaults[..] {
    //         [] => None,
    //         [single] => Some(single.clone()),
    //         [.., last] => {
    //             log::warn!("Only one `default = ..` attribute is expected; taking the last one");
    //             Some(last.clone())
    //         }
    //     }
    // }

    pub(crate) fn to_dart(&self) -> Cow<str> {
        match self {
            Self::Bool(lit) => if lit.value { "true" } else { "false" }.into(),
            Self::Str(lit) => format!("r\"{}\"", lit.value()).into(),
            Self::Int(lit) => lit.base10_digits().into(),
            Self::Float(lit) => lit.base10_digits().into(),
            Self::Vec(lit) => format!(
                "const [{}]",
                lit.iter().map(Self::to_dart).collect::<Vec<_>>().join(",")
            )
            .into(),
        }
    }
}
