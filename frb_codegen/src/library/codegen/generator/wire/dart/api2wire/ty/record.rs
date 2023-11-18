use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;

impl<'a> WireDartGeneratorApi2wireTrait for RecordWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, ty)| format!("api2wire_{}(raw.${})", ty.safe_ident(), idx + 1))
            .collect_vec()
            .join(",");
        Acc {
            wasm: Some(format!("return [{values}];")),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let ir = self.ir.inner.get(self.context.ir_pack);
        let values = ir
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                super::ty_struct::api_fill_for_field(
                    &field.ty.safe_ident(),
                    &format!("${}", idx + 1),
                    field.name.rust_style(),
                    field.ty.is_struct(),
                )
            })
            .collect_vec()
            .join("\n");
        Some(values)
    }
}
