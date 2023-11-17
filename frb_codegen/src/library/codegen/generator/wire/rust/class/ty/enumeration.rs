use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::class::ty::WireRustClassGeneratorClassTrait;
use crate::codegen::ir::ty::enumeration::{IrEnumMode, IrVariant, IrVariantKind};
use itertools::Itertools;

impl<'a> WireRustClassGeneratorClassTrait for EnumRefWireRustGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        if src.mode == IrEnumMode::Simple {
            return None;
        }

        let variants = src.variants();

        let variant_structs = variants
            .iter()
            .map(|variant| self.generate_variant(&variant))
            .join("\n\n");

        let union_fields = variants
            .iter()
            .map(|variant| format!("{0}: *mut wire_{1}_{0},", variant.name.raw, self.ir.ident.0))
            .join("\n");

        Some(format!(
            "#[repr(C)]
            #[derive(Clone)]
            pub struct {rust_wire_type} {{ tag: i32, kind: *mut {name}Kind }}

            #[repr(C)]
            pub union {name}Kind {{
                {union_fields}
            }}

            {variant_structs}",
            rust_wire_type = self.ir.rust_wire_type(Target::Io),
            name = self.ir.ident.0,
        ))
    }
}

impl<'a> EnumRefWireRustGenerator<'a> {
    fn generate_variant(&self, variant: &IrVariant) -> String {
        let fields = match &variant.kind {
            IrVariantKind::Value => vec![],
            IrVariantKind::Struct(s) => s
                .fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {}{},",
                        field.name.rust_style(),
                        field.ty.rust_wire_modifier(Target::Io),
                        field.ty.rust_wire_type(Target::Io)
                    )
                })
                .collect(),
        };
        format!(
            "#[repr(C)]
            #[derive(Clone)]
            pub struct wire_{}_{} {{ {} }}",
            self.ir.ident.0,
            variant.name.raw,
            fields.join("\n")
        )
    }
}
