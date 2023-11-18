use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeTrait for IrTypePrimitiveList {
    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm {
            match self.primitive {
                IrTypePrimitive::I64 | IrTypePrimitive::U64 => {
                    "Object /* BigInt64Array */".to_owned()
                }
                _ => self.dart_api_type(),
            }
        } else {
            format!("ffi.Pointer<wire_{}>", self.safe_ident())
        }
    }
}
