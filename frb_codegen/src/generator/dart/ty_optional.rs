use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::{Acc, Target};
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeDartGeneratorTrait for TypeOptionalGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let prefix = if !self.context.config.shared
            && self
                .get_context()
                .all_configs
                .is_type_shared(&self.ir.inner, true)
        {
            "_sharedPlatform.api2wire"
        } else {
            "api2wire"
        };
        Acc::new(|target| match target {
            Target::Io | Target::Wasm => Some(format!(
                "return raw == null ? {} : {prefix}_{}(raw);",
                if target.is_wasm() {
                    "null"
                } else {
                    "ffi.nullptr"
                },
                self.ir.inner.safe_ident()
            )),
            _ => None,
        })
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        if !self.ir.needs_initialization() || self.ir.is_list() || self.ir.is_boxed_primitive() {
            return None;
        }

        let prefix = if self.context.config.shared {
            ""
        } else if self
            .get_context()
            .all_configs
            .is_type_shared(&self.ir.inner, true)
        {
            "_sharedPlatform."
        } else {
            "_"
        };

        Some(format!(
            "if (apiObj != null) {}api_fill_to_wire_{}(apiObj, wireObj);",
            prefix,
            self.ir.inner.safe_ident()
        ))
    }

    fn wire2api_body(&self) -> String {
        let use_shared_instance = !self.context.config.shared
            && self
                .get_context()
                .all_configs
                .is_type_shared(&self.ir.inner, true);

        if use_shared_instance {
            format!(
                "return raw == null ? null : _sharedImpl.wire2api_{}(raw);",
                self.ir.inner.safe_ident()
            )
        } else {
            let prefix = if self.context.config.shared { "" } else { "_" };
            format!(
                "return raw == null ? null : {}wire2api_{}(raw);",
                prefix,
                self.ir.inner.safe_ident()
            )
        }
    }
}
