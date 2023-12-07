use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartTransferCstGeneratorEncoderTrait for OptionalWireDartTransferCstGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Wasm => Some(format!(
                "return raw == null ? {} : api2wire_{}(raw);",
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
                WireDartGenerator::new(self.ir.inner.clone(), self.context).dart_wire_type(target)
            )
        } else {
            WireDartGenerator::new(self.ir.inner.clone(), self.context).dart_wire_type(target)
        }
    }
}
