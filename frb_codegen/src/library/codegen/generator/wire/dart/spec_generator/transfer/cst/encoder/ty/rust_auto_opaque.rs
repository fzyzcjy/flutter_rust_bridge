use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::misc::dart_wire_type_from_rust_wire_type_or_wasm;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::rust_opaque::dart_or_rust_opaque_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::ty::ownership::IrTypeOwnershipMode;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for RustAutoOpaqueWireDartCodecCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        let enable_move = self.ir.ownership_mode == IrTypeOwnershipMode::Owned;
        Acc::new_common(Some(format!(
            "// ignore: invalid_use_of_internal_member
            return raw.cstEncode(move: {enable_move});",
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_or_rust_opaque_dart_wire_type(target)
    }
}
