use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::entrypoint::generate_serialize_inputs;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;

pub(crate) struct PdeWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for PdeWireDartCodecEntrypoint
{
    fn generate(
        &self,
        _context: WireDartGeneratorContext,
        _types: &[MirType],
        _mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        None
    }
}

impl WireDartCodecEntrypointTrait<'_> for PdeWireDartCodecEntrypoint {
    fn generate_dart2rust_inner_func_stmt(&self, func: &MirFunc, _wire_func_name: &str) -> String {
        let serialize_inputs = generate_serialize_inputs(func);
        let (maybe_port, maybe_return, maybe_bang) = if has_port_argument(func.mode) {
            (", port: port_", "", "")
        } else {
            ("", "return ", "!")
        };
        let func_id = func.id.unwrap();
        format!(
            "
            final serializer = SseSerializer(generalizedFrbRustBinding);{serialize_inputs}
            {maybe_return}pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: {func_id}{maybe_port}){maybe_bang};
            "
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::func::{
        MirFuncArgMode, MirFuncImplMode, MirFuncInput, MirFuncMode, MirFuncOutput, MirFuncOwnerInfo,
    };
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::utils::namespace::Namespace;

    fn func(mode: MirFuncMode, has_input: bool) -> MirFunc {
        MirFunc {
            namespace: Namespace::default(),
            name: MirIdent::new("calculate".into(), None),
            id: Some(42),
            inputs: has_input
                .then(|| MirFuncInput {
                    ownership_mode: None,
                    inner: MirField {
                        ty: MirType::Primitive(MirTypePrimitive::I32),
                        name: MirIdent::new("input_value".into(), None),
                        is_final: false,
                        is_rust_public: None,
                        comments: vec![],
                        default: None,
                        settings: MirFieldSettings::default(),
                    },
                    needs_extend_lifetime: false,
                })
                .into_iter()
                .collect(),
            output: MirFuncOutput {
                normal: MirType::Primitive(MirTypePrimitive::I32),
                error: None,
            },
            owner: MirFuncOwnerInfo::Function,
            mode,
            stream_dart_await: false,
            rust_async: false,
            initializer: false,
            init_dart_code: None,
            arg_mode: MirFuncArgMode::Named,
            accessor: None,
            comments: vec![],
            codec_mode_pack: CodecModePack {
                dart2rust: CodecMode::Pde,
                rust2dart: CodecMode::Pde,
            },
            rust_call_code: None,
            rust_aop_after: None,
            impl_mode: MirFuncImplMode::Normal,
            src_lineno_pseudo: 0,
        }
    }

    fn assert_cleanup_precedes_pde_handoff(output: &str) {
        let catch_start = output.find("} catch (_) {").unwrap();
        let dispose = output.find("serializer.dispose();").unwrap();
        let rethrow = output.find("rethrow;").unwrap();
        let catch_end = rethrow + output[rethrow..].find('}').unwrap();
        let ffi = output.find("pdeCallFfi(").unwrap();

        assert!(catch_start < dispose);
        assert!(dispose < rethrow);
        assert!(rethrow < catch_end);
        assert!(catch_end < ffi);
    }

    /// Keeps serializer cleanup scoped to encoding before normal and sync PDE calls.
    #[test]
    fn dart2rust_generation_cleans_up_encoding_failures_before_pde_handoff() {
        for mode in [MirFuncMode::Normal, MirFuncMode::Sync] {
            let output = PdeWireDartCodecEntrypoint
                .generate_dart2rust_inner_func_stmt(&func(mode, true), "unused");

            assert_cleanup_precedes_pde_handoff(&output);
            assert_eq!(output.contains("port: port_"), mode == MirFuncMode::Normal);
            assert_eq!(
                output.contains("return pdeCallFfi("),
                mode == MirFuncMode::Sync
            );
        }
    }

    /// Omits encoding cleanup when PDE calls have no serialized inputs.
    #[test]
    fn dart2rust_generation_omits_encoding_cleanup_without_pde_inputs() {
        let output = PdeWireDartCodecEntrypoint
            .generate_dart2rust_inner_func_stmt(&func(MirFuncMode::Normal, false), "unused");

        assert!(!output.contains("catch (_)") && !output.contains("serializer.dispose()"));
    }
}
