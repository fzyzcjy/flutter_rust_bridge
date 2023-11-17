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
