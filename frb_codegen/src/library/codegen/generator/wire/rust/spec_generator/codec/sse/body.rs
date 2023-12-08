use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::lang::rust::RustLang;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::sse::ty::{CodecSseTy, CodecSseTyContext};
use crate::codegen::generator::codec::structs::EncodeOrDecode;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::codec::sse::ty::CodecSseTyTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(super) fn generate_encode_or_decode(
    context: WireRustCodecSseGeneratorContext,
    types: &[IrType],
    mode: EncodeOrDecode,
) -> WireRustCodecOutputSpec {
    let mut inner = Default::default();
    inner += (types.iter())
        .map(|ty| generate_encode_or_decode_for_type(ty, context, mode))
        .collect();
    WireRustCodecOutputSpec { inner }
}

fn generate_encode_or_decode_for_type(
    ty: &IrType,
    context: WireRustCodecSseGeneratorContext,
    mode: EncodeOrDecode,
) -> Acc<WireRustOutputCode> {
    let rust_api_type = ty.rust_api_type();
    let safe_ident = ty.safe_ident();
    let body = CodecSseTy::new(ty.clone(), CodecSseTyContext::new(context.ir_pack))
        .generate(&Lang::RustLang(RustLang), mode);

    let code = match mode {
        EncodeOrDecode::Encode => format!(
            "
            void _sse_encode_{safe_ident}(Serializer serializer, {dart_api_type} src) {{
                {body}
            }}
            "
        ),
        EncodeOrDecode::Decode => format!(
            "
            {dart_api_type} _sse_decode_{safe_ident}(Serializer serializer) {{
                {body}
            }}
            "
        ),
    };

    Acc::new_common(code.into())
}
