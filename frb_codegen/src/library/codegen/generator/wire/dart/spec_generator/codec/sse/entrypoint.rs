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
    (func.inputs.iter())
        .map(|input| {
            format!(
                "{};",
                DartLang.call_encode(&input.inner.ty, &input.inner.name.dart_style())
            )
        })
        .join("\n")
}
