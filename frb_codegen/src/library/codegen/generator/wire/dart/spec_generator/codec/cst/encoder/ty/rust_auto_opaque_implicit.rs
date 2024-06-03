use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::rust_opaque::rust_opaque_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait
    for RustAutoOpaqueImplicitWireDartCodecCstGenerator<'a>
{
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        let needs_move = self.mir.needs_move();
        Acc::new_common(Some(format!(
            "// ignore: invalid_use_of_internal_member
            return raw.frbInternalCstEncode(move: {needs_move});",
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        rust_opaque_dart_wire_type(target)
    }
}
