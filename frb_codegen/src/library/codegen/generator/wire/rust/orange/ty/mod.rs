use crate::codegen::generator::wire::rust::base::*;

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

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorOrangeTrait {
    fn f_orange(&self) {}
}

fn f_out_orange(a: WireRustGenerator) {
    a.f_orange()
}
