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

use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartCodecDcoGeneratorDecoderTrait {
    fn generate_impl_decode_body(&self) -> String;
}
