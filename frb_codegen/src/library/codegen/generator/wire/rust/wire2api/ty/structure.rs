use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorWire2apiTrait for StructRefWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        let s = self.ir.get(self.context.ir_pack);
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &s.fields
                .iter()
                .map(|field| {
                    let field_generator =
                        WireRustGenerator::new(field.ty.clone(), self.context.clone());
                    format!(
                        "{}: {}{}",
                        field.name.rust_style(),
                        field_generator.rust_wire_modifier(Target::Io),
                        field_generator.rust_wire_type(Target::Io)
                    )
                })
                .collect_vec(),
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let api_struct = self.ir.get(self.context.ir_pack);
        let fields: Acc<Vec<_>> = api_struct
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let field_name = field.name.rust_style();
                let field_ = if api_struct.is_fields_named {
                    format!("{field_name}: ")
                } else {
                    String::new()
                };

                Acc {
                    wasm: format!("{field_} self_.get({idx}).wire2api()"),
                    io: format!("{field_} self.{field_name}.wire2api()"),
                    ..Default::default()
                }
            })
            .collect();

        let (left, right) = api_struct.brackets_pair();
        let rust_api_type = self.ir.rust_api_type();
        Acc {
            io: Some(format!(
                "
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.io.join(","),
            )),
            wasm: Some(format!(
                "
                let self_ = self.dyn_into::<JsArray>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.wasm.join(","),
                len = api_struct.fields.len(),
            )),
            ..Default::default()
        }
    }
}
