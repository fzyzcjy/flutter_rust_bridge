use crate::codegen::generator::wire::dart::spec_generator::base::*;

mod boxed;
mod dart_fn;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod optional_list;
mod ownership;
mod primitive;
mod primitive_list;
mod record;
mod rust_auto_opaque;
mod rust_opaque;
mod structure;
mod unencodable;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartGeneratorRust2DartTrait {}
