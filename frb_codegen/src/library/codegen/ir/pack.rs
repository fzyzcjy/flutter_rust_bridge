use std::collections::HashMap;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::enumeration::IrEnum;
use crate::codegen::ir::ty::structure::IrStruct;

pub type IrStructPool = HashMap<String, IrStruct>;
pub type IrEnumPool = HashMap<String, IrEnum>;

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct IrPack {
    pub funcs: Vec<IrFunc>,
    pub struct_pool: IrStructPool,
    pub enum_pool: IrEnumPool,
    pub has_executor: bool,
}
