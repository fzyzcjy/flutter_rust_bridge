use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for RustOpaqueWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(
            format!(
                "// ignore: invalid_use_of_internal_member
                return (raw as {}Impl).frbInternalCstEncode();",
                TODO,
            ),
        ))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        rust_opaque_dart_wire_type(target)
    }
}

pub(super) fn rust_opaque_dart_wire_type(target: Target) -> String {
    match target {
        Target::Io => "int",
        Target::Web => "Object",
    }
    .into()
}
