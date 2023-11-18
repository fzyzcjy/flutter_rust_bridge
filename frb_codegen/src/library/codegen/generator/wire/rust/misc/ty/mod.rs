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

use crate::codegen::generator::misc::Target;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorMiscTrait {
    fn rust_wire_type(&self, target: Target) -> String;

    fn rust_wire_modifier(&self, target: Target) -> String {
        if self.rust_wire_is_pointer(target) {
            "*mut ".to_string()
        } else {
            "".to_string()
        }
    }

    fn rust_wire_is_pointer(&self, _target: Target) -> bool {
        false
    }

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
