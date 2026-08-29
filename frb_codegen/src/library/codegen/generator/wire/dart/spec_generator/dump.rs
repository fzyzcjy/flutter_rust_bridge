use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::WireDartCodecCstGenerator;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::library::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub(crate) struct WireDartDumpInfo {
    types: Vec<WireDartDumpInfoType>,
}

#[derive(Serialize)]
pub(crate) struct WireDartDumpInfoType {
    safe_ident: String,
    dart_wire_type: HashMap<Target, String>,
}

pub(super) fn generate_dump_info(
    cache: &MirPackComputedCache,
    context: WireDartGeneratorContext,
) -> WireDartDumpInfo {
    WireDartDumpInfo {
        types: cache
            .distinct_types
            .iter()
            .map(|ty| {
                let gen = WireDartCodecCstGenerator::new(
                    ty.clone(),
                    context.as_wire_dart_codec_cst_context(),
                );
                WireDartDumpInfoType {
                    safe_ident: ty.safe_ident(),
                    dart_wire_type: Target::iter()
                        .map(|target| (target, gen.dart_wire_type(target)))
                        .collect(),
                }
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::func::{
        MirFunc, MirFuncArgMode, MirFuncImplMode, MirFuncMode, MirFuncOutput, MirFuncOwnerInfo,
    };
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;
    use crate::utils::namespace::Namespace;
    use serde_json::json;

    fn func() -> MirFunc {
        MirFunc {
            namespace: Namespace::default(),
            name: MirIdent::new("answer".into(), None),
            id: None,
            inputs: vec![],
            output: MirFuncOutput {
                normal: MirType::Primitive(MirTypePrimitive::I32),
                error: None,
            },
            owner: MirFuncOwnerInfo::Function,
            mode: MirFuncMode::Sync,
            stream_dart_await: false,
            rust_async: false,
            initializer: false,
            init_dart_code: None,
            arg_mode: MirFuncArgMode::Named,
            accessor: None,
            comments: vec![],
            codec_mode_pack: CodecModePack {
                dart2rust: CodecMode::Cst,
                rust2dart: CodecMode::Cst,
            },
            rust_call_code: None,
            rust_aop_after: None,
            impl_mode: MirFuncImplMode::Normal,
            src_lineno_pseudo: 0,
        }
    }

    /// Serializes empty and populated computed caches with every target wire type.
    #[test]
    fn dump_info_collects_distinct_types_and_target_wire_types() {
        let mut pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let context = WireDartGeneratorContext {
            mir_pack: &pack,
            config: &wire_dart_config,
            wire_rust_config: &wire_rust_config,
            api_dart_config: &api_dart_config,
        };

        assert_eq!(
            serde_json::to_value(generate_dump_info(
                &MirPackComputedCache::compute(&pack),
                context
            ))
            .unwrap(),
            json!({"types": []})
        );

        pack.funcs_all.push(func());
        let context = WireDartGeneratorContext {
            mir_pack: &pack,
            config: &wire_dart_config,
            wire_rust_config: &wire_rust_config,
            api_dart_config: &api_dart_config,
        };
        assert_eq!(
            serde_json::to_value(generate_dump_info(
                &MirPackComputedCache::compute(&pack),
                context
            ))
            .unwrap(),
            json!({
                "types": [{
                    "safe_ident": "i_32",
                    "dart_wire_type": {"Io": "int", "Web": "int"}
                }, {
                    "safe_ident": "unit",
                    "dart_wire_type": {"Io": "void", "Web": "void"}
                }]
            })
        );
    }
}
