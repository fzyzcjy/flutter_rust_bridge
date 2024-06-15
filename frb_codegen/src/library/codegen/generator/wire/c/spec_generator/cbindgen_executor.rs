use crate::codegen::generator::misc::path_texts::PathTexts;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::library::commands::cbindgen::{cbindgen, CbindgenArgs};
use crate::utils::file_utils::temp_change_file;

pub(crate) fn execute(
    config: &GeneratorWireCInternalConfig,
    extern_struct_names: Vec<String>,
    rust_output_texts: &PathTexts,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<String> {
    let _pb = progress_bar_pack.generate_cbindgen.start();

    let changed_file_handles = rust_output_texts
        .0
        .iter()
        .map(|rust_output_text| {
            temp_change_file(rust_output_text.path.clone(), |_| {
                rust_output_text.text.all_code()
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let ans = cbindgen(CbindgenArgs {
        rust_crate_dir: &config.rust_crate_dir,
        c_struct_names: extern_struct_names,
        exclude_symbols: vec![],
        after_includes: EXTRA_CODE.to_owned(),
    })?;

    drop(changed_file_handles); // do not drop too early

    Ok(ans)
}

// Please keep in sync with frb_rust and allo-isolate
const EXTRA_CODE: &str = "// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;

typedef int64_t DartPort;
typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);
void store_dart_post_cobject(DartPostCObjectFnType ptr);
// EXTRA END
";
