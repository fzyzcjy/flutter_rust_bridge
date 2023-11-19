use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorApi2wireTrait for OptionalListWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let inner = self.ir.inner.safe_ident();
        Acc {
            io: Some(format!(
                "final ans = inner.new_{safe_ident}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    final item = raw[i];
                    if (item == null) continue;
                    ans.ref.ptr[i] = api2wire_{inner}(item);
                }}
                return ans;",
                safe_ident = self.ir.safe_ident(),
            )),
            wasm: (self.context.config.wasm_enabled)
                .then(|| format!("return mapNonNull(raw, api2wire_{inner});")),
            ..Default::default()
        }
    }
}
