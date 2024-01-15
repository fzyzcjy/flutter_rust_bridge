// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

// Please do not move or rename this file
// This file paired with `src/deliberate_name_conflict.rs` are used to test module name conflict
// Ensuring that `extract_module` in `frb_codegen\src\library\commands\cargo_expand.rs` can handle duplicated module names(though full path is not the same)

use crate::deliberate_name_conflict::StructInUpperLevel;

pub struct StructInLowerLevel {
    pub inner: StructInUpperLevel,
}

pub fn test_duplicated_module_names(s: StructInLowerLevel) -> StructInUpperLevel {
    s.inner
}
