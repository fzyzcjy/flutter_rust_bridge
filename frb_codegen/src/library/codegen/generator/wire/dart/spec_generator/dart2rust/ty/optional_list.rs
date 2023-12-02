use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::general_list::general_or_optional_list_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorDart2RustTrait for OptionalListWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let inner = self.ir.inner.safe_ident();
        Acc {
            io: Some(format!(
                "final ans = wire.new_{safe_ident}(raw.length);
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

    fn dart_wire_type(&self, target: Target) -> String {
        general_or_optional_list_dart_wire_type(target, &self.ir.clone().into())
    }
}
