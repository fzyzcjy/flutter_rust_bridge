use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;

impl<'a> WireRustGeneratorWire2apiTrait for OptionalListWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &vec![
                format!(
                    "ptr: *mut *mut {}",
                    WireRustGenerator::new(*self.ir.inner.clone(), self.context.clone())
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: Some(TypeGeneralListGenerator::WIRE2API_BODY_WASM.to_owned()),
            io: Some(TypeGeneralListGenerator::WIRE2API_BODY_IO.to_owned()),
            ..Default::default()
        }
    }
}
