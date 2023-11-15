crate::ir! {
pub struct IrDartAnnotation {
    pub content: String,
    pub library: Option<IrDartImport>,
}
}
