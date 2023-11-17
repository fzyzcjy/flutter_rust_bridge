use crate::codegen::config::internal_config::GeneratorDartInternalConfig;
use crate::codegen::generator::dart_api::internal_config::GeneratorDartApiInternalConfig;
use convert_case::{Case, Casing};
use std::path::Path;

impl From<GeneratorDartInternalConfig> for GeneratorDartApiInternalConfig {
    fn from(config: GeneratorDartInternalConfig) -> Self {
        GeneratorDartApiInternalConfig {
            dart_api_class_name: TODO,
            dart_api_instance_name: compute_dart_api_instance_name(
                config.use_bridge_in_method,
                rust_input_path,
            ),
            dart_enums_style: config.dart_enums_style,
            use_bridge_in_method: config.use_bridge_in_method,
            dart3: config.dart3,
        }
    }
}

fn compute_dart_api_instance_name(use_bridge_in_method: bool, rust_input_path: &Path) -> String {
    if use_bridge_in_method {
        "bridge".to_owned()
    } else {
        rust_input_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_case(Case::Camel)
    }
}
