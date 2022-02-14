use crate::ir::*;

#[derive(Debug, Clone)]
pub enum ApiTypePrimitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
    Unit,
}

impl ApiTypeChild for ApiTypePrimitive {
    fn visit_sub_types<F: FnMut(&ApiType) -> bool>(&self, _f: &mut F, _api_file: &ApiFile) {}

    fn safe_ident(&self) -> String {
        self.rust_api_type()
    }

    fn dart_api_type(&self) -> String {
        match self {
            ApiTypePrimitive::U8
            | ApiTypePrimitive::I8
            | ApiTypePrimitive::U16
            | ApiTypePrimitive::I16
            | ApiTypePrimitive::U32
            | ApiTypePrimitive::I32
            | ApiTypePrimitive::U64
            | ApiTypePrimitive::I64 => "int",
            ApiTypePrimitive::F32 | ApiTypePrimitive::F64 => "double",
            ApiTypePrimitive::Bool => "bool",
            ApiTypePrimitive::Unit => "void",
        }
        .to_string()
    }

    fn dart_wire_type(&self) -> String {
        match self {
            ApiTypePrimitive::Bool => "int".to_owned(),
            _ => self.dart_api_type(),
        }
    }

    fn rust_api_type(&self) -> String {
        self.rust_wire_type()
    }

    fn rust_wire_type(&self) -> String {
        match self {
            ApiTypePrimitive::U8 => "u8",
            ApiTypePrimitive::I8 => "i8",
            ApiTypePrimitive::U16 => "u16",
            ApiTypePrimitive::I16 => "i16",
            ApiTypePrimitive::U32 => "u32",
            ApiTypePrimitive::I32 => "i32",
            ApiTypePrimitive::U64 => "u64",
            ApiTypePrimitive::I64 => "i64",
            ApiTypePrimitive::F32 => "f32",
            ApiTypePrimitive::F64 => "f64",
            ApiTypePrimitive::Bool => "bool",
            ApiTypePrimitive::Unit => "unit",
        }
        .to_string()
    }
}

impl ApiTypePrimitive {
    /// Representations of primitives within Dart's pointers, e.g. `ffi.Pointer<ffi.Uint8>`.
    /// This is enforced on Dart's side, and should be used instead of `dart_wire_type`
    /// whenever primitives are put behind a pointer.
    pub fn dart_native_type(&self) -> &'static str {
        match self {
            ApiTypePrimitive::U8 | ApiTypePrimitive::Bool => "ffi.Uint8",
            ApiTypePrimitive::I8 => "ffi.Int8",
            ApiTypePrimitive::U16 => "ffi.Uint16",
            ApiTypePrimitive::I16 => "ffi.Int16",
            ApiTypePrimitive::U32 => "ffi.Uint32",
            ApiTypePrimitive::I32 => "ffi.Int32",
            ApiTypePrimitive::U64 => "ffi.Uint64",
            ApiTypePrimitive::I64 => "ffi.Int64",
            ApiTypePrimitive::F32 => "ffi.Float",
            ApiTypePrimitive::F64 => "ffi.Double",
            ApiTypePrimitive::Unit => "ffi.Void",
        }
    }
    pub fn try_from_rust_str(s: &str) -> Option<Self> {
        match s {
            "u8" => Some(ApiTypePrimitive::U8),
            "i8" => Some(ApiTypePrimitive::I8),
            "u16" => Some(ApiTypePrimitive::U16),
            "i16" => Some(ApiTypePrimitive::I16),
            "u32" => Some(ApiTypePrimitive::U32),
            "i32" => Some(ApiTypePrimitive::I32),
            "u64" => Some(ApiTypePrimitive::U64),
            "i64" => Some(ApiTypePrimitive::I64),
            "f32" => Some(ApiTypePrimitive::F32),
            "f64" => Some(ApiTypePrimitive::F64),
            "bool" => Some(ApiTypePrimitive::Bool),
            "()" => Some(ApiTypePrimitive::Unit),
            _ => None,
        }
    }
}
