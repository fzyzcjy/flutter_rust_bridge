use convert_case::{Case, Casing};

use crate::ir::*;
use crate::target::Target;

impl IrTypeDelegateTime {
    #[inline]
    pub fn is_duration(&self) -> bool {
        matches!(self, Self::Duration)
    }
    #[inline]
    pub fn is_utc(&self) -> bool {
        matches!(self, Self::Naive | Self::Utc)
    }
}

impl IrTypeDelegateArray {
    pub fn dart_api_type(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, length } => {
                format!("{}Array{length}", general.dart_api_type())
            }
            IrTypeDelegateArray::PrimitiveArray { primitive, length } => {
                format!(
                    "{}Array{length}",
                    primitive.safe_ident().to_case(Case::Pascal)
                )
            }
        }
    }

    pub fn inner_dart_api_type(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, .. } => general.dart_api_type(),
            IrTypeDelegateArray::PrimitiveArray { primitive, .. } => {
                use IrTypePrimitive::*;
                match &primitive {
                    U8 | I8 | U16 | I16 | U32 | I32 | Usize | Isize => "int",
                    U64 | I64 => "BigInt",
                    F32 | F64 => "double",
                    Bool => "bool",
                    Unit => "void",
                }
                .to_string()
            }
        }
    }

    pub fn inner_rust_api_type(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, .. } => general.rust_api_type(),
            IrTypeDelegateArray::PrimitiveArray { primitive, .. } => primitive.rust_api_type(),
        }
    }

    pub fn inner_is_js_value(&self) -> bool {
        match self {
            IrTypeDelegateArray::GeneralArray { general, .. } => general.is_js_value(),
            IrTypeDelegateArray::PrimitiveArray { .. } => false,
        }
    }

    pub fn dart_init_method(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { .. } => format!(
                "{0}.init({1} fill): super(List<{1}>.filled(arraySize,fill));",
                self.dart_api_type(),
                self.inner_dart_api_type()
            ),
            IrTypeDelegateArray::PrimitiveArray { .. } => format!(
                "{0}.init(): super({1}(arraySize));",
                self.dart_api_type(),
                self.get_delegate().dart_api_type()
            ),
        }
    }
}

impl IrTypeTrait for IrTypeDelegate {
    fn dart_wire_type(&self, target: Target) -> String {
        match (self, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Wasm) => "List<String>".into(),
            (IrTypeDelegate::StringList, _) => "ffi.Pointer<wire_StringList>".to_owned(),
            _ => self.get_delegate().dart_wire_type(target),
        }
    }

    fn rust_api_type(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => {
                format!("[{}; {}]", array.inner_rust_api_type(), array.length())
            }
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::StringList => "Vec<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                format!("ZeroCopyBuffer<{}>", self.get_delegate().rust_api_type())
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.rust_api_type(),
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeDelegateTime::Naive => "chrono::NaiveDateTime",
                IrTypeDelegateTime::Local => "chrono::DateTime::<chrono::Local>",
                IrTypeDelegateTime::Utc => "chrono::DateTime::<chrono::Utc>",
                IrTypeDelegateTime::Duration => "chrono::Duration",
            }
            .to_owned(),
            IrTypeDelegate::TimeList(ir) => match ir {
                IrTypeDelegateTime::Naive => "Vec<chrono::NaiveDateTime>",
                IrTypeDelegateTime::Local => "Vec<chrono::DateTime::<chrono::Local>>",
                IrTypeDelegateTime::Utc => "Vec<chrono::DateTime::<chrono::Utc>>",
                IrTypeDelegateTime::Duration => "Vec<chrono::Duration>",
            }
            .to_owned(),
            IrTypeDelegate::Uuid => "uuid::Uuid".to_owned(),
            IrTypeDelegate::Uuids => "Vec<uuid::Uuid>".to_owned(),
            IrTypeDelegate::Backtrace => "String".to_owned(),
            IrTypeDelegate::Anyhow => "String".to_owned(),
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        match (self, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Io) => "wire_StringList".to_owned(),
            (IrTypeDelegate::StringList, Target::Wasm) => "JsValue".into(),
            _ => self.get_delegate().rust_wire_type(target),
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        self.get_delegate().rust_wire_is_pointer(target)
    }
}
