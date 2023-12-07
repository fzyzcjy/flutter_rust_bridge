use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for OptionalWireDartCodecCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Wasm => Some(format!(
                "return raw == null ? {} : cst_encode_{}(raw);",
                if target == TargetOrCommon::Wasm {
                    "null"
                } else {
                    "ffi.nullptr"
                },
                self.ir.inner.safe_ident()
            )),
            _ => None,
        })
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm {
            format!(
                "{}?",
                WireDartCodecCstGenerator::new(self.ir.inner.clone(), self.context)
                    .dart_wire_type(target)
            )
        } else {
            WireDartCodecCstGenerator::new(self.ir.inner.clone(), self.context)
                .dart_wire_type(target)
        }
    }
}
