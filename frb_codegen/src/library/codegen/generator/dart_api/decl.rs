use crate::codegen::generator::dart_api::base::*;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use enum_dispatch::enum_dispatch;
use syn::token::Dyn;

#[enum_dispatch]
pub(crate) trait DartApiGeneratorDeclTrait {
    fn dart_api_type(&self) -> String;
}

impl DartApiGeneratorDeclTrait for BoxedDartApiGenerator {
    fn dart_api_type(&self) -> String {
        self.inner.dart_api_type()
    }
}

impl DartApiGeneratorDeclTrait for DartOpaqueDartApiGenerator {
    fn dart_api_type(&self) -> String {
        "Object".to_owned()
    }
}

impl DartApiGeneratorDeclTrait for DelegateDartApiGenerator {
    fn dart_api_type(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => array.dart_api_type(),
            IrTypeDelegate::String => "String".to_string(),
            IrTypeDelegate::StringList => "List<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => self.get_delegate().dart_api_type(),
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.dart_api_type(),
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeTime::Local | IrTypeTime::Utc | IrTypeTime::Naive => "DateTime".to_string(),
                IrTypeTime::Duration => "Duration".to_string(),
            },
            IrTypeDelegate::TimeList(IrTypeTime::Local | IrTypeTime::Utc | IrTypeTime::Naive) => {
                "List<DateTime>".to_string()
            }
            IrTypeDelegate::TimeList(IrTypeTime::Duration) => "List<Duration>".to_string(),
            IrTypeDelegate::Uuid => "UuidValue".to_owned(),
            IrTypeDelegate::Uuids => "List<UuidValue>".to_owned(),
            IrTypeDelegate::Backtrace => "String".to_string(),
            IrTypeDelegate::Anyhow => "FrbAnyhowException".to_string(),
        }
    }
}

impl DartApiGeneratorDeclTrait for DynamicDartApiGenerator {
    fn dart_api_type(&self) -> String {
        "dynamic".to_owned()
    }
}

impl DartApiGeneratorDeclTrait for EnumRefDartApiGenerator {
    fn dart_api_type(&self) -> String {
        self.name.to_string()
    }
}

impl DartApiGeneratorDeclTrait for GeneralListDartApiGenerator {
    fn dart_api_type(&self) -> String {
        format!("List<{}>", self.inner.dart_api_type())
    }
}

impl DartApiGeneratorDeclTrait for OptionalDartApiGenerator {
    fn dart_api_type(&self) -> String {
        format!("{}?", self.inner.dart_api_type())
    }
}

impl DartApiGeneratorDeclTrait for OptionalListDartApiGenerator {
    fn dart_api_type(&self) -> String {
        format!("List<{}?>", self.inner.dart_api_type())
    }
}

impl DartApiGeneratorDeclTrait for PrimitiveDartApiGenerator {
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
    }
}

impl DartApiGeneratorDeclTrait for PrimitiveListDartApiGenerator {
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
}

impl DartApiGeneratorDeclTrait for RecordDartApiGenerator {
    fn dart_api_type(&self) -> String {
        let values = self
            .values
            .iter()
            .map(IrType::dart_api_type)
            .collect::<Vec<_>>()
            .join(",");
        if self.values.len() == 1 {
            format!("({values},)")
        } else {
            format!("({values})")
        }
    }
}

impl DartApiGeneratorDeclTrait for RustOpaqueDartApiGenerator {
    fn dart_api_type(&self) -> String {
        self.inner_dart.clone()
    }
}

impl DartApiGeneratorDeclTrait for StructRefDartApiGenerator {
    fn dart_api_type(&self) -> String {
        self.name.to_string()
    }
}

impl DartApiGeneratorDeclTrait for UnencodableDartApiGenerator {
    fn dart_api_type(&self) -> String {
        unimplemented!()
    }
}
