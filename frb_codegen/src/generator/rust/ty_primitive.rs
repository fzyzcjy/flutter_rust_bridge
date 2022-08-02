use crate::config::Acc;
use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypePrimitiveGenerator, IrTypePrimitive);

impl TypeRustGeneratorTrait for TypePrimitiveGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        "self".into()
    }
    fn wasm2api_body(&self) -> Option<std::borrow::Cow<str>> {
        Some("self.unchecked_into_f64() as _".into())
    }
}
