use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorApi2wireTrait for RustOpaqueWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "final ptr = inner.new_{0}();
                _api_fill_to_wire_{0}(raw, ptr);
                return ptr;",
                self.ir.safe_ident(),
            )),
            wasm: Some("return raw.shareOrMove();".to_owned()),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some("wireObj.ptr = apiObj.shareOrMove();".into())
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => {
                WireRustGenerator::new(self.ir.clone(), self.context.as_wire_rust_context())
                    .rust_wire_type(target)
            }
            Target::Wasm => "Object".into(),
        }
    }
}
