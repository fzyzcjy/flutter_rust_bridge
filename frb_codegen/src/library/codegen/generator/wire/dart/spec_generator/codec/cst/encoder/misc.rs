use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::WireDartCodecCstGeneratorImplTrait;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::WireRustCodecCstGenerator;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

pub(super) fn dart_wire_type_from_rust_wire_type_or_web(
    that: &impl WireDartCodecCstGeneratorImplTrait,
    target: Target,
    web_type: String,
) -> String {
    match target {
        Target::Io => {
            WireRustCodecCstGenerator::new(that.mir_type(), that.context().as_wire_rust_context())
                .rust_wire_type(target)
        }
        Target::Web => web_type,
    }
}
