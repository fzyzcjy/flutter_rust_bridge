use crate::ir::*;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct IrDartAnnotation {
    pub content: String,
    pub library: Option<IrDartImport>,
}
