use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for OptionalWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: (!is_js_value(&self.ir.inner)
                && !self.ir.is_primitive()
                && !self.ir.is_boxed_primitive())
            .then(|| "self.map(Wire2Api::wire2api)".into()),
            ..Default::default()
        }
    }
}
