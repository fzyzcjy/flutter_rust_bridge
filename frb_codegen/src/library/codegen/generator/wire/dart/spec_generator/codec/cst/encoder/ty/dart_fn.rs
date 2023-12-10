use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for DartFnWireDartCodecCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some("return cst_encode_DartOpaque(raw);".to_owned()))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        WireDartCodecCstGenerator::new(self.ir.get_delegate(), self.context).dart_wire_type(target)
    }
}
