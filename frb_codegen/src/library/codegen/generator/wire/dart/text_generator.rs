use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::text_generator_utils::{
    generate_text_respecting_wasm_flag, section_header_comment,
};
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::WireDartOutputSpec;

pub(super) struct WireDartOutputText {
    pub(super) text: Acc<Option<String>>,
}

pub(super) fn generate(
    spec: &WireDartOutputSpec,
    config: &GeneratorWireDartInternalConfig,
) -> anyhow::Result<WireDartOutputText> {
    let merged_code = generate_merged_code(spec);
    let text = generate_text_from_merged_code(
        config,
        (merged_code.clone())
            .map(|code, target| code.all_code(target, &config.dart_output_class_name_pack)),
    )?;
    Ok(WireDartOutputText { text })
}

fn generate_merged_code(spec: &WireDartOutputSpec) -> Acc<WireDartOutputCode> {
    let mut merged_code = Acc::<Vec<WireDartOutputCode>>::default();
    let mut add = |section_name: &str, item: &Acc<Vec<WireDartOutputCode>>| {
        merged_code.push(section_header_comment(section_name).into());
        merged_code += item.clone();
    };

    add(
        "boilerplate",
        &Acc::new_common(vec![spec.misc.boilerplate.clone()]),
    );
    add(
        "dispatcher_api_functions", // TODO improve name (it does not show there)
        &Acc::new_common(spec.misc.dispatcher_api_functions.clone()),
    );
    add(
        "dispatcher_opaque_getters", // TODO improve name (it does not show there)
        &Acc::new_common(spec.misc.dispatcher_opaque_getters.clone()),
    );
    add("c_binding", &Acc::new_io(vec![spec.misc.c_binding.clone()]));
    add("impl_wire2api", &spec.wire2api.impl_wire2api);
    add("api2wire_funcs", &spec.api2wire.api2wire_funcs);
    add(
        "api_fill_to_wire_funcs",
        &spec.api2wire.api_fill_to_wire_funcs,
    );

    merged_code.map(|code, _| code.into_iter().fold(Default::default(), |a, b| a + b))
}

fn generate_text_from_merged_code(
    config: &GeneratorWireDartInternalConfig,
    core_code: Acc<String>,
) -> anyhow::Result<Acc<Option<String>>> {
    Ok(generate_text_respecting_wasm_flag(
        core_code,
        config.wasm_enabled,
    ))
}
