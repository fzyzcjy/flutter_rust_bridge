use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::misc::dart_wire_type_from_rust_wire_type_or_wasm;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorDart2RustTrait for RustOpaqueWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "final ptr = wire.new_{0}();
                ptr.ptr = raw.shareOrMove();
                return ptr;",
                self.ir.safe_ident(),
            )),
            wasm: Some(
                "// ignore: invalid_use_of_internal_member
                return raw.shareOrMove();"
                    .to_owned(),
            ),
            ..Default::default()
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_wire_type_from_rust_wire_type_or_wasm(self, target, "Object".into())
    }
}
