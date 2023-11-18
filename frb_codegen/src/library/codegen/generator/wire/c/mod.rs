use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

pub(crate) fn generate(ir_pack: &IrPack) {
    todo!()
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
        .map(|func_name| format!("    dummy_var ^= ((int64_t) (void*) {func_name});"))
        .join("\n")
}
