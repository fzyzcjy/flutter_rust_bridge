use itertools::Itertools;
use crate::codegen::generator::api_dart::spec_generator::class::proxy_variant;

pub(super) fn generate_encode_to_enum(enum_name: &str, variants: &[VariantInfo]) -> String {
    let variants = (variants.iter())
        .map(|variant| {
            let ty_name = &variant.ty_name;
            let enum_variant_name = &variant.enum_variant_name;
            format!(
                "if (self is {ty_name}) {{
                    return {enum_name}.{enum_variant_name}(self._upstream);
                }}
                "
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
}
