use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::ir::ty::IrType;

pub(crate) fn generate(
    context: WireRustCodecSseGeneratorContext,
    types: &[IrType],
) -> WireRustCodecOutputSpec {
    WireRustCodecOutputSpec { inner: todo!() }
}
