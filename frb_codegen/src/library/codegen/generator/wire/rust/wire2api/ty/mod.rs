use crate::codegen::generator::wire::rust::base::*;

mod dart_opaque;
mod delegate;
mod enumeration;
mod general_list;
mod optional_list;
mod primitive_list;
mod record;
mod rust_opaque;
mod structure;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorWire2apiTrait {
    fn generate_wire2api_class(&self) -> Option<String> {
        None
    }
}

// the following have empty implementation
impl<'a> WireRustGeneratorWire2apiTrait for BoxedWireRustGenerator<'a> {}
impl<'a> WireRustGeneratorWire2apiTrait for DynamicWireRustGenerator<'a> {}
impl<'a> WireRustGeneratorWire2apiTrait for OptionalWireRustGenerator<'a> {}
impl<'a> WireRustGeneratorWire2apiTrait for PrimitiveWireRustGenerator<'a> {}
impl<'a> WireRustGeneratorWire2apiTrait for UnencodableWireRustGenerator<'a> {}
