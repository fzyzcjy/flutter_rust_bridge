use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl WireRustCodecCstGeneratorDecoderTrait for GenericWireRustCodecCstGenerator<'_> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc::default()
    }
    
    fn rust_wire_type(&self, _target: Target) -> String {
        // Generic types should not appear in generated code
        "UNREACHABLE_RUST_WIRE_TYPE".into()
    }
}

