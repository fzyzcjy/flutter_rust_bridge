use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for DartOpaqueWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(
            "return encodeDartOpaque(raw, portManager.dartHandlerPort, generalizedFrbRustBinding);"
                .to_owned(),
        ))
    }

    fn dart_wire_type(&self, _target: Target) -> String {
        "PlatformPointer".into()
    }
}
