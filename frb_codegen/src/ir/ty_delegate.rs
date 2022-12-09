use convert_case::{Case, Casing};

use crate::ir::*;
use crate::target::Target;

#[cfg(feature = "chrono")]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum IrTypeTime {
    Local,
    Utc,
    Naive,
    Duration,
}

#[cfg(feature = "chrono")]
impl IrTypeTime {
    fn safe_ident(&self) -> &str {
        match self {
            IrTypeTime::Local => "Local",
            IrTypeTime::Utc => "Utc",
            IrTypeTime::Duration => "Duration",
            IrTypeTime::Naive => "Naive",
        }
    }
}

/// types that delegate to another type
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum IrTypeDelegate {
    Array(IrTypeDelegateArray),
    String,
    StringList,
    ZeroCopyBufferVecPrimitive(IrTypePrimitive),
    PrimitiveEnum {
        ir: IrTypeEnumRef,
        /// Allows for `#[repr]`'s other than [i32]
        repr: IrTypePrimitive,
    },
    #[cfg(feature = "chrono")]
    Time(IrTypeTime),
    #[cfg(feature = "uuid")]
    Uuid,
    #[cfg(feature = "uuid")]
    Uuids,
}
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum IrTypeDelegateArray {
    GeneralArray {
        length: usize,
        general: Box<IrType>,
    },
    PrimitiveArray {
        length: usize,
        primitive: IrTypePrimitive,
    },
}

impl IrTypeDelegateArray {
    pub fn get_delegate(&self) -> IrType {
        match self {
            IrTypeDelegateArray::GeneralArray { general, .. } => {
                IrType::GeneralList(IrTypeGeneralList {
                    inner: general.clone(),
                })
            }
            IrTypeDelegateArray::PrimitiveArray { primitive, .. } => {
                IrType::PrimitiveList(IrTypePrimitiveList {
                    primitive: primitive.clone(),
                })
            }
        }
    }

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

    pub fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, length } => {
                format!("{}_array_{length}", general.dart_api_type())
            }
            IrTypeDelegateArray::PrimitiveArray { primitive, length } => {
                format!("{}_array_{length}", primitive.safe_ident())
            }
        }
    }

    pub fn inner_dart_api_type(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, .. } => general.dart_api_type(),
            IrTypeDelegateArray::PrimitiveArray { primitive, .. } => {
                use IrTypePrimitive::*;
                match &primitive {
                    U8 | I8 | U16 | I16 | U32 | I32 | Usize => "int",
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

    pub fn length(&self) -> usize {
        *match self {
            IrTypeDelegateArray::GeneralArray { length, .. } => length,
            IrTypeDelegateArray::PrimitiveArray { length, .. } => length,
        }
    }
}

impl IrTypeDelegate {
    pub fn get_delegate(&self) -> IrType {
        match self {
            IrTypeDelegate::Array(array) => array.get_delegate(),
            IrTypeDelegate::String => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(primitive) => {
                IrType::PrimitiveList(IrTypePrimitiveList {
                    primitive: primitive.clone(),
                })
            }
            IrTypeDelegate::StringList => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::PrimitiveEnum { repr, .. } => IrType::Primitive(repr.clone()),
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(_) => IrType::Primitive(IrTypePrimitive::I64),
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuid => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuids => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
        }
    }
}

impl IrTypeTrait for IrTypeDelegate {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.get_delegate().visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => array.safe_ident(),
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::StringList => "StringList".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer_".to_owned() + &self.get_delegate().dart_api_type()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.safe_ident(),
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => format!("Chrono_{}", ir.safe_ident()),
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuid => "Uuid".to_owned(),
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuids => "Uuids".to_owned(),
        }
    }

    fn dart_api_type(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => array.dart_api_type(),
            IrTypeDelegate::String => "String".to_string(),
            IrTypeDelegate::StringList => "List<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => self.get_delegate().dart_api_type(),
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.dart_api_type(),
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeTime::Local | IrTypeTime::Utc | IrTypeTime::Naive => "DateTime".to_string(),
                IrTypeTime::Duration => "Duration".to_string(),
            },
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuid => "UuidValue".to_owned(),
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuids => "List<UuidValue>".to_owned(),
        }
    }

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
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeTime::Naive => "chrono::NaiveDateTime".to_owned(),
                IrTypeTime::Local => "chrono::DateTime::<chrono::Local>".to_owned(),
                IrTypeTime::Utc => "chrono::DateTime::<chrono::Utc>".to_owned(),
                IrTypeTime::Duration => "chrono::Duration".to_owned(),
            },
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuid => "uuid::Uuid".to_owned(),
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuids => "Vec<uuid::Uuid>".to_owned(),
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
