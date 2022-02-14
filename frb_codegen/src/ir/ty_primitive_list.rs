use crate::ir::*;
use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
pub struct ApiTypePrimitiveList {
    pub primitive: ApiTypePrimitive,
}

impl ApiTypeChild for ApiTypePrimitiveList {
    fn visit_children_types<F: FnMut(&ApiType) -> bool>(&self, f: &mut F, _api_file: &ApiFile) {
        f(&ApiType::Primitive(self.primitive.clone()));
    }

    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        match &self.primitive {
            ApiTypePrimitive::U8 => "Uint8List",
            ApiTypePrimitive::I8 => "Int8List",
            ApiTypePrimitive::U16 => "Uint16List",
            ApiTypePrimitive::I16 => "Int16List",
            ApiTypePrimitive::U32 => "Uint32List",
            ApiTypePrimitive::I32 => "Int32List",
            ApiTypePrimitive::U64 => "Uint64List",
            ApiTypePrimitive::I64 => "Int64List",
            ApiTypePrimitive::F32 => "Float32List",
            ApiTypePrimitive::F64 => "Float64List",
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
}
