use crate::ir::*;
use crate::target::Target;

impl IrTypeTrait for IrTypePrimitive {
    fn dart_wire_type(&self, target: Target) -> String {
        match self {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 if target == Target::Wasm => {
                "Object".into()
            }
            _ => self.dart_api_type(),
        }
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
