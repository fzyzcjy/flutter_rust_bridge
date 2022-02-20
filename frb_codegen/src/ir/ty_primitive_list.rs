use crate::ir::*;
use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
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

    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<wire_{}>", self.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }

    fn wasm_wire_type(&self) -> String {
        match &self.primitive {
            IrTypePrimitive::U8 => "Box<[u8]>",
            IrTypePrimitive::I8 => "Box<[i8]>",
            IrTypePrimitive::U16 => "Box<[u16]>",
            IrTypePrimitive::I16 => "Box<[i16]>",
            IrTypePrimitive::U32 => "Box<[u32]>",
            IrTypePrimitive::I32 => "Box<[i32]>",
            IrTypePrimitive::U64 => "Box<[u64]>",
            IrTypePrimitive::I64 => "Box<[i64]>",
            IrTypePrimitive::F32 => "Box<[f32]>",
            IrTypePrimitive::F64 => "Box<[f64]>",
            IrTypePrimitive::Bool => "Box<[bool]>",
            _ => panic!(
                "Vec<{}> is not supported on the web.",
                self.primitive.rust_api_type()
            ),
        }
        .to_owned()
    }

    fn js_wire_type(&self) -> String {
        match &self.primitive {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 => "List<BigInt>".to_owned(),
            _ => self.dart_api_type(),
        }
    }
}
