use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::misc::has_port_argument;
use std::borrow::Cow;

pub(super) fn generate(config: &GeneratorWireDartInternalConfig) -> WireDartOutputCode {
    generate_wire_class(config) + generate_wasm_module_class(config)
}

fn generate_wire_class(config: &GeneratorWireDartInternalConfig) -> WireDartOutputCode {
    let wire_class_name = &config.dart_output_class_name_pack.wire_class_name;

    let body = (funcs.into_iter().map(generate_wire_class_method)).join("\n\n");

    format!(
        "class {wire_class_name} extends BaseWire {{
            {wire_class_name}(FutureOr<WasmModule> module) : super(WasmModule.cast<{dart_wasm_module_name}>(module));
            
            {body}
        }}
        ",
    )
    .into()
}

fn generate_wire_class_method(func: &IrFuncDisplay) -> String {
    format!(
        "{out} {name}({}) => wasmModule.{name}({});",
        func.inputs
            .iter()
            .map(|param| format!("{} {}", param.ty, param.name))
            .join(","),
        func.inputs
            .iter()
            .map(|param| param.name.to_string())
            .join(","),
        name = func.name,
        out = if has_port_argument(func) {
            "void".into()
        } else {
            reconstruct_dart_wire_type_from_raw_repr(&func.output)
        },
    )
}

fn generate_wasm_module_class(config: &GeneratorWireDartInternalConfig) -> WireDartOutputCode {
    let body = (funcs.into_iter().map(generate_wasm_module_class_method)).join("\n\n");

    format!(
        "@JS('wasm_bindgen') external {dart_wasm_module_name} get wasmModule;

        @JS() @anonymous class {dart_wasm_module_name} implements WasmModule {{
            external Object /* Promise */ call([String? moduleName]);

            external {dart_wasm_module_name} bind(dynamic thisArg, String moduleName);

            {body}
        }}
        "
    )
    .into()
}

fn generate_wasm_module_class_method(func: &IrFuncDisplay) -> String {
    format!(
        "external {} {name}({});",
        reconstruct_dart_wire_type_from_raw_repr(&func.output),
        func.inputs
            .iter()
            .map(|param| format!("{} {}", param.ty, param.name))
            .join(","),
        name = func.name,
    )
}

/// Since there exists no toolchain that can generate Dart bindings
/// for JS code, we have to supply our own stubs. The external function
/// generator however converts an [IrType] (sometimes a raw string)
/// into its string representation, so these heuristics parse those representations
/// into their appropriate Dart wire types. Note that only a subset of types
/// is supported, outside of which `dynamic` is returned.
///
/// In practice however this is optional as unlike Rust, Dart values are
/// aware of their own types (via the `runtimeType` property) and can
/// safely assume the `dynamic` or `Object` type instead.
fn reconstruct_dart_wire_type_from_raw_repr(ty: &str) -> String {
    let ty = ty.trim();
    let real_ty = if is_rust_pointer(ty) {
        "int"
    } else {
        "dynamic"
    };
    format!("{real_ty} /* {ty} */")
}

fn is_rust_pointer(ty: &str) -> bool {
    ty.starts_with("*mut") || ty.starts_with("*const")
}
