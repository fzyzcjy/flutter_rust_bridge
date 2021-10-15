pub fn generate_dummy(func_names: &[String]) -> String {
    format!(
        r#"
        static void dummy_method_to_enforce_bundling(void) {{
            int dummy_var = 0;
            {}
            NSLog(@"dummy_var=%d", dummy_var");
        }}
        "#,
        func_names
            .iter()
            .map(|func_name| { format!("dummy_var += ((int) (void*) {}) % 2;", func_name) })
            .join("\n"),
    )
}
