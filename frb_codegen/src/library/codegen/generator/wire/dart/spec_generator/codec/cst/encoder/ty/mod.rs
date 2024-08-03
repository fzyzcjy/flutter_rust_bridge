mod boxed;
mod dart_fn;
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
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartCodecCstGeneratorEncoderTrait {
    fn generate_encode_func_body(&self) -> Acc<Option<String>>;

    fn generate_encode_api_fill_to_wire_body(&self) -> Option<String> {
        None
    }

    fn dart_wire_type(&self, target: Target) -> String;
}
