use itertools::Itertools;

pub(super) fn generate(extern_func_names: Vec<String>) -> String {
    let func_names = [
        extern_func_names,
        (EXTRA_EXTERN_FUNC_NAMES.iter().map(|&x| x.to_owned())).collect_vec(),
    ]
    .concat();
    generate_dummy_function(&func_names)
}

fn generate_dummy_function(func_names: &[String]) -> String {
    let dummy_var_operations = compute_dummy_var_operations(func_names);
    format!(
        r#"static int64_t dummy_method_to_enforce_bundling(void) {{
    int64_t dummy_var = 0;
{dummy_var_operations}
    return dummy_var;
}}
"#
    )
}

fn compute_dummy_var_operations(func_names: &[String]) -> String {
    func_names
        .iter()
        .sorted()
        .dedup()
        .map(|func_name| format!("    dummy_var ^= ((int64_t) (void*) {func_name});"))
        .join("\n")
}

const EXTRA_EXTERN_FUNC_NAMES: &[&str] = &["store_dart_post_cobject"];
