use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeOptionalListGenerator, IrTypeOptionalList);

impl TypeDartGeneratorTrait for TypeOptionalListGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let inner = self.ir.inner.safe_ident();
        Acc {
            io: Some(format!(
                "final ans = inner.new_{safe_ident}_{idx}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    final item = raw[i];
                    if (item == null) continue;
                    ans.ref.ptr[i] = api2wire_{inner}(item);
                }}
                return ans;",
                safe_ident = self.ir.safe_ident(),
                idx = self.context.config.block_index,
            )),
            wasm: (self.context.config.wasm_enabled)
                .then(|| format!("return mapNonNull(raw, api2wire_{inner});")),
            ..Default::default()
        }
    }
}
