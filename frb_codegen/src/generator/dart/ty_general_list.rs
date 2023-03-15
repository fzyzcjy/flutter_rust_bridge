use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        let ident = self.ir.safe_ident();
        let context = self.context.config.block_index;
        let inner = self.ir.inner.safe_ident();

        let io = if self.ir.inner.is_primitive() {
            // Handle primitive enums list.
            // This is similar to `StringList` in
            // `frb_codegen/src/generator/dart/ty_delegate.rs`
            format!(
                "final ans = inner.new_{}_{}(raw.length);
                for (var i = 0; i < raw.length; i++){{
                    ans.ref.ptr[i] = api2wire_{}(raw[i]);
                }}
                return ans;",
                self.ir.safe_ident(),
                self.context.config.block_index,
                self.ir.inner.safe_ident()
            )
        } else {
            format!(
                "final ans = inner.new_{ident}_{context}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{inner}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;
                "
            )
        };

        Acc {
            io: Some(io),
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
