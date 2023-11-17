use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

use super::ExternFuncCollector;
use super::TypeStructRefGenerator;

type_rust_generator_struct!(TypeRecordGenerator, IrTypeRecord);

impl TypeRustGeneratorTrait for TypeRecordGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        let ir = self.ir.inner.get(self.context.ir_pack);
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

    fn new_with_nullptr(&self, collector: &mut ExternFuncCollector) -> String {
        self.as_struct_generator().new_with_nullptr(collector)
    }
}
