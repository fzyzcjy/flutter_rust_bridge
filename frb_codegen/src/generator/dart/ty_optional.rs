use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::{Acc, Target};
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeDartGeneratorTrait for TypeOptionalGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            Target::Io | Target::Wasm => Some(format!(
                "return raw == null ? {} : api2wire_{}(raw);",
                if target.is_wasm() {
                    if self.ir.is_primitive() || self.ir.is_boxed_primitive() {
                        "0"
                    } else {
                        "null"
                    }
                } else {
                    "ffi.nullptr"
                },
                self.ir.inner.safe_ident()
            )),
            _ => None,
        })
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
                "if (raw != null) {{_api_validate_{}(raw);}}",
                self.ir.inner.safe_ident()
            ))
        } else {
            None
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        if !self.ir.needs_initialization() || self.ir.is_list() || self.ir.is_boxed_primitive() {
            return None;
        }
        Some(format!(
            "if (apiObj != null) _api_fill_to_wire_{}(apiObj, wireObj);",
            self.ir.inner.safe_ident()
        ))
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return raw == null ? null : _wire2api_{}(raw);",
            self.ir.inner.safe_ident()
        )
    }
}
