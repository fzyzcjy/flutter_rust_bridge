use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;

pub(super) fn generate(config: &GeneratorWireDartInternalConfig) -> WireDartOutputCode {
    generate_wire_class(config) + generate_wasm_module(config)
}

fn generate_wire_class(config: &GeneratorWireDartInternalConfig) -> WireDartOutputCode {
    let wire_class_name = &config.dart_output_class_name_pack.wire_class_name;

    let body = (funcs.into_iter().map(generate_wasm_wire_func_method)).join("\n\n");
    format!(
        "class {wire_class_name} extends BaseWire {{
            {wire_class_name}(FutureOr<WasmModule> module) : super(WasmModule.cast<{dart_wasm_module_name}>(module));
            
            {body}
        }}
        ",
    )
    .into()
}

fn generate_wasm_module(config: &GeneratorWireDartInternalConfig) -> WireDartOutputCode {
    todo!()
}
