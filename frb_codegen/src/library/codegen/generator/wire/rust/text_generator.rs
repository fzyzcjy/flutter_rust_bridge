use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::WireRustOutputSpec;

// Call it "text", not "code", because the whole codegen is generating code,
// and we want to emphasize we are generating final output text here.
pub(super) struct WireRustOutputText {
    primary: Acc<Option<String>>,
}

pub(super) fn generate(spec: WireRustOutputSpec) -> WireRustOutputText {
    todo!()
}
