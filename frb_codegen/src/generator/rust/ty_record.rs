use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;

use super::ExternFuncCollector;

type_rust_generator_struct!(TypeRecordGenerator, IrTypeRecord);

impl TypeRustGeneratorTrait for TypeRecordGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        let len = self.ir.values.len();
        let fields: Acc<Vec<_>> = (0..self.ir.values.len())
            .map(|idx| Acc {
                wasm: format!("self_.get({idx}).wire2api()"),
                io: format!("self.field{idx}.wire2api()"),
                ..Default::default()
            })
            .collect();
        Acc {
            io: Some(format!("({},)", fields.io.join(","))),
            wasm: Some(format!(
                "let self_ = self.dyn_into::<JsArray>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                ({},)",
                fields.wasm.join(",")
            )),
            ..Default::default()
        }
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                format!(
                    "field{idx}: {}{}",
                    value.rust_wire_modifier(Target::Io),
                    value.rust_wire_type(Target::Io)
                )
            })
            .collect();
        Some(values)
    }

    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        let wire = self.ir.rust_wire_type(Target::Io);
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                let value = if value.rust_wire_is_pointer(Target::Io) {
                    "core::ptr::null_mut()".to_owned()
                } else if value.is_rust_opaque() || value.is_dart_opaque() {
                    format!("{}::new_with_null_ptr()", value.rust_wire_type(Target::Io))
                } else {
                    "Default::default()".to_owned()
                };
                format!("field{idx}: {value}")
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "impl NewWithNullPtr for {wire} {{
                fn new_with_null_ptr() -> Self {{
                    Self {{ {values} }}
                }}
            }}

            impl Default for {wire} {{
                fn default() -> Self {{
                    Self::new_with_null_ptr()
                }}
            }}"
        )
    }
}
