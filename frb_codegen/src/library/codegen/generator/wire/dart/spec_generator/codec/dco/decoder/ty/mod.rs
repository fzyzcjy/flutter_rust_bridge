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
#[cfg(test)]
pub(crate) mod test_utils {
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::WireDartCodecDcoGeneratorContext;
    use crate::codegen::ir::mir::pack::MirPack;
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

    pub(crate) fn config() -> GeneratorApiDartInternalConfig {
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

    pub(crate) fn context<'a>(
        mir_pack: &'a MirPack,
        api_dart_config: &'a GeneratorApiDartInternalConfig,
    ) -> WireDartCodecDcoGeneratorContext<'a> {
        WireDartCodecDcoGeneratorContext {
            mir_pack,
            api_dart_config,
        }
    }
}
mod trait_def;

use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartCodecDcoGeneratorDecoderTrait {
    fn generate_impl_decode_body(&self) -> String;
}
