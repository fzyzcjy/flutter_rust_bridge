use crate::ir::*;

#[derive(Debug, Clone)]
pub struct IrField {
    pub ty: IrType,
    pub name: IrIdent,
    pub non_final: bool,
    pub comments: Vec<IrComment>,
}
