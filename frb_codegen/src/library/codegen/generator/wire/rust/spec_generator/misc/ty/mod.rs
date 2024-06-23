use crate::codegen::generator::wire::rust::spec_generator::base::*;

mod boxed;
pub(crate) mod dart_fn;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod primitive;
mod primitive_list;
mod record;
mod rust_auto_opaque_implicit;
mod rust_opaque;
mod structure;
mod trait_def;

use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::utils::namespace::Namespace;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorMiscTrait {
    fn wrapper_struct_name(&self) -> Option<String> {
        None
    }

    fn generate_static_checks(&self) -> Option<String> {
        None
    }

    fn generate_imports(&self) -> Option<Vec<Namespace>> {
        None
    }

    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        Default::default()
    }

    fn generate_wire_func_call_decode_wrapper(&self) -> Option<String> {
        None
    }

    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        None
    }
}
