use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for OptionalWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Web => Some(format!(
                "return raw == null ? {} : cst_encode_{}(raw);",
                if target == TargetOrCommon::Web {
                    "null"
                } else {
                    "ffi.nullptr"
                },
                self.mir.inner.safe_ident()
            )),
            _ => None,
        })
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Web {
            format!(
                "{}?",
                WireDartCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                    .dart_wire_type(target)
            )
        } else {
            WireDartCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                .dart_wire_type(target)
        }
    }
}
