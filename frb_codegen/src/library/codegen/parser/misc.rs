use crate::library::misc::consts::HANDLER_NAME;

pub(crate) fn parse_has_executor(source_rust_content: &str) -> bool {
    source_rust_content.contains(&format!("static ref {HANDLER_NAME}"))
}
