use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::lang::dart::DartLang;
use crate::codegen::generator::codec::sse::lang::{Lang, LangTrait};
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::base::WireDartCodecSseGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::body::generate_encode_or_decode;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) struct SseWireDartCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for SseWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
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
    fn generate_dart2rust_inner_func_stmt(&self, func: &IrFunc, wire_func_name: &str) -> String {
        let maybe_serialize_port = if has_port_argument(func.mode) {
            "serializer.serialize_TODO(port_);"
        } else {
            ""
        };

        let serialize_inputs = (func.inputs.iter())
            .map(|input| format!("{};", DartLang.call_encode(&input.ty, &input.name.raw)))
            .join("\n");

        format!(
            "
            final serializer = SseSerializer();
            {maybe_serialize_port}{serialize_inputs}
            final raw_ = serializer.intoRaw();
            return {wire_func_name}(raw_.ptr, raw_.rustVecLen, raw_.dataLen);
            "
        )
    }
}
