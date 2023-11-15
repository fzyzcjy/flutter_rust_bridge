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
