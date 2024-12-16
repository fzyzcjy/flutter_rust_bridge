use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for DartFnWireRustCodecCstGenerator<'a> {
    fn generate_wire_func_param_api_type(&self) -> Option<String> {
        Some(self.mir.get_delegate().rust_api_type())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        WireRustCodecCstGenerator::new(self.mir.get_delegate(), self.context).rust_wire_type(target)
    }
}
