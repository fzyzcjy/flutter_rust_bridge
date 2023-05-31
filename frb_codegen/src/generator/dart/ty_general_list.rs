use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn api2wire_body(
        &self,
        _shared_dart_api2wire_funcs: &Option<Acc<String>>,
    ) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        let ident = self.ir.safe_ident();
        let inner = self.ir.inner.safe_ident();

        Acc {
            io: Some(format!(
                "final ans = inner.new_{ident}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    {}
                }}
                return ans;
                ",
                if self.ir.inner.is_primitive() {
                    // Handle primitive enums list.
                    // This is similar to `StringList` in
                    // `frb_codegen/src/generator/dart/ty_delegate.rs`
                    format!("ans.ref.ptr[i] = api2wire_{inner}(raw[i]);")
                } else {
                    let prefix = if !self.context.config.shared { "_" } else { "" };
                    format!("{prefix}api_fill_to_wire_{inner}(raw[i], ans.ref.ptr[i]);")
                }
            )),
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
            "return (raw as List<dynamic>).map({}wire2api_{}).toList();",
            self.get_private_prefix(),
            self.ir.inner.safe_ident()
        )
    }

    fn get_context(&self) -> &TypeGeneratorContext {
        &self.context
    }
}
