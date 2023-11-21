use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::ty::IrType;

crate::ir! {
pub struct IrFunc {
    pub name: NamespacedName,
    pub inputs: Vec<IrField>,
    pub output: IrType,
    pub error_output: Option<IrType>,
    pub owner: IrFuncOwnerInfo,
    pub mode: IrFuncMode,
    pub comments: Vec<IrComment>,
}

#[derive(Copy)]
pub enum IrFuncMode {
    Normal,
    Sync,
    Stream {
        // The index of StreamSink in the function arguments
        argument_index: usize,
    },
}

pub enum IrFuncOwnerInfo {
    Function,
    Method(IrFuncOwnerInfoMethod),
}

pub struct IrFuncOwnerInfoMethod {
    pub(crate) struct_name: String,
    pub(crate) actual_method_name: String,
    pub(crate) mode: IrFuncOwnerInfoMethodMode,
}

pub enum IrFuncOwnerInfoMethodMode {
    Static,
    Instance,
}
}

impl IrFunc {
    pub(crate) fn fallible(&self) -> bool {
        self.error_output.is_some()
    }
}
