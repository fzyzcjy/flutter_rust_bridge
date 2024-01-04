use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::lang::rust::RustLang;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::sse::misc::with_sse_extra_types;
use crate::codegen::generator::codec::sse::ty::{CodecSseTy, CodecSseTyContext};
use crate::codegen::generator::codec::structs::EncodeOrDecode;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::codec::sse::ty::CodecSseTyTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

pub(super) fn generate_encode_or_decode(
    context: WireRustCodecSseGeneratorContext,
    types: &[IrType],
    mode: EncodeOrDecode,
) -> WireRustCodecOutputSpec {
    let types = with_sse_extra_types(types);

    let mut inner = Default::default();
    inner += (types.iter())
        .filter_map(|ty| generate_encode_or_decode_for_type(ty, context, mode))
        .unique_by(|(rust_api_type, _)| rust_api_type.to_owned())
        .map(|(_, ans)| ans)
        .collect();
    WireRustCodecOutputSpec { inner }
}

fn generate_encode_or_decode_for_type(
    ty: &IrType,
    context: WireRustCodecSseGeneratorContext,
    mode: EncodeOrDecode,
) -> Option<(String, Acc<WireRustOutputCode>)> {
    let rust_api_type = ty.rust_api_type();
    let body = CodecSseTy::new(
        ty.clone(),
        CodecSseTyContext::new(context.ir_pack, context.api_dart_config),
    )
    .generate(&Lang::RustLang(RustLang), mode);

    if let Some(body) = body {
        let body = body.trim();
        let code  = match mode {
            EncodeOrDecode::Encode => format!(
                "
                impl SseEncode for {rust_api_type} {{
                    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {{{body}}}
                }}
                "
            ),
            EncodeOrDecode::Decode => format!(
                "
                impl SseDecode for {rust_api_type} {{
                    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {{{body}}}
                }}
                "
            ),
        };

        Some((rust_api_type, Acc::new_common(code.into())))
    } else {
        None
    }
}
