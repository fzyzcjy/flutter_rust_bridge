use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;

mod io;
mod wasm;

pub(super) fn generate() -> Acc<WireDartOutputCode> {
    Acc {
        io: io::generate(),
        wasm: wasm::generate(),
        ..Default::default()
    }
}
