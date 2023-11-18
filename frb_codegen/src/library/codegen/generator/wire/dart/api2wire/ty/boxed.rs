use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorApi2wireTrait for BoxedWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let as_primitive = self.ir.inner.is_primitive().then(|| {
            format!(
                "return inner.new_{}_{}(api2wire_{}(raw));",
                self.ir.safe_ident(),
                self.context.config.block_index,
                self.ir.inner.safe_ident()
            )
        });
        let ident = self.ir.safe_ident();
        let context = self.context.config.block_index;
        let inner = self.ir.inner.safe_ident();
        let empty_struct = is_empty_struct(self);
        Acc {
            io: Some(as_primitive.unwrap_or_else(|| {
                if self.ir.inner.is_array() {
                    format!("return api2wire_{inner}(raw);")
                } else if empty_struct {
                    format!(
                        "final ptr = inner.new_{ident}_{context}();
                        return ptr;",
                    )
                } else {
                    format!(
                        "final ptr = inner.new_{ident}_{context}();
                        _api_fill_to_wire_{inner}(raw, ptr.ref);
                        return ptr;"
                    )
                }
            })),
            wasm: Some(format!(
                "return api2wire_{}(raw);",
                self.ir.inner.safe_ident()
            )),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        if self.ir.inner.is_array() {
            return Some(format!(
                "wireObj = api2wire_{}(apiObj);",
                self.ir.inner.safe_ident()
            ));
        }
        (!self.ir.inner.is_primitive() && !is_empty_struct(self)).then(|| {
            format!(
                "_api_fill_to_wire_{}(apiObj, wireObj.ref);",
                self.ir.inner.safe_ident()
            )
        })
    }
}
