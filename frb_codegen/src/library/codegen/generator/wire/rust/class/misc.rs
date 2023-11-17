use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::rust_wire_type::WireRustGeneratorRustWireTypeTrait;

pub(super) fn generate_class_from_fields(
    ty: &IrType,
    fields: &[String],
    context: &WireRustGeneratorContext,
) -> String {
    let struct_name =
        WireRustGenerator::new(ty.clone(), context.clone()).rust_wire_type(Target::Io);
    format!(
        r###"
            #[repr(C)]
            #[derive(Clone)]
            pub struct {struct_name} {{
                {fields}
            }}
        "###,
        struct_name = struct_name,
        fields = fields.join(",\n"),
    )
}
