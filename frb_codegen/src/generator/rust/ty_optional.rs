use crate::generator::rust::generate_import;
use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeRustGeneratorTrait for TypeOptionalGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: (!self.ir.inner.is_js_value()
                && !self.ir.is_primitive()
                && !self.ir.is_boxed_primitive())
            .then(|| "self.map(Wire2Api::wire2api)".into()),
            ..Default::default()
        }
    }

    fn convert_to_dart(&self, obj: String) -> String {
        let inner = TypeRustGenerator::new(
            *self.ir.inner.clone(),
            self.context.config,
            self.context.all_configs,
        );
        let obj = match inner.wrapper_struct() {
            // An architecture has been created so that the inner type of optional field is always
            // IrTypeBoxed. Here, too, if we use inner.self_access("v".to_owned()), since it will go to
            // self_access in TypeBoxedGenerator, the dereferenced value *v was returned from there, which
            // gave the error that the external struct could not be dereferenced.
            //
            // For now, removed the parameter inner.self_access("v".to_owned()) and then added a bit of a
            // hacky method here (not used self_access function), as it only covers mirrored optional fields.
            Some(wrapper) => format!("{}.map(|v| {}(v))", obj, wrapper),
            None => obj,
        };
        format!("{obj}.into_dart()")
    }

    fn imports(&self) -> Option<String> {
        generate_import(
            &self.ir.inner,
            self.context.config,
            self.context.all_configs,
        )
    }
}
