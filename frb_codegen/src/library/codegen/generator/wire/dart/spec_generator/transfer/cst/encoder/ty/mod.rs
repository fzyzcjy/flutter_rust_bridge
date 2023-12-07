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

use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorImplTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartTransferCstGeneratorEncoderTrait {
    fn api2wire_body(&self) -> Acc<Option<String>>;

    fn api_fill_to_wire_body(&self) -> Option<String> {
        None
    }

    fn dart_wire_type(&self, target: Target) -> String;
}
