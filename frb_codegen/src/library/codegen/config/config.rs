use crate::codegen::{dumper::internal_config::ConfigDumpContent, RustOpaqueCodecMode};
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
    pub dart_collection_deep_equality: Option<bool>,
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
    pub default_rust_opaque_codec: Option<RustOpaqueCodecMode>,
    pub local: Option<bool>,
    pub default_external_library_loader_web_prefix: Option<String>,
    pub wasm_bindgen_name: Option<String>,
    pub dart_type_rename: Option<HashMap<String, String>>,
    pub enable_lifetime: Option<bool>,
    pub type_64bit_int: Option<bool>,
    pub default_dart_async: Option<bool>,
    pub auto_upgrade_dependency: Option<bool>,
    pub parse_const: Option<bool>,
    pub dart_format: Option<bool>,
    pub dart_fix: Option<bool>,
    pub rust_format: Option<bool>,
    pub stop_on_error: Option<bool>,
    pub dump: Option<Vec<ConfigDumpContent>>,
    pub dump_all: Option<bool>,
    pub rust_features: Option<Vec<String>>,
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
    dart_collection_deep_equality,
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
    default_rust_opaque_codec,
    local,
    default_external_library_loader_web_prefix,
    wasm_bindgen_name,
    dart_type_rename,
    enable_lifetime,
    type_64bit_int,
    default_dart_async,
    auto_upgrade_dependency,
    parse_const,
    dart_format,
    dart_fix,
    rust_format,
    stop_on_error,
    dump,
    dump_all,
    rust_features,
);

#[cfg(test)]
mod tests {
    use super::Config;
    use std::collections::HashMap;

    /// Merges representative field shapes with high-priority values.
    #[test]
    fn merges_representative_field_shapes_with_high_priority_values() {
        let high = Config {
            rust_input: Some("high_input".to_owned()),
            dart3: Some(false),
            dart_type_rename: Some(HashMap::from([("High".to_owned(), "Value".to_owned())])),
            rust_features: Some(vec!["high_feature".to_owned()]),
            ..Default::default()
        };
        let low = Config {
            rust_input: Some("low_input".to_owned()),
            dart_output: Some("low_output.dart".to_owned()),
            dart3: Some(true),
            dart_type_rename: Some(HashMap::from([("Low".to_owned(), "Value".to_owned())])),
            rust_features: Some(vec!["low_feature".to_owned()]),
            ..Default::default()
        };

        let merged = Config::merge(high, low);

        assert_eq!(merged.rust_input.as_deref(), Some("high_input"));
        assert_eq!(merged.dart_output.as_deref(), Some("low_output.dart"));
        assert_eq!(merged.dart3, Some(false));
        assert_eq!(
            merged.dart_type_rename,
            Some(HashMap::from([("High".to_owned(), "Value".to_owned())]))
        );
        assert_eq!(merged.rust_features, Some(vec!["high_feature".to_owned()]));
    }

    /// Preserves absent fields when neither side provides one.
    #[test]
    fn leaves_fields_none_when_both_configs_omit_them() {
        let merged = Config::merge(Config::default(), Config::default());

        assert_eq!(merged.base_dir, None);
        assert_eq!(merged.rust_input, None);
        assert_eq!(merged.dart3, None);
        assert_eq!(merged.rust_features, None);
    }

    /// Rejects misspelled configuration fields during deserialization.
    #[test]
    fn rejects_unknown_configuration_fields() {
        let error = serde_yaml::from_str::<Config>("rust_input: crate::api\nmisspelled: true\n")
            .unwrap_err();

        assert!(error.to_string().contains("misspelled"));
    }
}
