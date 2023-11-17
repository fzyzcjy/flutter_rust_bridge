use crate::codegen::config::internal_config::GeneratorDartInternalConfig;
use crate::codegen::generator::api_dart::internal_config::GeneratorDartApiInternalConfig;
use convert_case::{Case, Casing};
use std::path::Path;

impl From<GeneratorDartInternalConfig> for GeneratorDartApiInternalConfig {
    fn from(config: GeneratorDartInternalConfig) -> Self {
        // TODO multi file support
        let dart_output_path = config
            .dart_output_path_pack
            .dart_decl_output_path
            .values()
            .next()
            .unwrap();
        let dart_output_stem = get_file_stem(dart_output_path);

        GeneratorDartApiInternalConfig {
            dart_api_class_name: dart_output_stem.to_case(Case::Pascal),
            dart_api_instance_name: compute_dart_api_instance_name(
                config.use_bridge_in_method,
                dart_output_stem,
            ),
            dart_enums_style: config.dart_enums_style,
            use_bridge_in_method: config.use_bridge_in_method,
            dart3: config.dart3,
        }
    }
}

fn compute_dart_api_instance_name(use_bridge_in_method: bool, dart_output_stem: &str) -> String {
    if use_bridge_in_method {
        "bridge".to_owned()
    } else {
        dart_output_stem.to_case(Case::Camel)
    }
}

fn get_file_stem(p: &Path) -> &str {
    p.file_stem().unwrap().to_str().unwrap()
}
