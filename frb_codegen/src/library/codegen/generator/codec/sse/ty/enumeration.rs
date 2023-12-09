use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrVariantKind};
use itertools::Itertools;

impl<'a> CodecSseTyTrait for EnumRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        format!("return TODO;")
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("return TODO;")
    }
}

pub(crate) fn generate_enum_encode_rust(src: &IrEnum) -> String {
    let self_ref = self.generate_access_object_core("self".to_owned());

    let variants = src
        .variants()
        .iter()
        .enumerate()
        .map(|(idx, variant)| {
            let tag = format!("{idx}.into_dart()");
            match &variant.kind {
                IrVariantKind::Value => {
                    format!("{self_path}::{} => vec![{tag}],", variant.name)
                }
                IrVariantKind::Struct(st) => {
                    let fields = Some(tag)
                        .into_iter()
                        .chain(st.fields.iter().map(|field| {
                            format!("{}.into_into_dart().into_dart()", field.name.rust_style())
                        }))
                        .join(",");
                    let pattern = st
                        .fields
                        .iter()
                        .map(|field| field.name.rust_style().to_owned())
                        .join(",");
                    let (left, right) = st.brackets_pair();
                    let variant_name = &variant.name;
                    format!("{self_path}::{variant_name}{left}{pattern}{right} => vec![{fields}],",)
                }
            }
        })
        .join("\n");

    format!(
        "
        match {self_ref} {{
            {variants}
        }}.into_dart()
        "
    )
}
