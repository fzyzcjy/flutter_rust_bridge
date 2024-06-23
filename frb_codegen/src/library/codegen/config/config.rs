use crate::codegen::dumper::internal_config::ConfigDumpContent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for code generation
/// Refer to `GenerateCommandArgs` for documentations
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub base_dir: Option<String>,
    pub rust_input: Option<String>,
    pub dart_output: Option<String>,
    pub c_output: Option<String>,
    pub duplicated_c_output: Option<Vec<String>>,
    pub rust_root: Option<String>,
    pub rust_output: Option<String>,
    pub dart_entrypoint_class_name: Option<String>,
    pub dart_format_line_length: Option<u32>,
    pub dart_preamble: Option<String>,
    pub rust_preamble: Option<String>,
    pub dart_enums_style: Option<bool>,
    pub add_mod_to_lib: Option<bool>,
    pub llvm_path: Option<Vec<String>>,
    pub llvm_compiler_opts: Option<String>,
    pub dart_root: Option<String>,
    pub build_runner: Option<bool>,
    pub extra_headers: Option<String>,
    pub web: Option<bool>,
    pub deps_check: Option<bool>,
    pub dart3: Option<bool>,
    pub full_dep: Option<bool>,
    pub local: Option<bool>,
    pub default_external_library_loader_web_prefix: Option<String>,
    pub dart_type_rename: Option<HashMap<String, String>>,
    pub enable_lifetime: Option<bool>,
    pub type_64bit_int: Option<bool>,
    pub default_dart_async: Option<bool>,
    pub stop_on_error: Option<bool>,
    pub dump: Option<Vec<ConfigDumpContent>>,
    pub dump_all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetaConfig {
    pub watch: bool,
}

macro_rules! generate_merge {
    ($($field:ident,)*) => (
        impl Config {
            // Only used internally
            #[doc(hidden)]
            pub fn merge(priority_high: Self, priority_low: Self) -> Self {
                Self {
                    $(
                        $field: priority_high.$field.or(priority_low.$field),
                    )*
                }
            }
        }
    );
}

generate_merge!(
    base_dir,
    rust_input,
    dart_output,
    c_output,
    duplicated_c_output,
    rust_root,
    rust_output,
    dart_entrypoint_class_name,
    dart_format_line_length,
    dart_preamble,
    rust_preamble,
    dart_enums_style,
    add_mod_to_lib,
    llvm_path,
    llvm_compiler_opts,
    dart_root,
    build_runner,
    extra_headers,
    web,
    deps_check,
    dart3,
    full_dep,
    local,
    default_external_library_loader_web_prefix,
    dart_type_rename,
    enable_lifetime,
    type_64bit_int,
    default_dart_async,
    stop_on_error,
    dump,
    dump_all,
);
