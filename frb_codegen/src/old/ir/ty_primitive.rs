use crate::ir::*;
use crate::target::Target;

impl IrTypeTrait for IrTypePrimitive {
    fn dart_api_type(&self) -> String {
        match self {
            IrTypePrimitive::U8
            | IrTypePrimitive::I8
            | IrTypePrimitive::U16
            | IrTypePrimitive::I16
            | IrTypePrimitive::U32
            | IrTypePrimitive::I32
            | IrTypePrimitive::U64
            | IrTypePrimitive::I64
            | IrTypePrimitive::Usize
            | IrTypePrimitive::Isize => "int",
            IrTypePrimitive::F32 | IrTypePrimitive::F64 => "double",
            IrTypePrimitive::Bool => "bool",
            IrTypePrimitive::Unit => "void",
        }
        .to_string()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match self {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 if target.is_wasm() => "Object".into(),
            _ => self.dart_api_type(),
        }
    }

    fn rust_api_type(&self) -> String {
        self.rust_wire_type(Target::Io)
    }

    fn rust_wire_type(&self, _target: Target) -> String {
        match self {
            IrTypePrimitive::U8 => "u8",
            IrTypePrimitive::I8 => "i8",
            IrTypePrimitive::U16 => "u16",
            IrTypePrimitive::I16 => "i16",
            IrTypePrimitive::U32 => "u32",
            IrTypePrimitive::I32 => "i32",
            IrTypePrimitive::U64 => "u64",
            IrTypePrimitive::Unit => "unit",
            IrTypePrimitive::Usize => "usize",
            IrTypePrimitive::Isize => "isize",
            IrTypePrimitive::I64 => "i64",
            IrTypePrimitive::F32 => "f32",
            IrTypePrimitive::F64 => "f64",
            IrTypePrimitive::Bool => "bool",
        }
        .to_string()
    }

    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
        match self {
            IrTypePrimitive::Unit => String::from("()"),
            _ => self.rust_api_type(),
        }
    }
}

impl IrTypePrimitive {
    /// Representations of primitives within Dart's pointers, e.g. `ffi.Pointer<ffi.Uint8>`.
    /// This is enforced on Dart's side, and should be used instead of `dart_wire_type`
    /// whenever primitives are put behind a pointer.
    pub fn dart_native_type(&self) -> &'static str {
        match self {
            IrTypePrimitive::U8 => "ffi.Uint8",
            IrTypePrimitive::I8 => "ffi.Int8",
            IrTypePrimitive::U16 => "ffi.Uint16",
            IrTypePrimitive::I16 => "ffi.Int16",
            IrTypePrimitive::U32 => "ffi.Uint32",
            IrTypePrimitive::I32 => "ffi.Int32",
            IrTypePrimitive::U64 => "ffi.Uint64",
            IrTypePrimitive::I64 => "ffi.Int64",
            IrTypePrimitive::Usize => "ffi.UintPtr",
            IrTypePrimitive::Isize => "ffi.IntPtr",
            IrTypePrimitive::F32 => "ffi.Float",
            IrTypePrimitive::F64 => "ffi.Double",
            IrTypePrimitive::Bool => "ffi.Bool",
            IrTypePrimitive::Unit => "ffi.Void",
        }
    }
}
