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

impl IrTypeTrait for IrTypeDelegate {
    fn dart_wire_type(&self, target: Target) -> String {
        match (self, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Wasm) => "List<String>".into(),
            (IrTypeDelegate::StringList, _) => "ffi.Pointer<wire_StringList>".to_owned(),
            _ => self.get_delegate().dart_wire_type(target),
        }
    }
}
