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

#[cfg(test)]
mod tests {
    use super::*;

    /// Sorts and deduplicates external symbols before emitting dummy references.
    #[test]
    fn emits_sorted_unique_external_symbols_and_required_extra_symbol() {
        let generated = generate(vec![
            "zeta".to_owned(),
            "alpha".to_owned(),
            "zeta".to_owned(),
        ]);

        assert_eq!(
            generated,
            "static int64_t dummy_method_to_enforce_bundling(void) {\n    int64_t dummy_var = 0;\n    dummy_var ^= ((int64_t) (void*) alpha);\n    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);\n    dummy_var ^= ((int64_t) (void*) zeta);\n    return dummy_var;\n}\n"
        );
    }

    /// Emits a valid dummy method when no generated external symbols are present.
    #[test]
    fn emits_required_extra_symbol_for_empty_input() {
        let generated = generate(vec![]);

        assert!(generated.contains("(void*) store_dart_post_cobject"));
        assert_eq!(generated.matches("dummy_var ^=").count(), 1);
    }
}
