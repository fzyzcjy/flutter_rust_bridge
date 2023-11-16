use crate::codegen::generator::dart_api::base::*;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use enum_dispatch::enum_dispatch;
use syn::token::Dyn;

#[enum_dispatch]
pub(crate) trait DartApiGeneratorDeclTrait {
    fn dart_api_type(&self) -> String;
}

impl<'a> DartApiGeneratorDeclTrait for BoxedDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = DartApiGenerator::new(*self.ir.inner.clone(), self.context.ir_pack);
        inner.dart_api_type()
    }
}

impl<'a> DartApiGeneratorDeclTrait for DartOpaqueDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        "Object".to_owned()
    }
}

impl<'a> DartApiGeneratorDeclTrait for DelegateDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir {
            IrTypeDelegate::Array(array) => array.dart_api_type(),
            IrTypeDelegate::String => "String".to_string(),
            IrTypeDelegate::StringList => "List<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => self.get_delegate().dart_api_type(),
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                ir.dart_api_type()
            }
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeDelegateTime::Local | IrTypeDelegateTime::Utc | IrTypeDelegateTime::Naive => {
                    "DateTime".to_string()
                }
                IrTypeDelegateTime::Duration => "Duration".to_string(),
            },
            IrTypeDelegate::TimeList(
                IrTypeDelegateTime::Local | IrTypeDelegateTime::Utc | IrTypeDelegateTime::Naive,
            ) => "List<DateTime>".to_string(),
            IrTypeDelegate::TimeList(IrTypeDelegateTime::Duration) => "List<Duration>".to_string(),
            IrTypeDelegate::Uuid => "UuidValue".to_owned(),
            IrTypeDelegate::Uuids => "List<UuidValue>".to_owned(),
            IrTypeDelegate::Backtrace => "String".to_string(),
            IrTypeDelegate::Anyhow => "FrbAnyhowException".to_string(),
        }
    }
}

impl<'a> DartApiGeneratorDeclTrait for DynamicDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        "dynamic".to_owned()
    }
}

impl<'a> DartApiGeneratorDeclTrait for EnumRefDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        self.ir.ident.0.to_string()
    }
}

impl<'a> DartApiGeneratorDeclTrait for GeneralListDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = DartApiGenerator::new(*self.ir.inner.clone(), self.context.ir_pack);
        format!("List<{}>", inner.dart_api_type())
    }
}

impl<'a> DartApiGeneratorDeclTrait for OptionalDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = DartApiGenerator::new(*self.ir.inner.clone(), self.context.ir_pack);
        format!("{}?", inner.dart_api_type())
    }
}

impl<'a> DartApiGeneratorDeclTrait for OptionalListDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = DartApiGenerator::new(*self.ir.inner.clone(), self.context.ir_pack);
        format!("List<{}?>", inner.dart_api_type())
    }
}

impl<'a> DartApiGeneratorDeclTrait for PrimitiveDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir {
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
        .to_owned()
    }
}

impl<'a> DartApiGeneratorDeclTrait for PrimitiveListDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir.primitive {
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
            _ => panic!("does not support {:?} yet", &self.ir.primitive),
        }
        .to_string()
    }
}

impl<'a> DartApiGeneratorDeclTrait for RecordDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let values = self
            .ir
            .values
            .iter()
            .map(|ty| DartApiGenerator::new(ty.clone(), self.context.ir_pack).dart_api_type())
            .collect::<Vec<_>>()
            .join(",");
        if self.ir.values.len() == 1 {
            format!("({values},)")
        } else {
            format!("({values})")
        }
    }
}

impl<'a> DartApiGeneratorDeclTrait for RustOpaqueDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = DartApiGenerator::new(*self.ir.inner.clone(), self.context.ir_pack);
        inner.dart_api_type()
    }
}

impl<'a> DartApiGeneratorDeclTrait for StructRefDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        self.ir.ident.0.to_string()
    }
}

impl<'a> DartApiGeneratorDeclTrait for UnencodableDartApiGenerator<'a> {
    fn dart_api_type(&self) -> String {
        unimplemented!()
    }
}
