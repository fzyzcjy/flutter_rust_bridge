use crate::codegen::generator::wire::rust::base::*;

mod dart_opaque;
mod delegate;
mod general_list;
mod optional_list;
mod primitive_list;
mod record;
mod rust_opaque;
mod structure;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustClassGeneratorClassTrait {
    fn generate_class(&self) -> Option<String> {
        None
    }
}

// the following have empty implementation
impl<'a> WireRustClassGeneratorClassTrait for BoxedWireRustGenerator<'a> {}
impl<'a> WireRustClassGeneratorClassTrait for DynamicWireRustGenerator<'a> {}
impl<'a> WireRustClassGeneratorClassTrait for EnumRefWireRustGenerator<'a> {}
impl<'a> WireRustClassGeneratorClassTrait for OptionalWireRustGenerator<'a> {}
impl<'a> WireRustClassGeneratorClassTrait for PrimitiveWireRustGenerator<'a> {}
impl<'a> WireRustClassGeneratorClassTrait for UnencodableWireRustGenerator<'a> {}
