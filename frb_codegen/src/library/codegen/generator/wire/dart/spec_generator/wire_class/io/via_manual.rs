use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::wire_class::io::common::generate_wire_class_header;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    rust_extern_funcs: &[ExternFunc],
) -> anyhow::Result<WireDartOutputCode> {
    let wire_class_header =
        generate_wire_class_header(&config.dart_output_class_name_pack.wire_class_name);

    let code = format!(
        "
        {wire_class_header}
        }}
        "
    );
    Ok(code.into())
}
