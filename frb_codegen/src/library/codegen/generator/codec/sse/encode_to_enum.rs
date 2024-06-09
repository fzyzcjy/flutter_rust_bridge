use itertools::Itertools;

pub(super) fn generate_encode_to_enum(enum_name: &str, variants: &[VariantInfo]) -> String {
    let variants = (variants.iter())
        .map(|variant| {
            format!(
                "if (self is {ty_name}) {{
                    return {enum_name}.{enum_variant_name}(self{extra_code});
                }}
                ",
                ty_name = variant.ty_name,
                enum_variant_name = variant.enum_variant_name,
                extra_code = variant.extra_code,
            )
        })
        .join("");

    format!(
        "
        (() {{
            {variants}
            throw Exception('not reachable');
        }})()
        "
    )
}

pub(crate) struct VariantInfo {
    pub enum_variant_name: String,
    pub ty_name: String,
    pub extra_code: String,
}
