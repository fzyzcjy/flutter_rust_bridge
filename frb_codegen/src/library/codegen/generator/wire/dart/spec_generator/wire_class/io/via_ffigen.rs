use crate::codegen::generator::wire::dart::internal_config::{
    DartOutputClassNamePack, GeneratorWireDartInternalConfig,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::wire_class::io::common::generate_wire_class_header;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};
use anyhow::ensure;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<WireDartOutputCode> {
    let content = execute_ffigen(config, c_file_content, progress_bar_pack)?;
    let content = postpare_modify(&content, &config.dart_output_class_name_pack);
    sanity_check(&content, &config.dart_output_class_name_pack)?;
    Ok(WireDartOutputCode::parse(&content))
}

fn execute_ffigen(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<String> {
    let _pb = progress_bar_pack.generate_ffigen.start();
    ffigen(FfigenArgs {
        c_file_content,
        dart_class_name: &config.dart_output_class_name_pack.wire_class_name,
        llvm_path: &config.llvm_path,
        llvm_compiler_opts: &config.llvm_compiler_opts,
        dart_root: &config.dart_root,
        function_rename: Some(
            &[(format!("{}(.*)", config.c_symbol_prefix), "$1".to_owned())].into(),
        ),
    })
}

fn postpare_modify(
    content_raw: &str,
    dart_output_class_name_pack: &DartOutputClassNamePack,
) -> String {
    lazy_static! {
        static ref FILTER: Regex =
            Regex::new(r#"(?s)final class WireSyncRust2DartSse extends ffi.Struct \{.*?\}"#)
                .unwrap();
    }

    let DartOutputClassNamePack {
        wire_class_name, ..
    } = &dart_output_class_name_pack;

    let ans = content_raw
        .replace(
            &format!("class {wire_class_name} {{"),
            &generate_wire_class_header(wire_class_name),
        )
        .replace("final class DartCObject extends ffi.Opaque {}", "")
        .replace("final class _Dart_Handle extends ffi.Opaque {}", "")
        .replace(
            "typedef WireSyncRust2DartDco = ffi.Pointer<DartCObject>;",
            "",
        );
    let ans = FILTER.replace_all(&ans, "").to_string();
    ans
}

fn sanity_check(
    generated_dart_wire_code: &str,
    dart_output_class_name_pack: &DartOutputClassNamePack,
) -> anyhow::Result<()> {
    ensure!(
        generated_dart_wire_code.contains(&dart_output_class_name_pack.wire_class_name),
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        "Nothing is generated for dart wire class. \
            Maybe you forget to put code like `mod the_generated_bridge_code;` to your `lib.rs`?",
        // frb-coverage:ignore-end
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postpare_modify_removes_ffigen_21_wire_sync_struct() {
        let output = postpare_modify(
            r#"
class RustLibWire {
}

final class WireSyncRust2DartSse extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;

  static ffi.Pointer<WireSyncRust2DartSse> $allocate(
    ffi.Allocator $allocator, {
    required ffi.Pointer<ffi.Uint8> ptr,
    required int len,
  }) => $allocator<WireSyncRust2DartSse>()
    ..ref.ptr = ptr
    ..ref.len = len;
}
"#,
            &DartOutputClassNamePack {
                entrypoint_class_name: "RustLib".to_owned(),
                api_class_name: "RustLibApi".to_owned(),
                api_impl_class_name: "RustLibApiImpl".to_owned(),
                api_impl_platform_class_name: "RustLibApiImplPlatform".to_owned(),
                wire_class_name: "RustLibWire".to_owned(),
                wasm_module_name: "RustLibWasmModule".to_owned(),
            },
        );

        assert!(
            !output.contains("..ref.ptr = ptr"),
            "ffigen 21 class fragment remains and causes expected_executable parser errors:\n{output}"
        );
    }
}
