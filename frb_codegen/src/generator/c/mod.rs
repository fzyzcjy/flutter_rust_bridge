pub fn generate_dummy(func_names: &[String], api_block_name: &str) -> String {
    format!(
        r#"static int64_t dummy_method_to_enforce_bundling{suffix}(void) {{
    int64_t dummy_var = 0;
{content}
    return dummy_var;
}}"#,
        suffix = if api_block_name.is_empty() {
            "".into()
        } else {
            format!("_{api_block_name}")
        },
        content = func_names
            .iter()
            .map(|func_name| { format!("    dummy_var ^= ((int64_t) (void*) {});", func_name) })
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
