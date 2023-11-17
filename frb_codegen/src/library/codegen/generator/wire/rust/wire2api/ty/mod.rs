use crate::codegen::generator::wire::rust::base::*;
use std::borrow::Cow;

mod boxed;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod optional_list;
mod primitive;
mod primitive_list;
mod record;
mod rust_opaque;
mod structure;
mod unencodable;

use crate::codegen::generator::acc::Acc;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorWire2apiTrait {
    fn generate_wire2api_class(&self) -> Option<String> {
        None
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Default::default()
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<Cow<str>> {
        None
    }
}
