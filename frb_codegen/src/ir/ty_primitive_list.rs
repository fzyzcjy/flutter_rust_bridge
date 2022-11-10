use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrTypePrimitiveList {
    pub primitive: IrTypePrimitive,
}

impl IrTypeTrait for IrTypePrimitiveList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, _ir_file: &IrFile) {
        f(&IrType::Primitive(self.primitive.clone()));
    }

    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        match &self.primitive {
            IrTypePrimitive::U8 => "Uint8List",
            IrTypePrimitive::I8 => "Int8List",
            IrTypePrimitive::U16 => "Uint16List",
            IrTypePrimitive::I16 => "Int16List",
            IrTypePrimitive::U32 => "Uint32List",
            IrTypePrimitive::I32 => "Int32List",
            IrTypePrimitive::U64 => "Uint64List",
            IrTypePrimitive::I64 => "Int64List",
            IrTypePrimitive::F32 => "Float32List",
            IrTypePrimitive::F64 => "Float64List",
            _ => panic!("does not support {:?} yet", &self.primitive),
        }
        .to_string()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            self.dart_api_type()
        } else {
            format!("ffi.Pointer<wire_{}>", self.safe_ident())
        }
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            match self.primitive {
                IrTypePrimitive::Bool | IrTypePrimitive::Unit => "JsValue".into(),
                _ => format!("Box<[{}]>", self.primitive.rust_api_type()),
            }
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        !target.is_wasm()
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
            IrTypePrimitive::I32 => "js_sys::Int32Array",
            IrTypePrimitive::U64 => "js_sys::BigUint64Array",
            IrTypePrimitive::I64 => "js_sys::BigInt64Array",
            IrTypePrimitive::F32 => "js_sys::Float32Array",
            IrTypePrimitive::F64 => "js_sys::Float64Array",
            IrTypePrimitive::Bool | IrTypePrimitive::Unit => "js_sys::Array",
        }
    }
}
