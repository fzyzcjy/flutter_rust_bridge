use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;

pub(crate) struct DcoWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for DcoWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[MirType],
        mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        match mode {
            EncodeOrDecode::Encode => None,
            EncodeOrDecode::Decode => Some(super::decoder::generate(
                context.as_wire_dart_codec_dco_context(),
                types,
            )),
        }
    }
}

impl WireDartCodecEntrypointTrait<'_> for DcoWireDartCodecEntrypoint {
    // frb-coverage:ignore-start
    fn generate_dart2rust_inner_func_stmt(&self, _func: &MirFunc, _wire_func_name: &str) -> String {
        unreachable!()
    }
    // frb-coverage:ignore-end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::wire::dart::internal_config::{
        DartOutputClassNamePack, GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig,
        GeneratorWireDartInternalConfig,
    };
    use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
    use crate::codegen::ir::mir::pack::MirPack;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn pack() -> MirPack {
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

    fn api_dart_config() -> GeneratorApiDartInternalConfig {
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

    /// Generates only Dart decode bindings and suppresses unsupported DCO encoding.
    #[test]
    fn dco_entrypoint_returns_decode_output_and_no_encode_output() {
        let pack = pack();
        let api_dart_config = api_dart_config();
        let wire_dart_config = GeneratorWireDartInternalConfig {
            has_ffigen: false,
            web_enabled: false,
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
        };
        let wire_rust_config = GeneratorWireRustInternalConfig {
            rust_crate_dir: PathBuf::new(),
            web_enabled: false,
            rust_output_path: PathBuf::new(),
            c_symbol_prefix: String::new(),
            has_ffigen: false,
            default_stream_sink_codec: crate::codegen::generator::codec::structs::CodecMode::Dco,
            default_rust_opaque_codec:
                crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode::Nom,
            rust_preamble: String::new(),
        };
        let context = WireDartGeneratorContext {
            mir_pack: &pack,
            config: &wire_dart_config,
            wire_rust_config: &wire_rust_config,
            api_dart_config: &api_dart_config,
        };
        let entrypoint = DcoWireDartCodecEntrypoint;
        let types = [MirType::Primitive(MirTypePrimitive::I32)];

        let decode = entrypoint
            .generate(context, &types, EncodeOrDecode::Decode)
            .expect("DCO decode generation should produce output");
        assert_eq!(decode.inner.common.len(), 1);
        assert_eq!(decode.inner.io.len(), 1);
        assert_eq!(decode.inner.web.len(), 1);
        assert_eq!(decode.inner.common[0].api_impl_class_methods.len(), 1);
        assert!(decode.inner.io[0].api_impl_class_methods.is_empty());
        assert!(decode.inner.web[0].api_impl_class_methods.is_empty());
        assert_eq!(
            decode.inner.common[0].api_impl_class_methods[0].signature,
            "int dco_decode_i_32(dynamic raw)"
        );
        assert_eq!(
            decode.inner.common[0].api_impl_class_methods[0]
                .body
                .as_deref(),
            Some(
                "// Codec=Dco (DartCObject based), see doc to use other codecs\nreturn raw as int;"
            )
        );
        assert!(entrypoint
            .generate(context, &types, EncodeOrDecode::Encode)
            .is_none());
    }
}
