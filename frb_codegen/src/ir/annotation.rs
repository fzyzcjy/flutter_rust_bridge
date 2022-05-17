use crate::ir::*;

#[derive(Debug, Clone)]
pub struct IrAnnotation {
    pub content: String,
    pub library: Option<IrImport>,
}
