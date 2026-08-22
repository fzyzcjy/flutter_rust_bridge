use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::internal_config::{
    DartOutputClassNamePack, GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig,
    GeneratorWireDartInternalConfig,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::utils::dart_keywords;
use itertools::Itertools;

pub(super) fn generate(
    config: &GeneratorWireDartInternalConfig,
    rust_extern_funcs: &[ExternFunc],
) -> WireDartOutputCode {
    let methods = rust_extern_funcs
        .iter()
        .filter(|x| x.target == Target::Web)
        .map(generate_method)
        .collect_vec();
    generate_wire_class(config, &methods) + generate_wasm_module_class(config, &methods)
}

fn generate_wire_class(
    config: &GeneratorWireDartInternalConfig,
    methods: &[MethodInfo],
) -> WireDartOutputCode {
    let DartOutputClassNamePack {
        wire_class_name, ..
    } = &config.dart_output_class_name_pack;

    let body = (methods.iter())
        .map(|x| format!("{} => {};", x.declaration, x.implementation))
        .join("\n\n");

    format!(
        "class {wire_class_name} implements BaseWire {{
            {wire_class_name}.fromExternalLibrary(ExternalLibrary lib);

            {body}
        }}
        ",
    )
    .into()
}

fn generate_wasm_module_class(
    config: &GeneratorWireDartInternalConfig,
    methods: &[MethodInfo],
) -> WireDartOutputCode {
    let DartOutputClassNamePack {
        wasm_module_name, ..
    } = &config.dart_output_class_name_pack;

    let GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig {
        wasm_bindgen_name, ..
    } = &config.default_external_library_loader;

    let body = (methods.iter())
        .map(|x| format!("external {};", x.declaration))
        .join("\n\n");

    format!(
        "@JS('{wasm_bindgen_name}') external {wasm_module_name} get wasmModule;

        @JS() @anonymous extension type {wasm_module_name}._(JSObject _) implements JSObject {{
            {body}
        }}
        "
    )
    .into()
}

struct MethodInfo {
    declaration: String,
    implementation: String,
}

fn generate_method(func: &ExternFunc) -> MethodInfo {
    let func_name = &func.partial_func_name;

    let return_type = (func.return_type.as_ref())
        .map(|x| reconstruct_dart_wire_type_from_raw_repr(x))
        .unwrap_or_else(|| "void".to_owned());

    let input_params = (func.params.iter())
        .map(|param| format!("{} {}", param.dart_type, dart_param_name(&param.name)))
        .join(",");

    let forward_args = (func.params.iter())
        .map(|param| dart_param_name(&param.name))
        .join(",");

    MethodInfo {
        declaration: format!("{return_type} {func_name}({input_params})"),
        implementation: format!("wasmModule.{func_name}({forward_args})"),
    }
}

fn dart_param_name(rust_name: &str) -> String {
    dart_keywords::escape(rust_name.strip_prefix("r#").unwrap_or(rust_name).to_owned())
}

/// Since there exists no toolchain that can generate Dart bindings
/// for JS code, we have to supply our own stubs. The external function
/// generator however converts an [MirType] (sometimes a raw string)
/// into its string representation, so these heuristics parse those representations
/// into their appropriate Dart wire types. Note that only a subset of types
/// is supported, outside of which `dynamic` is returned.
///
/// In practice however this is optional as unlike Rust, Dart values are
/// aware of their own types (via the `runtimeType` property) and can
/// safely assume the `dynamic` or `Object` type instead.
fn reconstruct_dart_wire_type_from_raw_repr(ty: &str) -> String {
    let ty = ty.trim();
    let real_ty = if is_rust_pointer(ty) { "int" } else { "JSAny?" };
    format!("{real_ty} /* {ty} */")
}

fn is_rust_pointer(ty: &str) -> bool {
    ty.starts_with("*mut") || ty.starts_with("*const")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that Rust raw identifiers become valid Dart parameter names.
    #[test]
    fn dart_param_name_removes_rust_raw_identifier_prefix() {
        assert_eq!(dart_param_name("r#type"), "type");
        assert_eq!(dart_param_name("r#async"), "async_");
        assert_eq!(dart_param_name("value"), "value");
    }
}
