use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::lang::Lang::RustLang;
use crate::codegen::generator::codec::sse::ty::{CodecSseTy, CodecSseTyContext};
use crate::codegen::generator::codec::structs::EncodeOrDecode;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;

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
    let body =
        CodecSseTy::new(ty, CodecSseTyContext::new(context.ir_pack)).generate(&RustLang, mode);

    Acc::new_common(
        format!(
            // TODO
            "
            {rust_api_type} _sse_decode_{safe_ident}(Serializer serializer) {{
                {body}
            }}
            "
        )
        .into(),
    )
}
