use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::utils::basic_code::basic::SimpleCodeTrait;

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

pub(crate) fn generate_text_respecting_web_flag(
    raw: Acc<String>,
    web_enabled: bool,
) -> Acc<Option<String>> {
    raw.map(|value, target| (target != TargetOrCommon::Web || web_enabled).then_some(value))
}
