use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::misc::rust_wire_type_add_prefix_or_js_value;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumMode, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

impl<'a> WireRustGeneratorDart2RustTrait for EnumRefWireRustGenerator<'a> {}

impl<'a> EnumRefWireRustGenerator<'a> {
    fn generate_wire2api_class_variant(&self, variant: &IrVariant) -> String {
        let fields = match &variant.kind {
            IrVariantKind::Value => vec![],
            IrVariantKind::Struct(s) => s
                .fields
                .iter()
                .map(|field| {
                    let field_generator = WireRustGenerator::new(field.ty.clone(), self.context);
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
            self.ir.ident.0.name,
            variant.name,
            fields.join("\n")
        )
    }

    fn generate_impl_new_with_nullptr_variant(&self, variant: &IrVariant) -> Option<ExternFunc> {
        let typ = format!("{}_{}", self.ir.ident.0.name, variant.name);
        let body = if let IrVariantKind::Struct(st) = &variant.kind {
            st.fields
                .iter()
                .map(|field| self.generate_impl_new_with_nullptr_variant_field(field))
                .collect_vec()
        } else {
            return None;
        };

        Some(ExternFunc {
            func_name: format!("inflate_{typ}"),
            params: vec![],
            return_type: Some(format!("*mut {}Kind", self.ir.ident.0.name)),
            body: format!(
                "flutter_rust_bridge::for_generated::new_leak_box_ptr({}Kind {{
                    {}: flutter_rust_bridge::for_generated::new_leak_box_ptr({} {{
                        {}
                    }})
                }})",
                self.ir.ident.0.name,
                variant.name.rust_style(),
                format_args!("wire_{typ}"),
                body.join(",")
            ),
            target: Target::Io,
        })
    }

    fn generate_impl_new_with_nullptr_variant_field(&self, field: &IrField) -> String {
        let ty_generator = WireRustGenerator::new(field.ty.clone(), self.context);

        let init = if ty_generator.rust_wire_is_pointer(Target::Io)
            || matches!(field.ty, IrType::RustOpaque(_) | IrType::DartOpaque(_))
        {
            "core::ptr::null_mut()".to_owned()
        } else {
            "Default::default()".to_owned()
        };

        format!("{field_name}: {init}", field_name = field.name.rust_style())
    }
}

fn generate_impl_wire2api_body_variant(
    enu: &IrEnum,
    target: TargetOrCommon,
    idx: usize,
    variant: &IrVariant,
) -> String {
    match &variant.kind {
        IrVariantKind::Value => {
            format!("{} => {}::{},", idx, enu.name.rust_style(), variant.name)
        }
        IrVariantKind::Struct(st) => {
            let fields = st
                .fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    let field_name = field.name.rust_style();
                    let field_ = if st.is_fields_named {
                        format!("{field_name}: ")
                    } else {
                        String::new()
                    };

                    if target != TargetOrCommon::Wasm {
                        format!("{field_} ans.{field_name}.wire2api()")
                    } else {
                        format!("{field_} self_.get({}).wire2api()", idx + 1)
                    }
                })
                .join(",");

            let (left, right) = st.brackets_pair();
            let enum_name = &enu.name.rust_style();
            let variant_name = &variant.name;

            if target == TargetOrCommon::Wasm {
                format!("{idx} => {{ {enum_name}::{variant_name}{left}{fields}{right} }},")
            } else {
                format!(
                    "{idx} => unsafe {{
                        let ans = flutter_rust_bridge::for_generated::box_from_leak_ptr(self.kind);
                        let ans = flutter_rust_bridge::for_generated::box_from_leak_ptr(ans.{variant_name});
                        {enum_name}::{variant_name}{left}{fields}{right}
                    }}",
                )
            }
        }
    }
}
