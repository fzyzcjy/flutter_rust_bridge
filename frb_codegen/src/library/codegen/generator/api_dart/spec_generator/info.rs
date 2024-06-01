use crate::codegen::generator::api_dart::spec_generator::base::*;
use crate::codegen::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateArray, MirTypeDelegateArrayMode, MirTypeDelegatePrimitiveEnum,
    MirTypeDelegateTime,
};
use crate::codegen::mir::ty::general_list::MirTypeGeneralList;
use crate::codegen::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::mir::ty::{MirType, MirTypeTrait};
use convert_case::{Case, Casing};
use enum_dispatch::enum_dispatch;
use itertools::Itertools;

#[enum_dispatch]
pub(crate) trait ApiDartGeneratorInfoTrait {
    fn dart_api_type(&self) -> String;

    fn dart_import(&self) -> Option<String> {
        None
    }
}

impl<'a> ApiDartGeneratorInfoTrait for BoxedApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        inner.dart_api_type()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for DartFnApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        format!(
            "FutureOr<{}> Function({})",
            ApiDartGenerator::new(self.ir.output.normal.clone(), self.context).dart_api_type(),
            (self.ir.inputs.iter())
                .map(|x| ApiDartGenerator::new(x.clone(), self.context).dart_api_type())
                .join(", "),
        )
    }
}

impl<'a> ApiDartGeneratorInfoTrait for DartOpaqueApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        "Object".to_owned()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for DelegateApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir {
            MirTypeDelegate::Array(array) => array.dart_api_type(self.context),
            MirTypeDelegate::String => "String".to_string(),
            MirTypeDelegate::Char => "String".to_string(),
            // MirTypeDelegate::StringList => "List<String>".to_owned(),
            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     ApiDartGenerator::new(self.ir.get_delegate(), self.context).dart_api_type()
            // }
            MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { ir, .. }) => {
                ApiDartGenerator::new(MirType::EnumRef(ir.clone()), self.context).dart_api_type()
            }
            MirTypeDelegate::Time(ir) => match ir {
                MirTypeDelegateTime::Local
                | MirTypeDelegateTime::Utc
                | MirTypeDelegateTime::Naive => "DateTime".to_string(),
                MirTypeDelegateTime::Duration => "Duration".to_string(),
            },
            // MirTypeDelegate::TimeList(
            //     MirTypeDelegateTime::Local | MirTypeDelegateTime::Utc | MirTypeDelegateTime::Naive,
            // ) => "List<DateTime>".to_string(),
            // MirTypeDelegate::TimeList(MirTypeDelegateTime::Duration) => "List<Duration>".to_string(),
            MirTypeDelegate::Uuid => "UuidValue".to_owned(),
            // MirTypeDelegate::Uuids => "List<UuidValue>".to_owned(),
            MirTypeDelegate::Backtrace => "String".to_string(),
            MirTypeDelegate::AnyhowException => "AnyhowException".to_string(),
            MirTypeDelegate::Map(ir) => format!(
                "Map<{}, {}>",
                ApiDartGenerator::new(*ir.key.clone(), self.context).dart_api_type(),
                ApiDartGenerator::new(*ir.value.clone(), self.context).dart_api_type(),
            ),
            MirTypeDelegate::Set(ir) => format!(
                "Set<{}>",
                ApiDartGenerator::new(*ir.inner.clone(), self.context).dart_api_type(),
            ),
            MirTypeDelegate::StreamSink(ir) => format!(
                "RustStreamSink<{}>",
                ApiDartGenerator::new(*ir.inner.clone(), self.context).dart_api_type(),
            ),
            MirTypeDelegate::BigPrimitive(_) => "BigInt".to_owned(),
            MirTypeDelegate::RustAutoOpaqueExplicit(ir) => {
                ApiDartGenerator::new(ir.inner.clone(), self.context).dart_api_type()
            }
        }
    }

    fn dart_import(&self) -> Option<String> {
        match &self.ir {
            MirTypeDelegate::Uuid /*| MirTypeDelegate::Uuids*/ => {
                Some("import 'package:uuid/uuid.dart';".to_owned())
            }
            _ => None,
        }
    }
}

impl MirTypeDelegateArray {
    pub(crate) fn dart_api_type(&self, context: ApiDartGeneratorContext) -> String {
        let length = self.length;
        match &self.mode {
            MirTypeDelegateArrayMode::General(general) => {
                format!(
                    "{}Array{length}",
                    ApiDartGenerator::new(general.clone(), context).dart_api_type()
                )
            }
            MirTypeDelegateArrayMode::Primitive(primitive) => {
                format!(
                    "{}Array{length}",
                    primitive.safe_ident().to_case(Case::Pascal)
                )
            }
        }
    }
}

impl<'a> ApiDartGeneratorInfoTrait for DynamicApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        "dynamic".to_owned()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for EnumRefApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        self.ir.ident.0.name.to_string()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for GeneralListApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        format!("List<{}>", inner.dart_api_type())
    }
}

impl<'a> ApiDartGeneratorInfoTrait for OptionalApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        format!("{}?", inner.dart_api_type())
    }
}

impl<'a> ApiDartGeneratorInfoTrait for PrimitiveApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir {
            MirTypePrimitive::U8
            | MirTypePrimitive::I8
            | MirTypePrimitive::U16
            | MirTypePrimitive::I16
            | MirTypePrimitive::U32
            | MirTypePrimitive::I32 => "int",
            MirTypePrimitive::I64 | MirTypePrimitive::Isize => "PlatformInt64",
            MirTypePrimitive::U64 | MirTypePrimitive::Usize => "BigInt",
            MirTypePrimitive::F32 | MirTypePrimitive::F64 => "double",
            MirTypePrimitive::Bool => "bool",
            MirTypePrimitive::Unit => "void",
        }
        .to_owned()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for PrimitiveListApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        if self.ir.strict_dart_type {
            match &self.ir.primitive {
                MirTypePrimitive::U8 => "Uint8List",
                MirTypePrimitive::I8 => "Int8List",
                MirTypePrimitive::U16 => "Uint16List",
                MirTypePrimitive::I16 => "Int16List",
                MirTypePrimitive::U32 => "Uint32List",
                MirTypePrimitive::I32 => "Int32List",
                MirTypePrimitive::U64 => "Uint64List",
                MirTypePrimitive::I64 => "Int64List",
                MirTypePrimitive::F32 => "Float32List",
                MirTypePrimitive::F64 => "Float64List",
                // frb-coverage:ignore-start
                _ => panic!("does not support {:?} yet", &self.ir.primitive),
                // frb-coverage:ignore-end
            }
            .to_string()
        } else {
            ApiDartGenerator::new(
                MirTypeGeneralList {
                    inner: Box::new(MirType::Primitive(self.ir.primitive.clone())),
                },
                self.context,
            )
            .dart_api_type()
        }
    }
}

impl<'a> ApiDartGeneratorInfoTrait for RecordApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let values = (self.ir.values.iter())
            .map(|ty| ApiDartGenerator::new(ty.clone(), self.context).dart_api_type())
            .collect_vec()
            .join(",");
        let extra_comma = if self.ir.values.len() == 1 { "," } else { "" };
        format!("({values}{extra_comma})")
    }
}

impl<'a> ApiDartGeneratorInfoTrait for RustAutoOpaqueImplicitApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        inner.dart_api_type()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for RustOpaqueApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        self.ir.sanitized_type()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for StructRefApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        self.ir.ident.0.name.to_string()
    }

    fn dart_import(&self) -> Option<String> {
        let st = self.ir.get(self.context.ir_pack);
        Some(
            st.dart_metadata
                .iter()
                .flat_map(|it| &it.library)
                .map(|it| it.to_code())
                .join("\n"),
        )
    }
}
