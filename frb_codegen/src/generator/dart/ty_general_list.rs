use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        Acc {
            io: Some(format!(
                "final ans = inner.new_{}_{}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;",
                self.ir.safe_ident(),
                self.context.config.block_index,
                self.ir.inner.safe_ident()
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

    fn api_validate(&self) -> Option<String> {
        let ir_file = self.context.ir_file;
        let config = self.context.config;

        self.ir.inner.visit_types(
            &mut |ty| {
                let ident = ty.safe_ident();
                let mut lock = REQUIRES_VALIDATION.lock().unwrap();
                let cache = lock.get_mut(&ident);
                if cache.is_some() {
                    true
                } else {
                    lock.insert(ident.clone(), false);
                    drop(lock);

                    let res = TypeDartGenerator::new(ty.clone(), ir_file, config)
                        .api_validate()
                        .is_some();
                    REQUIRES_VALIDATION.lock().unwrap().insert(ident, res);

                    res
                }
            },
            ir_file,
        );

        if REQUIRES_VALIDATION
            .lock()
            .unwrap()
            .get(&self.ir.inner.safe_ident())
            .copied()
            .unwrap_or_default()
        {
            Some(format!(
                "for (var i = 0; i < raw.length; ++i) {{
                    _api_validate_{}(raw[i]);
                }}",
                self.ir.inner.safe_ident()
            ))
        } else {
            None
        }
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            self.ir.inner.safe_ident()
        )
    }
}
