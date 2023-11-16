use crate::codegen::generator::output::c::COutputCode;
use crate::codegen::generator::output::dart::DartOutputCode;
use crate::codegen::generator::output::rust::RustOutputCode;

pub(crate) mod c;
pub(crate) mod dart;
pub(crate) mod rust;

pub(crate) struct OutputCode {
    dart: DartOutputCode,
    rust: RustOutputCode,
    c: COutputCode,
}

impl OutputCode {
    pub(crate) fn merge(self, other: Self) -> Self {
        todo!()
    }
}
