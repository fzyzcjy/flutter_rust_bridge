use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;

pub(super) fn generate_class_from_fields(
    ty: impl Into<IrType>,
    context: WireRustGeneratorContext,
    fields: &[String],
) -> String {
    let struct_name = WireRustGenerator::new(ty.into(), context).rust_wire_type(Target::Io);
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
