use crate::codegen::ir::import::IrDartImport;

crate::ir! {
pub struct IrDartAnnotation {
    pub content: String,
    pub library: Option<IrDartImport>,
}
}
