use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use serde::Serialize;

pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireDartOutputSpecCodecSseDecoder {
    pub impl_decode: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustCodecSseGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecCodecSseDecoder {
    WireDartOutputSpecCodecSseDecoder {
        impl_decode: generate_impl_decode(&cache.distinct_input_types, context),
    }
}

pub(crate) fn generate_impl_decode(
    types: &[IrType],
    context: WireRustCodecSseGeneratorContext,
) -> Acc<Vec<WireRustOutputCode>> {
    let mut lines = Acc::<Vec<WireRustOutputCode>>::default();
    lines.push_acc(generate_impl_decode_misc());
    lines += (types.iter())
        .map(|ty| generate_impl_decode_for_type(ty, context))
        .collect();
    lines
}

fn generate_impl_decode_misc() -> Acc<WireRustOutputCode> {
    Acc::new_common(
        "
        pub trait SseDecodable {
            fn sse_decode(reader: &mut SseReader) -> Self;
        }
        "
        .into(),
    )
}

fn generate_impl_decode_for_type(
    ty: &IrType,
    context: WireRustCodecSseGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let rust_api_type = ty.rust_api_type();
    let body = TODO;

    Acc::new_common(
        format!(
            "
            impl SseDecodable for {rust_api_type} {{
                fn sse_decode(reader: &mut SseReader) -> Self {{
                    {body}
                }}
            }}
            "
        )
        .into(),
    )
}
