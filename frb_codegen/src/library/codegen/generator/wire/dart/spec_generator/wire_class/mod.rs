use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::dart::internal_config::{
    DartOutputClassNamePack, GeneratorWireDartInternalConfig,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::misc::GeneratorProgressBarPack;

mod io;
mod web;

pub(super) fn generate(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
    rust_extern_funcs: &[ExternFunc],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<Acc<Vec<WireDartOutputCode>>> {
    if !config.enable {
        return Ok(Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Web => {
                generate_disabled_text(&config.dart_output_class_name_pack).into()
            }
            TargetOrCommon::Common => "".into(),
        }));
    }

    Ok(Acc {
        io: vec![io::generate(config, c_file_content, progress_bar_pack)?],
        web: vec![web::generate(config, rust_extern_funcs)],
        ..Default::default()
    })
}

fn generate_disabled_text(dart_output_class_name_pack: &DartOutputClassNamePack) -> String {
    format!(
        "class {wire_class_name} implements BaseWire {{
          {wire_class_name}.fromExternalLibrary(ExternalLibrary lib);
        }}",
        wire_class_name = dart_output_class_name_pack.wire_class_name,
    )
}
