use crate::codegen::generator::api_dart::spec_generator::class::proxy_variant;

pub(super) fn generate_lockable_encode_to_enum(enum_name: &str) -> String {
    let variants = (mir.variants.iter().enumerate())
        .map(|(index, variant)| {
            let variant_dart_extra_type = proxy_variant::compute_dart_extra_type(variant, context);
            format!(
                "if (self is {variant_dart_extra_type}) {{
                    return {enum_name}.variant{index}(self._upstream);
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
