use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;

pub(super) fn generate() -> WireDartOutputCode {
    let body = (funcs.into_iter().map(generate_wasm_wire_func_method)).join("\n\n");
    format!(
        "class {dart_wire_class_name} extends FlutterRustBridgeWasmWireBase<{dart_wasm_module_name}> {{
            {dart_wire_class_name}(FutureOr<WasmModule> module) : super(WasmModule.cast<{dart_wasm_module_name}>(module));
            
            {body}
        }}
        ",
    )
    .into()
}
