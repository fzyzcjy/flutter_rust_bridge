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

impl IrTypePrimitiveList {
    pub fn rust_wasm_wire_type(&self) -> &str {
        match &self.primitive {
            IrTypePrimitive::U8 => "js_sys::Uint8Array",
            IrTypePrimitive::I8 => "js_sys::Int8Array",
            IrTypePrimitive::U16 => "js_sys::Uint16Array",
            IrTypePrimitive::I16 => "js_sys::Int16Array",
            IrTypePrimitive::U32 | IrTypePrimitive::Usize => "js_sys::Uint32Array",
            IrTypePrimitive::I32 | IrTypePrimitive::Isize => "js_sys::Int32Array",
            IrTypePrimitive::U64 => "js_sys::BigUint64Array",
            IrTypePrimitive::I64 => "js_sys::BigInt64Array",
            IrTypePrimitive::F32 => "js_sys::Float32Array",
            IrTypePrimitive::F64 => "js_sys::Float64Array",
            IrTypePrimitive::Bool | IrTypePrimitive::Unit => "js_sys::Array",
        }
    }
}
