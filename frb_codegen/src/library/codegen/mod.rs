//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;
mod generator;
pub(crate) mod ir;
pub(crate) mod parser;

use crate::codegen::config::internal_config::{GeneratorDartInternalConfig, InternalConfig};
use crate::codegen::generator::dart_api::internal_config::GeneratorDartApiInternalConfig;
use crate::codegen::generator::output::OutputCode;
pub use config::config::{Config, ConfigDump};
pub use config::config_parser::*;
use convert_case::Case;
use log::debug;
use std::path::Path;

/// Execute the main code generator
pub fn generate(config: Config) -> anyhow::Result<()> {
    debug!("config={config:?}");
    let internal_config = InternalConfig::parse(config)?;
    debug!("internal_config={internal_config:?}");

    let ir_pack = parser::parse(&internal_config.parser)?;

    // TODO seems not this ideal. we need to call various external tools that directly write to fs
    let output_code_dart_api: OutputCode =
        generator::dart_api::generate(&ir_pack, &internal_config.generator.dart.into())?.into();
    let output_code_wire = generator::wire::generate(&ir_pack)?;
    let output_code = output_code_dart_api.merge(output_code_wire);
    output_code.write()?;

    Ok(())
}

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
