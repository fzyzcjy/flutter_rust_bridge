use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::class::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::class::ty::WireRustClassGeneratorClassTrait;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> WireRustClassGeneratorClassTrait for StructRefWireRustGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
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
}
