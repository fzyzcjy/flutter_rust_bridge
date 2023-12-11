use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{ExternClass, ExternClassMode, ExternFunc};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumMode, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::rust_wire_type_add_prefix_or_js_value;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for EnumRefWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        let src = self.ir.get(self.context.ir_pack);
        if src.mode == IrEnumMode::Simple {
            return None;
        }

        let variants = src.variants();

        let union_fields = variants
            .iter()
            .map(|variant| {
                format!(
                    "{0}: *mut wire_cst_{1}_{0},",
                    variant.name, self.ir.ident.0.name
                )
            })
            .join("\n");

        let rust_wire_type = self.rust_wire_type(Target::Io);
        let name = &self.ir.ident.0.name;

        let mut extern_classes = vec![
            ExternClass {
                name: rust_wire_type,
                mode: ExternClassMode::Struct,
                body: format!("tag: i32, kind: *mut {name}Kind",),
            },
            ExternClass {
                name: format!("{name}Kind"),
                mode: ExternClassMode::Union,
                body: union_fields,
            },
        ];

        extern_classes
            .extend((variants.iter()).map(|variant| self.generate_decoder_class_variant(variant)));

        Some(WireRustOutputCode {
            extern_classes,
            ..Default::default()
        })
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        let enu = self.ir.get(self.context.ir_pack);
        Acc::new(|target| {
            if matches!(target, TargetOrCommon::Common) {
                return None;
            }

            let wasm = target == TargetOrCommon::Wasm;
            let variants = enu
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    generate_impl_cst_decode_body_variant(enu, target, idx, variant)
                })
                .join("\n");

            Some(format!(
                "{}match self{} {{
                    {}
                    _ => unreachable!(),
                }}",
                if wasm {
                    "let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();"
                } else {
                    ""
                },
                if wasm {
                    "_.get(0).unchecked_into_f64() as _"
                } else {
                    ".tag"
                },
                variants,
            ))
        })
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        let src = self.ir.get(self.context.ir_pack);

        let inflators = src
            .variants()
            .iter()
            .filter_map(|variant| self.generate_impl_new_with_nullptr_variant(variant))
            .collect_vec();

        Some(WireRustOutputCode {
            body: generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                self.context,
                "Self { tag: -1, kind: core::ptr::null_mut() }",
                true,
            ),
            extern_funcs: inflators,
            ..Default::default()
        })
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> EnumRefWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class_variant(&self, variant: &IrVariant) -> ExternClass {
        let fields = match &variant.kind {
            IrVariantKind::Value => vec![],
            IrVariantKind::Struct(s) => s
                .fields
                .iter()
                .map(|field| {
                    let field_generator =
                        WireRustCodecCstGenerator::new(field.ty.clone(), self.context);
                    format!(
                        "{}: {}{},",
                        field.name.rust_style(),
                        field_generator.rust_wire_modifier(Target::Io),
                        field_generator.rust_wire_type(Target::Io)
                    )
                })
                .collect(),
        };
        ExternClass {
            name: format!("wire_cst_{}_{}", self.ir.ident.0.name, variant.name,),
            mode: ExternClassMode::Struct,
            body: fields.join("\n"),
        }
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
            func_name: format!("cst_inflate_{typ}"),
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
                format_args!("wire_cst_{typ}"),
                body.join(",")
            ),
            target: Target::Io,
        })
    }

    fn generate_impl_new_with_nullptr_variant_field(&self, field: &IrField) -> String {
        let ty_generator = WireRustCodecCstGenerator::new(field.ty.clone(), self.context);

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

fn generate_impl_cst_decode_body_variant(
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
                        format!("{field_} ans.{field_name}.cst_decode()")
                    } else {
                        format!("{field_} self_.get({}).cst_decode()", idx + 1)
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
