use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::misc::OutputTexts;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::library::commands::cbindgen::{cbindgen, CbindgenArgs};
use crate::utils::file_utils::temp_change_file;
use anyhow::ensure;
use itertools::Itertools;

pub(crate) fn execute(
    config: &GeneratorWireCInternalConfig,
    extern_struct_names: Vec<String>,
    rust_output_texts: &OutputTexts,
) -> anyhow::Result<String> {
    let rust_output_path_common = &config.rust_output_path[TargetOrCommon::Common];
    ensure!((rust_output_texts.paths().iter()).any(|path| path == rust_output_path_common));
    let changed_file_handles = rust_output_texts
        .0
        .iter()
        .map(|rust_output_text| {
            temp_change_file(rust_output_text.path.clone(), |_| {
                rust_output_text.text.clone()
                    + (if rust_output_text.path == rust_output_path_common {
                        DUMMY_WIRE_CODE_FOR_BINDGEN
                    } else {
                        ""
                    })
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let ans = cbindgen(CbindgenArgs {
        rust_crate_dir: &config.rust_crate_dir,
        c_struct_names: extern_struct_names,
        // TODO will try to avoid manually specify exclude_symbols by using `pub(crate)` instead of `pub`
        // exclude_symbols: generated_rust.get_exclude_symbols(all_symbols),
        exclude_symbols: vec![],
    })?;

    drop(changed_file_handles); // do not drop too early

    Ok(ans)
}

// TODO
// fn compute_input_rust_code() {}

// NOTE [DartPostCObjectFnType] was originally [*mut DartCObject] but I changed it to [*mut c_void]
// because cannot automatically generate things related to [DartCObject]. Anyway this works fine.
// NOTE please sync [DUMMY_WIRE_CODE_FOR_BINDGEN] and [EXTRA_EXTERN_FUNC_NAMES]
const DUMMY_WIRE_CODE_FOR_BINDGEN: &str = r#"
    // ----------- DUMMY CODE FOR BINDGEN ----------

    // copied from: allo-isolate
    pub type DartPort = i64;
    pub type DartPostCObjectFnType = unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool;
    #[no_mangle] pub unsafe extern "C" fn store_dart_post_cobject(ptr: DartPostCObjectFnType) { panic!("dummy code") }
    #[no_mangle] pub unsafe extern "C" fn get_dart_object(ptr: usize) -> Dart_Handle { panic!("dummy code") }
    #[no_mangle] pub unsafe extern "C" fn drop_dart_object(ptr: usize) { panic!("dummy code") }
    #[no_mangle] pub unsafe extern "C" fn new_dart_opaque(handle: Dart_Handle) -> usize { panic!("dummy code") }
    #[no_mangle] pub unsafe extern "C" fn init_frb_dart_api_dl(obj: *mut c_void) -> isize { panic!("dummy code") }

    pub struct DartCObject;
    pub type WireSyncReturn = *mut DartCObject;

    // ---------------------------------------------
"#;
