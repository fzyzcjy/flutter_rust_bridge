use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{ExternClass, ExternClassMode};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumMode, MirEnumVariant, MirVariantKind};
use crate::codegen::ir::mir::ty::structure::MirStruct;
use itertools::Itertools;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::rust_wire_type_add_prefix_or_js_value;

const NIL_FIELD: &str = "nil__";

impl<'a> WireRustCodecCstGeneratorDecoderTrait for EnumRefWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        let src = self.mir.get(self.context.mir_pack);
        if src.mode == MirEnumMode::Simple {
            return None;
        }
        // frb-coverage:ignore-start
        // This blank line is weirdly reported as non-covered, so we ignore it
        // frb-coverage:ignore-end
        let variants = src.variants();

        let union_fields = variants
            .iter()
            .filter_map(|variant| match variant.kind {
                MirVariantKind::Value => None,
                MirVariantKind::Struct(_) => Some(format!(
                    "{0}: wire_cst_{1}_{0},",
                    variant.name, self.mir.ident.0.name
                )),
            })
            .chain(Some(format!("{NIL_FIELD}: (),")))
            .join("\n");

        let rust_wire_type = self.rust_wire_type(Target::Io);
        let name = &self.mir.ident.0.name;
        let union_kind = format!("{name}Kind");

        let mut extern_classes = vec![
            ExternClass {
                name: rust_wire_type,
                mode: ExternClassMode::Struct,
                body: format!("tag: i32, kind: {union_kind},"),
                needs_ffigen: true,
            },
            ExternClass {
                name: union_kind,
                mode: ExternClassMode::Union,
                body: union_fields,
                needs_ffigen: true,
            },
        ];

        extern_classes.extend(variants.iter().filter_map(|variant| match &variant.kind {
            MirVariantKind::Value => None,
            MirVariantKind::Struct(s) => Some(self.generate_decoder_enum_variant(&variant.name, s)),
        }));

        Some(WireRustOutputCode {
            extern_classes,
            ..Default::default()
        })
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        let enu = self.mir.get(self.context.mir_pack);
        Acc::new(|target| {
            if matches!(target, TargetOrCommon::Common) {
                return None;
            }

            let web = target == TargetOrCommon::Web;
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
                if web {
                    "let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();"
                } else {
                    ""
                },
                if web {
                    "_.get(0).unchecked_into_f64() as _"
                } else {
                    ".tag"
                },
                variants,
            ))
        })
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        let name = &self.mir.ident.0.name;
        Some(WireRustOutputCode {
            body: generate_impl_new_with_nullptr_code_block(
                self.mir.clone(),
                self.context,
                &format!("Self {{ tag: -1, kind: {name}Kind {{ {NIL_FIELD}: () }} }}"),
            ),
            ..Default::default()
        })
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.mir, target)
    }
}

impl<'a> EnumRefWireRustCodecCstGenerator<'a> {
    fn generate_decoder_enum_variant(
        &self,
        variant_name: &MirIdent,
        r#struct: &MirStruct,
    ) -> ExternClass {
        let fields = r#struct
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
            .collect::<Vec<_>>();
        ExternClass {
            name: format!("wire_cst_{}_{}", self.mir.ident.0.name, variant_name),
            mode: ExternClassMode::Struct,
            body: fields.join("\n"),
            needs_ffigen: true,
        }
    }
}

fn generate_impl_cst_decode_body_variant(
    enu: &MirEnum,
    target: TargetOrCommon,
    idx: usize,
    variant: &MirEnumVariant,
) -> String {
    match &variant.kind {
        MirVariantKind::Value => {
            format!("{} => {}::{},", idx, enu.name.rust_style(), variant.name)
        }
        MirVariantKind::Struct(st) => {
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

                    if target != TargetOrCommon::Web {
                        format!("{field_} ans.{field_name}.cst_decode()")
                    } else {
                        format!("{field_} self_.get({}).cst_decode()", idx + 1)
                    }
                })
                .join(",");

            let (left, right) = st.brackets_pair();
            let enum_name = &enu.name.rust_style();
            let variant_name = &variant.name;

            if target == TargetOrCommon::Web {
                format!("{idx} => {{ {enum_name}::{variant_name}{left}{fields}{right} }},")
            } else {
                format!(
                    "{idx} => {{
                        let ans = unsafe {{ self.kind.{variant_name} }};
                        {enum_name}::{variant_name}{left}{fields}{right}
                    }}",
                )
            }
        }
    }
}
