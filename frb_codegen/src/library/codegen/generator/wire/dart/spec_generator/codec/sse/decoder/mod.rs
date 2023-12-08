use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecOutputSpec;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::base::WireDartCodecSseGeneratorContext;
use crate::codegen::ir::ty::IrType;

mod ty;

pub(crate) fn generate(
    context: WireDartCodecSseGeneratorContext,
    types: &[IrType],
) -> WireDartCodecOutputSpec {
    WireDartCodecOutputSpec { inner: todo!() }
}
