use crate::ir::*;
use crate::target::Target;

#[cfg(feature = "chrono")]
#[derive(Debug, Clone)]
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
}

impl IrTypeDelegate {
    pub fn get_delegate(&self) -> IrType {
        match self {
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
        }
    }
}

impl IrTypeTrait for IrTypeDelegate {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.get_delegate().visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::StringList => "StringList".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer_".to_owned() + &self.get_delegate().dart_api_type()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.safe_ident(),
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => format!("Chrono_{}", ir.safe_ident()),
        }
    }

    fn dart_api_type(&self) -> String {
        match self {
            IrTypeDelegate::String => "String".to_string(),
            IrTypeDelegate::StringList => "List<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => self.get_delegate().dart_api_type(),
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.dart_api_type(),
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeTime::Local | IrTypeTime::Utc | IrTypeTime::Naive => "DateTime".to_string(),
                IrTypeTime::Duration => "Duration".to_string(),
            },
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match (self, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Wasm) => "List<String>".into(),
            (IrTypeDelegate::StringList, _) => "ffi.Pointer<wire_StringList>".to_owned(),
            #[cfg(feature = "chrono")]
            (IrTypeDelegate::Time(_), _) => "int".to_owned(),
            _ => self.get_delegate().dart_wire_type(target),
        }
    }

    fn rust_api_type(&self) -> String {
        match self {
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
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        match (self, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Io) => "wire_StringList".to_owned(),
            (IrTypeDelegate::StringList, Target::Wasm) => "JsValue".into(),
            #[cfg(feature = "chrono")]
            (IrTypeDelegate::Time(_), Target::Wasm) => "JsValue".into(),
            _ => self.get_delegate().rust_wire_type(target),
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        self.get_delegate().rust_wire_is_pointer(target)
    }
}
