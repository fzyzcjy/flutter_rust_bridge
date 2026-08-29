mod boxed;
mod dart_fn;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod primitive;
mod primitive_list;
mod record;
mod rust_auto_opaque_implicit;
mod rust_opaque;
mod structure;
mod trait_def;

use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartCodecCstGeneratorEncoderTrait {
    fn generate_encode_func_body(&self) -> Acc<Option<String>>;

    fn generate_encode_api_fill_to_wire_body(&self) -> Option<String> {
        None
    }

    fn dart_wire_type(&self, target: Target) -> String;
}

#[cfg(test)]
pub(crate) mod test_utils {
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::generator::wire::dart::internal_config::{
        DartOutputClassNamePack, GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig,
        GeneratorWireDartInternalConfig,
    };
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::WireDartCodecCstGeneratorContext;
    use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
    use crate::codegen::ir::mir::pack::MirPack;
    use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
    use std::collections::HashMap;
    use std::path::PathBuf;

    pub(crate) fn pack() -> MirPack {
        MirPack {
            funcs_all: vec![],
            extra_types_all: vec![],
            struct_pool: Default::default(),
            enum_pool: Default::default(),
            dart_code_of_type: Default::default(),
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: Default::default(),
        }
    }

    pub(crate) fn api_dart_config() -> GeneratorApiDartInternalConfig {
        GeneratorApiDartInternalConfig {
            dart_collection_deep_equality: false,
            dart_enums_style: true,
            dart3: true,
            dart_decl_base_output_path: PathBuf::new(),
            dart_impl_output_path: Default::default(),
            dart_entrypoint_class_name: "Entrypoint".into(),
            dart_preamble: String::new(),
            dart_type_rename: HashMap::new(),
        }
    }

    pub(crate) fn wire_dart_config(web_enabled: bool) -> GeneratorWireDartInternalConfig {
        GeneratorWireDartInternalConfig {
            has_ffigen: false,
            web_enabled,
            llvm_path: vec![],
            llvm_compiler_opts: String::new(),
            dart_root: PathBuf::new(),
            extra_headers: String::new(),
            dart_impl_output_path: Default::default(),
            dart_output_class_name_pack: DartOutputClassNamePack {
                entrypoint_class_name: "Entrypoint".into(),
                api_class_name: "Api".into(),
                api_impl_class_name: "ApiImpl".into(),
                api_impl_platform_class_name: "ApiImplPlatform".into(),
                wire_class_name: "Wire".into(),
                wasm_module_name: "Wasm".into(),
            },
            default_external_library_loader:
                GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig {
                    stem: String::new(),
                    io_directory: String::new(),
                    web_prefix: String::new(),
                    wasm_bindgen_name: String::new(),
                },
            c_symbol_prefix: String::new(),
        }
    }

    pub(crate) fn wire_rust_config(web_enabled: bool) -> GeneratorWireRustInternalConfig {
        GeneratorWireRustInternalConfig {
            rust_crate_dir: PathBuf::new(),
            web_enabled,
            rust_output_path: PathBuf::new(),
            c_symbol_prefix: String::new(),
            has_ffigen: false,
            default_stream_sink_codec: CodecMode::Cst,
            default_rust_opaque_codec: RustOpaqueCodecMode::Nom,
            rust_preamble: String::new(),
        }
    }

    pub(crate) fn context<'a>(
        mir_pack: &'a MirPack,
        wire_dart_config: &'a GeneratorWireDartInternalConfig,
        wire_rust_config: &'a GeneratorWireRustInternalConfig,
        api_dart_config: &'a GeneratorApiDartInternalConfig,
    ) -> WireDartCodecCstGeneratorContext<'a> {
        WireDartCodecCstGeneratorContext {
            mir_pack,
            config: wire_dart_config,
            wire_rust_config,
            api_dart_config,
        }
    }
}
