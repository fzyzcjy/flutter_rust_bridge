use convert_case::{Case, Casing};

use crate::ir::*;
use crate::target::Target;

impl IrTypeDelegateArray {
    pub fn inner_rust_api_type(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, .. } => general.rust_api_type(),
            IrTypeDelegateArray::PrimitiveArray { primitive, .. } => primitive.rust_api_type(),
        }
    }
}

impl IrTypeTrait for IrTypeDelegate {}
