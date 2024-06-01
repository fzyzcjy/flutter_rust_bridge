use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for DartFnWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(format!(
            "return cst_encode_DartOpaque(encode_{}(raw));",
            self.mir.safe_ident(),
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        WireDartCodecCstGenerator::new(self.mir.get_delegate(), self.context).dart_wire_type(target)
    }
}
