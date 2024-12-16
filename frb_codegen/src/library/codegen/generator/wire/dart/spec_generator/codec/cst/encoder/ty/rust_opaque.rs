use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ApiDartGenerator;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for RustOpaqueWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(format!(
            "// ignore: invalid_use_of_internal_member
                return (raw as {}Impl).frbInternalCstEncode();",
            ApiDartGenerator::new(self.mir.clone(), self.context.as_api_dart_context())
                .dart_api_type(),
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        rust_opaque_dart_wire_type(target)
    }
}

pub(super) fn rust_opaque_dart_wire_type(target: Target) -> String {
    match target {
        Target::Io => "int",
        // Target::Web => "Object",
        Target::Web => "int",
    }
    .into()
}
