use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::ir::ty::IrType::StructRef;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorApi2wireTrait for BoxedWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let ir_safe_ident = self.ir.safe_ident();
        let inner_safe_ident = self.ir.inner.safe_ident();
        let empty_struct = is_empty_struct(self);

        Acc {
            io: Some(if self.ir.inner.is_primitive() {
                format!("return inner.new_{ir_safe_ident}(api2wire_{inner_safe_ident}(raw));")
            } else if self.ir.inner.is_array() {
                format!("return api2wire_{inner_safe_ident}(raw);")
            } else {
                format!(
                    "final ptr = inner.new_{ir_safe_ident}();
                    {},
                    return ptr;",
                    if empty_struct {
                        "".to_owned()
                    } else {
                        format!("_api_fill_to_wire_{inner_safe_ident}(raw, ptr.ref);")
                    }
                )
            }),
            wasm: Some(format!("return api2wire_{inner_safe_ident}(raw);")),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let inner_safe_ident = self.ir.inner.safe_ident();

        if self.ir.inner.is_array() {
            Some(format!("wireObj = api2wire_{inner_safe_ident}(apiObj);"))
        } else if !self.ir.inner.is_primitive() && !is_empty_struct(self) {
            Some(format!(
                "_api_fill_to_wire_{inner_safe_ident}(apiObj, wireObj.ref);"
            ))
        } else {
            None
        }
    }
}

fn is_empty_struct(ty: &BoxedWireDartGenerator) -> bool {
    if let StructRef(ref s) = ty.ir.inner.as_ref() {
        s.get(ty.context.ir_pack).fields.is_empty()
    } else {
        false
    }
}
