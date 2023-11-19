use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};
use crate::utils::dart_basic_code::DartBasicCode;

pub(super) fn generate(config: &GeneratorWireDartInternalConfig) -> anyhow::Result<DartBasicCode> {
    let content = execute_ffigen(config)?;
    let content = postpare_modify(&content, &config.dart_wire_class_name);
    Ok(DartBasicCode::parse(&content))
}

fn execute_ffigen(config: &GeneratorWireDartInternalConfig) -> anyhow::Result<String> {
    ffigen(FfigenArgs {
        c_file_content: &config.c_file_content,
        dart_class_name: &config.dart_wire_class_name,
        llvm_path: &config.llvm_path,
        llvm_compiler_opts: &config.llvm_compiler_opts,
        dart_root: &config.dart_root,
    })
}

fn postpare_modify(content_raw: &str, dart_wire_class_name: &str) -> String {
    content_raw
        .replace(
            &format!("class {dart_wire_class_name} {{",),
            &format!(
                "class {dart_wire_class_name} implements FlutterRustBridgeWireBase {{
            @internal
            late final dartApi = DartApiDl(init_frb_dart_api_dl);",
            ),
        )
        .replace("final class DartCObject extends ffi.Opaque {}", "")
        .replace("typedef WireSyncReturn = ffi.Pointer<DartCObject>;", "")
    // .replace( "class _Dart_Handle extends ffi.Opaque {}", "base class _Dart_Handle extends ffi.Opaque {}")
}
