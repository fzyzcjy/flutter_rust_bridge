use crate::codegen::generator::codec::sse::ty::structure::GeneralizedStructGenerator;
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::misc::StructOrRecord;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrVariant, IrVariantKind};
use itertools::Itertools;

impl<'a> CodecSseTyTrait for EnumRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        let src = self.ir.get(self.context.ir_pack);
        generate_enum_encode_rust_general(lang, src, "self", "Self", |idx, variant| {
            let fields = (variant.kind.fields().iter())
                .map(|field| {
                    format!(
                        "{};\n",
                        lang.call_encode(&field.ty, field.name.rust_style())
                    )
                })
                .join("");

            format!(
                "
            {{
                {};
                {fields}
            }}
            ",
                lang.call_encode(&TAG_TYPE, idx),
            )
        })
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        let src = self.ir.get(self.context.ir_pack);

        let var_decl = lang.var_decl();
        let expr_decode_tag = lang.call_decode(&TAG_TYPE);

        let variants = (src.variants().iter().enumerate())
            .map(|(idx, variant)| {
                (
                    format!("{idx}"),
                    generate_decode_variant(variant, &src.name, lang),
                )
            })
            .collect_vec();

        let body = lang.switch_expr("tag_", variants);

        format!(
            "
            {var_decl} tag_ = {expr_decode_tag};
            {body}
            "
        )
    }
}

fn generate_decode_variant(variant: &IrVariant, enum_name: &NamespacedName, lang: &Lang) -> String {
    let enum_sep = enum_sep(lang);
    match &variant.kind {
        IrVariantKind::Value => format!("{}{enum_sep}{}", enum_name.rust_style(), variant.name),
        IrVariantKind::Struct(st) => {
            GeneralizedStructGenerator::new(st.clone(), StructOrRecord::Struct)
                .generate_decode(lang)
        }
    }
}

pub(crate) fn generate_enum_encode_rust_general(
    lang: &Lang,
    src: &IrEnum,
    self_ref: &str,
    self_path: &str,
    generate_branch: impl Fn(usize, &IrVariant) -> String,
) -> String {
    let enum_sep = enum_sep(lang);
    let variants = (src.variants().iter().enumerate())
        .map(|(idx, variant)| {
            let variant_name = &variant.name;
            let body = generate_branch(idx, variant);
            let pattern = match &variant.kind {
                IrVariantKind::Value => "".to_owned(),
                IrVariantKind::Struct(st) => {
                    let pattern = (st.fields.iter())
                        .map(|field| field.name.rust_style().to_owned())
                        .join(",");
                    let (left, right) = st.brackets_pair();
                    format!("{left}{pattern}{right}")
                }
            };
            (
                format!("{self_path}{enum_sep}{variant_name}{pattern}"),
                body,
            )
        })
        .collect_vec();

    lang.switch_expr(self_ref, variants)
}

fn enum_sep(lang: &Lang) -> &'static str {
    match lang {
        Lang::DartLang(_) => "_",
        Lang::RustLang(_) => "::",
    }
}

const TAG_TYPE: IrType = Primitive(IrTypePrimitive::I32);
