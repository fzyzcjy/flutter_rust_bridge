use crate::codegen::generator::wire::rust::spec_generator::base::*;

mod boxed;
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
pub(crate) trait WireRustGeneratorMiscTrait {
    fn wrapper_struct_name(&self) -> Option<String> {
        None
    }

    fn generate_static_checks(&self) -> Option<String> {
        None
    }

    fn generate_imports(&self) -> Option<Vec<String>> {
        None
    }
}
