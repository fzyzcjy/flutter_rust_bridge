use crate::generator::rust::generate_import;
use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeRustGeneratorTrait for TypeOptionalGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: Some(
                if self.ir.inner.is_js_value() {
                    "(!self.is_null() && !self.is_undefined()).then(|| Wire2Api::wire2api(self))"
                } else {
                    "self.map(Wire2Api::wire2api)"
                }
                .into(),
            ),
            // io: Some("(!self.is_null()).then(|| self.wire2api())".into()),
            ..Default::default()
        }
    }

    fn wasm2api_body(&self) -> Option<std::borrow::Cow<str>> {
        (!self.ir.inner.is_primitive() && !self.ir.inner.is_js_value())
            .then(|| "(!self.is_null() && !self.is_undefined()).then(|| self.wire2api())".into())
    }

    fn convert_to_dart(&self, obj: String) -> String {
        let inner = TypeRustGenerator::new(
            *self.ir.inner.clone(),
            self.context.ir_file,
            self.context.config,
        );
        let obj = match inner.wrapper_struct() {
            Some(wrapper) => format!(
                "{}.map(|v| {}({}))",
                obj,
                wrapper,
                inner.self_access("v".to_owned())
            ),
            None => obj,
        };
        format!("{}.into_dart()", obj)
    }

    fn imports(&self) -> Option<String> {
        generate_import(&self.ir.inner, self.context.ir_file, self.context.config)
    }
}
