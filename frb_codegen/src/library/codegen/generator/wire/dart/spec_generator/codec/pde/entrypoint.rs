use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::entrypoint::{
    generate_serialize_inputs, SseWireDartCodecEntrypoint,
};
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;

pub(crate) struct PdeWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for PdeWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
        mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        None
    }
}

impl WireDartCodecEntrypointTrait<'_> for PdeWireDartCodecEntrypoint {
    fn generate_dart2rust_inner_func_stmt(&self, func: &IrFunc, _wire_func_name: &str) -> String {
        let serialize_inputs = generate_serialize_inputs(func);
        let maybe_port = if has_port_argument(func.mode) {
            "port_, "
        } else {
            ""
        };
        format!(
            "
            final serializer = SseSerializer(generalizedFrbRustBinding);
            {serialize_inputs}
            return TODO_func({maybe_port}serializer);
            "
        )
    }
}
