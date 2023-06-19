use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

use super::ExternFuncCollector;
use super::TypeStructRefGenerator;

type_rust_generator_struct!(TypeRecordGenerator, IrTypeRecord);

impl TypeRecordGenerator<'_> {
    fn as_struct_generator(&self) -> TypeStructRefGenerator {
        TypeStructRefGenerator {
            ir: self.ir.inner.clone(),
            context: self.context.clone(),
        }
    }
}

impl TypeRustGeneratorTrait for TypeRecordGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        let ir = self.ir.inner.get(self.context.ir_file);
        let len = ir.fields.len();
        let values: Acc<Vec<_>> = ir
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| Acc {
                wasm: format!("self_.get({idx}).wire2api()"),
                io: format!("self.{}.wire2api()", field.name.rust_style()),
                ..Default::default()
            })
            .collect();
        Acc {
            io: Some(format!("({},)", values.io.join(","))),
            wasm: Some(format!(
                "let self_ = self.dyn_into::<JsArray>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                ({},)",
                values.wasm.join(",")
            )),
            ..Default::default()
        }
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        self.as_struct_generator().wire_struct_fields()
    }

    fn static_checks(&self) -> Option<String> {
        self.as_struct_generator().static_checks()
    }

    fn wrapper_struct(&self) -> Option<String> {
        self.as_struct_generator().wrapper_struct()
    }

    fn wrap_obj(&self, obj: String, wired_fallible_func: bool) -> String {
        self.as_struct_generator()
            .wrap_obj(obj, wired_fallible_func)
    }

    fn new_with_nullptr(&self, collector: &mut ExternFuncCollector) -> String {
        self.as_struct_generator().new_with_nullptr(collector)
    }

    fn get_context(&self) -> &TypeGeneratorContext {
        &self.context
    }
}
