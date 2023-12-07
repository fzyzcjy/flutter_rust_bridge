use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::misc::dart_wire_type_from_rust_wire_type_or_wasm;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for RustOpaqueWireDartCodecCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(format!(
            "// ignore: invalid_use_of_internal_member
            return raw.cstEncode();",
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_or_rust_opaque_dart_wire_type(target)
    }
}

pub(super) fn dart_or_rust_opaque_dart_wire_type(target: Target) -> String {
    match target {
        Target::Io => "PlatformPointer",
        Target::Wasm => "Object",
    }
    .into()
}
