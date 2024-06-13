use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for PrimitiveWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        match self.mir {
            MirTypePrimitive::I64 | MirTypePrimitive::Isize => Acc {
                io: Some("return raw.toInt();".into()),
                web: Some(CAST_NATIVE_BIG_INT.into()),
                ..Default::default()
            },
            MirTypePrimitive::U64 | MirTypePrimitive::Usize => Acc {
                io: Some("return raw.toSigned(64).toInt();".into()),
                web: Some(CAST_NATIVE_BIG_INT.into()),
                ..Default::default()
            },
            _ => "return raw;".into(),
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match &self.mir {
            MirTypePrimitive::I64
            | MirTypePrimitive::U64
            | MirTypePrimitive::Isize
            | MirTypePrimitive::Usize => match target {
                Target::Io => "int".into(),
                Target::Web => "JSAny".into(),
            },
            _ => ApiDartGenerator::new(self.mir.clone(), self.context.as_api_dart_context())
                .dart_api_type(),
        }
    }
}

const CAST_NATIVE_BIG_INT: &str = "return castNativeBigInt(raw);";

/// Representations of primitives within Dart's pointers, e.g. `ffi.Pointer<ffi.Uint8>`.
/// This is enforced on Dart's side, and should be used instead of `dart_wire_type`
/// whenever primitives are put behind a pointer.
pub fn dart_native_type_of_primitive(prim: &MirTypePrimitive) -> &'static str {
    match prim {
        MirTypePrimitive::U8 => "ffi.Uint8",
        MirTypePrimitive::I8 => "ffi.Int8",
        MirTypePrimitive::U16 => "ffi.Uint16",
        MirTypePrimitive::I16 => "ffi.Int16",
        MirTypePrimitive::U32 => "ffi.Uint32",
        MirTypePrimitive::I32 => "ffi.Int32",
        MirTypePrimitive::U64 => "ffi.Uint64",
        MirTypePrimitive::I64 => "ffi.Int64",
        MirTypePrimitive::Usize => "ffi.UintPtr",
        MirTypePrimitive::Isize => "ffi.IntPtr",
        MirTypePrimitive::F32 => "ffi.Float",
        MirTypePrimitive::F64 => "ffi.Double",
        MirTypePrimitive::Bool => "ffi.Bool",
        MirTypePrimitive::Unit => "ffi.Void",
    }
}
