use crate::codegen::mir::import::MirDartImport;

crate::ir! {
pub struct MirDartAnnotation {
    pub content: String,
    pub library: Option<MirDartImport>,
}
}
