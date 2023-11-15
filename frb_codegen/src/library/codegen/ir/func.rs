use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::IrType;

crate::ir! {
pub struct IrFunc {
    pub name: String,
    pub inputs: Vec<IrField>,
    pub output: IrType,
    pub error_output: Option<IrType>,
    pub fallible: bool,
    pub mode: IrFuncMode,
    pub comments: Vec<IrComment>,
}

pub enum IrFuncMode {
    Normal,
    Sync,
    Stream {
        // The index of StreamSink in the function arguments
        argument_index: usize,
    },
}
}
