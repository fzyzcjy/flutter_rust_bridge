use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::entrypoint::generate_serialize_inputs;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;

pub(crate) struct PdeWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for PdeWireDartCodecEntrypoint
{
    fn generate(
        &self,
        _context: WireDartGeneratorContext,
        _types: &[MirType],
        _mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        None
    }
}

impl WireDartCodecEntrypointTrait<'_> for PdeWireDartCodecEntrypoint {
    fn generate_dart2rust_inner_func_stmt(&self, func: &MirFunc, _wire_func_name: &str) -> String {
        let serialize_inputs = generate_serialize_inputs(func);
        let (maybe_port, maybe_return, maybe_bang) = if has_port_argument(func.mode) {
            (", port: port_", "", "")
        } else {
            ("", "return ", "!")
        };
        let func_id = func.id.unwrap();
        format!(
            "
            final serializer = SseSerializer(generalizedFrbRustBinding);{serialize_inputs}
            {maybe_return}pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: {func_id}{maybe_port}){maybe_bang};
            "
        )
    }
}
