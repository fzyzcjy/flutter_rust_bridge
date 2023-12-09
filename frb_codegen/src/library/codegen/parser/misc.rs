pub(crate) fn parse_has_executor(source_rust_content: &str) -> bool {
    source_rust_content.contains(&"static ref FLUTTER_RUST_BRIDGE_HANDLER".to_string())
}
