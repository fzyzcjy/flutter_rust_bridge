use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        let contains_opacity = self
            .ir
            .inner
            .distinct_types(self.context.ir_file)
            .iter()
            .any(IrType::is_opaque);
        Acc {
            io: if contains_opacity {
                Some(format!(
                    "
                final ans = inner.new_{0}_{1}(raw.length);
                try {{
                    for (var i = 0; i < raw.length; ++i) {{
                        _api_fill_to_wire_{2}(raw[i], ans.ref.ptr[i]);
                    }}
                }} catch(e) {{
                    inner.drop_{0}_{1}(ans, raw.length);
                    rethrow;
                }}
                return ans;",
                    self.ir.safe_ident(),
                    self.context.config.block_index,
                    self.ir.inner.safe_ident(),
                ))
            } else {
                Some(format!(
                    "
                    final ans = inner.new_{0}_{1}(raw.length);
                    for (var i = 0; i < raw.length; ++i) {{
                        _api_fill_to_wire_{2}(raw[i], ans.ref.ptr[i]);
                    }}
                    return ans;",
                    self.ir.safe_ident(),
                    self.context.config.block_index,
                    self.ir.inner.safe_ident(),
                ))
            },
            wasm: self.context.config.wasm_enabled.then(|| {
                format!(
                    "return raw.map(api2wire_{}).toList();",
                    self.ir.inner.safe_ident()
                )
            }),
            ..Default::default()
        }
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            self.ir.inner.safe_ident()
        )
    }
}
