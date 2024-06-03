use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::lang::rust::RustLang;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::sse::misc::with_sse_extra_types;
use crate::codegen::generator::codec::sse::ty::{CodecSseTy, CodecSseTyContext};
use crate::codegen::generator::codec::structs::{CodecMode, EncodeOrDecode};
use crate::codegen::generator::misc::comments::generate_codec_comments;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::codec::sse::ty::CodecSseTyTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

pub(super) fn generate_encode_or_decode(
    context: WireRustCodecSseGeneratorContext,
    types: &[MirType],
    mode: EncodeOrDecode,
) -> WireRustCodecOutputSpec {
    let types = with_sse_extra_types(types);

    let mut inner = Default::default();
    inner += (types.iter())
        .map(|ty| generate_encode_or_decode_for_type(ty, context, mode))
        .collect();
    WireRustCodecOutputSpec { inner }
}

fn generate_encode_or_decode_for_type(
    ty: &MirType,
    context: WireRustCodecSseGeneratorContext,
    mode: EncodeOrDecode,
) -> Acc<WireRustOutputCode> {
    let rust_api_type = ty.rust_api_type();
    let body = CodecSseTy::new(
        ty.clone(),
        CodecSseTyContext::new(context.mir_pack, context.api_dart_config),
    )
    .generate(&Lang::RustLang(RustLang), mode);
    let codec_comments = generate_codec_comments(CodecMode::Sse);

    if let Some(body) = body {
        let body = body.trim();
        let code  = match mode {
            EncodeOrDecode::Encode => format!(
                "
                impl SseEncode for {rust_api_type} {{
                    {codec_comments}
                    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {{{body}}}
                }}
                "
            ),
            EncodeOrDecode::Decode => format!(
                "
                impl SseDecode for {rust_api_type} {{
                    {codec_comments}
                    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {{{body}}}
                }}
                "
            ),
        };

        Acc::new_common(code.into())
    } else {
        Acc::default()
    }
}
