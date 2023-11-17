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
pub(crate) trait WireRustGeneratorApi2wireTrait {
    fn generate_impl_into_dart(&self) -> Option<String> {
        None
    }

    fn generate_convert_to_dart(&self, obj: String) -> String {
        format!("{obj}.into_into_dart().into_dart()")
    }

    fn self_access(&self, obj: String) -> String {
        obj
    }
}
