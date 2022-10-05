use convert_case::{Case, Casing};

use crate::ir::*;
use crate::target::Target;

#[cfg(feature = "chrono")]
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
pub enum IrTypeDelegate {
    GeneralArray {
        length: usize,
        general: Box<IrType>,
    },
    PrimitiveArray {
        length: usize,
        primitive: IrTypePrimitive,
    },
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

impl IrTypeDelegate {
    pub fn get_delegate(&self) -> IrType {
        match self {
            IrTypeDelegate::GeneralArray { general, .. } => {
                IrType::GeneralList(IrTypeGeneralList {
                    inner: general.clone(),
                })
            }
            IrTypeDelegate::PrimitiveArray { primitive, .. } => {
                IrType::PrimitiveList(IrTypePrimitiveList {
                    primitive: primitive.clone(),
                })
            }
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

    pub fn inner_dart_api_type(&self) -> String {
        match self {
            IrTypeDelegate::GeneralArray { general, .. } => general.dart_api_type(),
            IrTypeDelegate::PrimitiveArray { primitive, .. } => {
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
            _ => panic!("Unexpected type"),
        }
    }
    pub fn inner_rust_api_type(&self) -> String {
        match self {
            IrTypeDelegate::GeneralArray { general, .. } => general.rust_api_type(),
            IrTypeDelegate::PrimitiveArray { primitive, .. } => primitive.rust_api_type(),
            _ => panic!("Unexpected type"),
        }
    }

    pub fn inner_is_js_value(&self) -> bool {
        match self {
            IrTypeDelegate::GeneralArray { general, .. } => general.is_js_value(),
            IrTypeDelegate::PrimitiveArray { .. } => false,
            _ => panic!("Unexpected type"),
        }
    }

    pub fn array_cast_string(&self) -> String {
        match self {
            IrTypeDelegate::GeneralArray { .. } => "raw".into(),
            IrTypeDelegate::PrimitiveArray { .. } => {
                format!("raw as {}", self.get_delegate().dart_api_type())
            }
            _ => panic!("Unexpected type"),
        }
    }

    pub fn dart_init_method(&self) -> String {
        match self {
            IrTypeDelegate::GeneralArray { length, .. } => format!(
                "{0}.init({1} fill): super(List<{1}>.filled({length},fill));",
                self.dart_api_type(),
                self.inner_dart_api_type()
            ),
            IrTypeDelegate::PrimitiveArray { length, .. } => format!(
                "{0}.init(): super({1}({length}));",
                self.dart_api_type(),
                self.get_delegate().dart_api_type()
            ),
            _ => panic!("Unexpected type"),
        }
    }
}

impl IrTypeTrait for IrTypeDelegate {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.get_delegate().visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegate::GeneralArray { general, length } => {
                format!("{}_array_{length}", general.dart_api_type())
            }
            IrTypeDelegate::PrimitiveArray { primitive, length } => {
                format!("{}_array_{length}", primitive.safe_ident())
            }
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
            IrTypeDelegate::GeneralArray { general, length } => {
                format!("{}Array{length}", general.dart_api_type())
            }
            IrTypeDelegate::PrimitiveArray { primitive, length } => {
                format!(
                    "{}Array{length}",
                    primitive.safe_ident().to_case(Case::Pascal)
                )
            }
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
            IrTypeDelegate::GeneralArray { length, .. }
            | IrTypeDelegate::PrimitiveArray { length, .. } => {
                format!("[{}; {length}]", self.inner_rust_api_type())
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
