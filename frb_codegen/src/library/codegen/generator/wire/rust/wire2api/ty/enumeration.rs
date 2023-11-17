use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::enumeration::{IrEnumMode, IrVariant, IrVariantKind};
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorWire2apiTrait for EnumRefWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        if src.mode == IrEnumMode::Simple {
            return None;
        }

        let variants = src.variants();

        let variant_structs = variants
            .iter()
            .map(|variant| self.generate_wire2api_class_variant(&variant))
            .join("\n\n");

        let union_fields = variants
            .iter()
            .map(|variant| format!("{0}: *mut wire_{1}_{0},", variant.name.raw, self.ir.ident.0))
            .join("\n");

        let rust_wire_type = WireRustGenerator::new(self.ir.clone().into(), self.context.clone())
            .rust_wire_type(Target::Io);

        Some(format!(
            "#[repr(C)]
            #[derive(Clone)]
            pub struct {rust_wire_type} {{ tag: i32, kind: *mut {name}Kind }}

            #[repr(C)]
            pub union {name}Kind {{
                {union_fields}
            }}

            {variant_structs}",
            name = self.ir.ident.0,
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let enu = self.ir.get(self.context.ir_pack);
        Acc::new(|target| {
            if matches!(target, Common) {
                return None;
            }
            let wasm = target == Target::Wasm;
            let mut variants =
                (enu.variants())
                    .iter()
                    .enumerate()
                    .map(|(idx, variant)| match &variant.kind {
                        IrVariantKind::Value => {
                            format!("{} => {}::{},", idx, enu.name, variant.name.raw)
                        }
                        IrVariantKind::Struct(st) => {
                            let mut fields = (st.fields).iter().enumerate().map(|(idx, field)| {
                                let field_name = field.name.rust_style();
                                let field_ = if st.is_fields_named {
                                    format!("{field_name}: ")
                                } else {
                                    String::new()
                                };

                                if target != Target::Wasm {
                                    format!("{field_} ans.{field_name}.wire2api()")
                                } else {
                                    format!("{field_} self_.get({}).wire2api()", idx + 1)
                                }
                            });

                            let (left, right) = st.brackets_pair();
                            if target == Target::Wasm {
                                format!(
                                    "{idx} => {{
                                        {enum_name}::{variant_name}{left}{fields}{right} }},",
                                    enum_name = enu.name,
                                    variant_name = variant.name.raw,
                                    fields = fields.join(",")
                                )
                            } else {
                                format!(
                                    "{idx} => unsafe {{
                                        let ans = support::box_from_leak_ptr(self.kind);
                                        let ans = support::box_from_leak_ptr(ans.{variant_name});
                                        {enum_name}::{variant_name}{left}{fields}{right}
                                    }}",
                                    enum_name = enu.name,
                                    variant_name = variant.name.raw,
                                    fields = fields.join(",")
                                )
                            }
                        }
                    });
            Some(format!(
                "{}
                match self{} {{
                    {}
                    _ => unreachable!(),
                }}",
                if wasm {
                    "let self_ = self.unchecked_into::<JsArray>();"
                } else {
                    ""
                },
                if wasm {
                    "_.get(0).unchecked_into_f64() as _"
                } else {
                    ".tag"
                },
                variants.join("\n"),
            ))
        })
    }
}

impl<'a> EnumRefWireRustGenerator<'a> {
    fn generate_wire2api_class_variant(&self, variant: &IrVariant) -> String {
        let fields = match &variant.kind {
            IrVariantKind::Value => vec![],
            IrVariantKind::Struct(s) => s
                .fields
                .iter()
                .map(|field| {
                    let field_generator =
                        WireRustGenerator::new(field.ty.clone(), self.context.clone());
                    format!(
                        "{}: {}{},",
                        field.name.rust_style(),
                        field_generator.rust_wire_modifier(Target::Io),
                        field_generator.rust_wire_type(Target::Io)
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
