use crate::codegen::ir::ty::IrType;

pub(super) fn generate_class_from_fields(ty: &IrType, fields: &[String]) -> String {
    format!(
        r###"
            #[repr(C)]
            #[derive(Clone)]
            pub struct {struct_name} {{
                {fields}
            }}
        "###,
        struct_name = ty.rust_wire_type(Target::Io),
        fields = fields.join(",\n"),
    )
}
