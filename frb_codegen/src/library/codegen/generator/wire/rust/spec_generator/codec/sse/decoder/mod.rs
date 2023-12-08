use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use serde::Serialize;

pub(crate) mod ty;

pub(crate) fn generate(
    context: WireRustCodecSseGeneratorContext,
    types: &[IrType],
) -> WireRustCodecOutputSpec {
    let mut inner = Acc::<Vec<WireRustOutputCode>>::default();
    inner.push_acc(generate_misc());
    inner += (types.iter())
        .map(|ty| generate_for_type(ty, context))
        .collect();
    WireRustCodecOutputSpec { inner }
}

fn generate_misc() -> Acc<WireRustOutputCode> {
    Acc::new_common(
        "
        pub trait SseDecodable {
            fn sse_decode(reader: &mut SseReader) -> Self;
        }
        "
        .into(),
    )
}

fn generate_for_type(
    ty: &IrType,
    context: WireRustCodecSseGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let rust_api_type = ty.rust_api_type();
    let body: String = todo!();

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
