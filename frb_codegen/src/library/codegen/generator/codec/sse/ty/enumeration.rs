use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrVariant, IrVariantKind};
use itertools::Itertools;

impl<'a> CodecSseTyTrait for EnumRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        format!("return TODO;")
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("return TODO;")
    }
}

pub(crate) fn generate_enum_encode_rust(
    src: &IrEnum,
    self_ref: &str,
    self_path: &str,
    generate_branch: impl Fn(usize, &IrVariant) -> String,
) -> String {
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
            format!("{self_path}::{variant_name}{pattern} => {body},")
        })
        .join("\n");

    format!(
        "
        match {self_ref} {{
            {variants}
        }}
        "
    )
}
