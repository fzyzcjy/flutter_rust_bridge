use crate::codegen::ir::mir::import::MirDartImport;

crate::mir! {
pub struct MirDartAnnotation {
    pub content: String,
    pub library: Option<MirDartImport>,
}
}
