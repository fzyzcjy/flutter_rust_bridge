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

#[cfg(test)]
mod tests {
    use super::*;

    /// Emits each runtime type check with its matching enum constructor.
    #[test]
    fn encode_to_enum_preserves_variant_names_and_extra_code() {
        let output = generate_encode_to_enum(
            "Message",
            &[
                VariantInfo {
                    enum_variant_name: "text".to_owned(),
                    ty_name: "TextMessage".to_owned(),
                    extra_code: ".value".to_owned(),
                },
                VariantInfo {
                    enum_variant_name: "image".to_owned(),
                    ty_name: "ImageMessage".to_owned(),
                    extra_code: "".to_owned(),
                },
            ],
        );

        assert!(output.contains("if (self is TextMessage)"));
        assert!(output.contains("return Message.text(self.value);"));
        assert!(output.contains("if (self is ImageMessage)"));
        assert!(output.contains("return Message.image(self);"));
        assert!(output.contains("throw Exception('not reachable');"));
    }
}
