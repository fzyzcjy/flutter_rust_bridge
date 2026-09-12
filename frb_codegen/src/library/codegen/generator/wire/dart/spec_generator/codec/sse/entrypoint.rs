use crate::codegen::generator::codec::sse::lang::dart::DartLang;
use crate::codegen::generator::codec::sse::lang::LangTrait;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::body::generate_encode_or_decode;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;
use itertools::Itertools;

pub(crate) struct SseWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for SseWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[MirType],
        mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        Some(generate_encode_or_decode(
            context.as_wire_dart_codec_sse_context(),
            types,
            mode,
        ))
    }
}

impl WireDartCodecEntrypointTrait<'_> for SseWireDartCodecEntrypoint {
    fn generate_dart2rust_inner_func_stmt(&self, func: &MirFunc, wire_func_name: &str) -> String {
        let serialize_inputs = generate_serialize_inputs(func);
        let maybe_port = if has_port_argument(func.mode) {
            "port_, "
        } else {
            ""
        };
        format!(
            "
            final serializer = SseSerializer(generalizedFrbRustBinding);{serialize_inputs}
            final raw_ = serializer.intoRaw();
            return wire.{wire_func_name}({maybe_port}raw_.ptr, raw_.rustVecLen, raw_.dataLen);
            "
        )
    }
}

pub(crate) fn generate_serialize_inputs(func: &MirFunc) -> String {
    if func.inputs.is_empty() {
        return String::new();
    }

    let inputs = (func.inputs.iter())
        .map(|input| {
            format!(
                "{};",
                DartLang.call_encode(&input.inner.ty, &input.inner.name.dart_style())
            )
        })
        .join("\n");
    format!(
        "try {{
            {inputs}
        }} catch (_) {{
            serializer.dispose();
            rethrow;
        }}"
    )
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
                dart2rust: CodecMode::Sse,
                rust2dart: CodecMode::Sse,
            },
            rust_call_code: None,
            rust_aop_after: None,
            impl_mode: MirFuncImplMode::Normal,
            src_lineno_pseudo: 0,
        }
    }

    fn assert_cleanup_precedes_transfer_and_ffi(output: &str) {
        let catch_start = output.find("} catch (_) {").unwrap();
        let dispose = output.find("serializer.dispose();").unwrap();
        let rethrow = output.find("rethrow;").unwrap();
        let catch_end = rethrow + output[rethrow..].find('}').unwrap();
        let transfer = output.find("serializer.intoRaw()").unwrap();
        let ffi = output.find("return wire.calculate(").unwrap();

        assert!(catch_start < dispose);
        assert!(dispose < rethrow);
        assert!(rethrow < catch_end);
        assert!(catch_end < transfer);
        assert!(transfer < ffi);
    }

    /// Keeps serializer cleanup scoped to encoding before SSE transfer and FFI calls.
    #[test]
    fn dart2rust_generation_cleans_up_encoding_failures_before_sse_handoff() {
        for mode in [MirFuncMode::Normal, MirFuncMode::Sync] {
            let output = SseWireDartCodecEntrypoint
                .generate_dart2rust_inner_func_stmt(&func(mode, true), "calculate");

            assert_cleanup_precedes_transfer_and_ffi(&output);
            assert_eq!(
                output.contains("wire.calculate(port_,"),
                mode == MirFuncMode::Normal
            );
        }
    }

    /// Omits encoding cleanup when SSE calls have no serialized inputs.
    #[test]
    fn dart2rust_generation_omits_encoding_cleanup_without_sse_inputs() {
        let output = SseWireDartCodecEntrypoint
            .generate_dart2rust_inner_func_stmt(&func(MirFuncMode::Normal, false), "calculate");

        assert!(!output.contains("catch (_)") && !output.contains("serializer.dispose()"));
    }
}
