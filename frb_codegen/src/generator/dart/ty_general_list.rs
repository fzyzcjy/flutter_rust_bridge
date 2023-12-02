use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

use super::func::{get_api2wire_prefix, get_api_to_fill_wire_prefix};

type_dart_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        let ident = self.ir.safe_ident();
        let inner = self.ir.inner.safe_ident();
        let api2wire_prefix = get_api2wire_prefix(
            &format!("api2wire_{}", inner),
            self.context.config,
            &self.ir.inner,
            false,
            self.get_context().all_configs,
        );
        let api_fill_to_wire_prefix = get_api_to_fill_wire_prefix(
            self.context.config,
            &self.ir.inner,
            self.get_context().all_configs,
        );
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
                    format!("ans.ref.ptr[i] = {api2wire_prefix}api2wire_{inner}(raw[i]);")
                } else {
                    format!("{api_fill_to_wire_prefix}api_fill_to_wire_{inner}(raw[i], ans.ref.ptr[i]);")
                }
            )),
            wasm: self.context.config.wasm_enabled.then(|| {
                format!(
                    "return raw.map({api2wire_prefix}api2wire_{}).toList();",
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
}
