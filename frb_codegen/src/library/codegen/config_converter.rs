use crate::codegen::config::internal_config::GeneratorDartInternalConfig;
use crate::codegen::generator::dart_api::internal_config::GeneratorDartApiInternalConfig;
use convert_case::{Case, Casing};
use std::path::Path;

impl From<GeneratorDartInternalConfig> for GeneratorDartApiInternalConfig {
    fn from(config: GeneratorDartInternalConfig) -> Self {
        let dart_api_instance_name = if config.use_bridge_in_method {
            "bridge".to_owned()
        } else {
            Path::new(&config.rust_input_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
                .to_case(Case::Camel)
        };

        GeneratorDartApiInternalConfig {
            dart_api_class_name: TODO,
            dart_api_instance_name,
            dart_enums_style: config.dart_enums_style,
            use_bridge_in_method: config.use_bridge_in_method,
            dart3: config.dart3,
        }
    }
}
