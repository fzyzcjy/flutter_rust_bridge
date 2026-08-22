use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::utils::basic_code::simple_code_trait::SimpleCodeTrait;

pub(crate) fn section_header_comment<T: SimpleCodeTrait>(
    section_name: &str,
    item: &Acc<Vec<T>>,
) -> Acc<Vec<T>> {
    item.map_ref(|x, _target| {
        vec![T::new_body(
            if x.iter().all(|x| x.body().trim().is_empty()) {
                "".to_owned()
            } else {
                section_header_comment_raw(section_name)
            },
        )]
    })
}

pub(crate) fn section_header_comment_raw(section_name: &str) -> String {
    format!("\n\n// Section: {section_name}\n\n")
}

pub(crate) fn generate_text_respecting_web_flag<T>(
    raw: Acc<T>,
    web_enabled: bool,
) -> Acc<Option<T>> {
    raw.map(|value, target| (target != TargetOrCommon::Web || web_enabled).then_some(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::acc::Acc;
    use crate::utils::basic_code::general_code::GeneralRustCode;

    /// Adds a section header only when at least one target contains generated text.
    #[test]
    fn adds_section_headers_only_for_nonempty_content() {
        let empty = Acc::new(|_| Vec::<GeneralRustCode>::new());
        assert_eq!(section_header_comment("empty", &empty).common[0].body, "");

        let content = Acc::new_common(vec![GeneralRustCode {
            body: "body".to_owned(),
        }]);
        assert_eq!(
            section_header_comment("types", &content).common[0].body,
            "\n\n// Section: types\n\n"
        );
        assert_eq!(
            section_header_comment_raw("types"),
            "\n\n// Section: types\n\n"
        );
    }

    /// Omits only web output when web support is disabled.
    #[test]
    fn respects_the_web_generation_flag() {
        let input = Acc::new(|target| target.to_string());
        let disabled = generate_text_respecting_web_flag(input.clone(), false);
        let enabled = generate_text_respecting_web_flag(input, true);

        assert_eq!(disabled.common.as_deref(), Some("Common"));
        assert_eq!(disabled.io.as_deref(), Some("Io"));
        assert_eq!(disabled.web, None);
        assert_eq!(enabled.web.as_deref(), Some("Web"));
    }
}
