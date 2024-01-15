// Please do not move or rename this file
// This file paired with `src/api/deliberate_name_conflict.rs` are used to test module name conflict
// Ensuring that `extract_module` in `frb_codegen\src\library\commands\cargo_expand.rs` can handle duplicated module names(though full path is not the same)

pub struct StructInUpperLevel {
    pub upper: usize,
}
