use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::IrType;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for DartFnWireDartCodecCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Wasm => {
                Some("return cst_encode_DartOpaque(raw);".to_owned())
            }
            TargetOrCommon::Common => None,
        })
    }

    fn dart_wire_type(&self, target: Target) -> String {
        WireDartCodecCstGenerator::new(self.ir.get_delegate(), self.context).dart_wire_type(target)
    }
}
