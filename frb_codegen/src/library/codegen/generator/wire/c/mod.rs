use std::path::Path;

fn generate_dummy_function(func_names: &[String]) -> String {
    get_dummy_func("", func_names)
}

fn get_dummy_func(api_block_name: &str, func_names: &[String]) -> String {
    format!(
        r#"static int64_t {signature}(void) {{
    int64_t dummy_var = 0;
{content}
    return dummy_var;
}}
"#,
        signature = get_dummy_signature(api_block_name),
        content = get_dummy_var(func_names),
    )
}

fn get_dummy_signature(api_block_name: &str) -> String {
    match api_block_name.is_empty() {
        true => "dummy_method_to_enforce_bundling".into(),
        false => format!(r#"dummy_method_to_enforce_bundling_{api_block_name}"#),
    }
}

fn get_dummy_var(func_names: &[String]) -> String {
    func_names
        .iter()
        .map(|func_name| format!("    dummy_var ^= ((int64_t) (void*) {func_name});"))
        .collect_vec()
        .join("\n")
}
