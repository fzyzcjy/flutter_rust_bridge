use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::mir::namespace::Namespace;
use crate::codegen::mir::ty::enumeration::IrTypeEnumRef;
use crate::codegen::mir::ty::general_list::{ir_list, IrTypeGeneralList};
use crate::codegen::mir::ty::primitive::IrTypePrimitive;
use crate::codegen::mir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::mir::ty::record::IrTypeRecord;
use crate::codegen::mir::ty::rust_auto_opaque_implicit::IrRustAutoOpaqueRaw;
use crate::codegen::mir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::mir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
/// types that delegate to another type
pub enum IrTypeDelegate {
    Array(IrTypeDelegateArray),
    String,
    Char,
    // StringList,// TODO avoid this special case?
    // ZeroCopyBufferVecPrimitive(IrTypePrimitive),
    PrimitiveEnum(IrTypeDelegatePrimitiveEnum),
    Time(IrTypeDelegateTime),
    // TimeList(IrTypeDelegateTime),// TODO avoid this special case?
    Uuid,
    // Uuids,// TODO avoid this special case?
    Backtrace,
    AnyhowException,
    Map(IrTypeDelegateMap),
    Set(IrTypeDelegateSet),
    StreamSink(IrTypeDelegateStreamSink),
    BigPrimitive(IrTypeDelegateBigPrimitive),
    RustAutoOpaqueExplicit(IrTypeDelegateRustAutoOpaqueExplicit),
}

pub struct IrTypeDelegateArray {
    pub namespace: Namespace,
    pub length: usize,
    pub mode: IrTypeDelegateArrayMode,
}

pub enum IrTypeDelegateArrayMode {
    General(Box<IrType>),
    Primitive(IrTypePrimitive),
}

pub struct IrTypeDelegatePrimitiveEnum {
    pub(crate) ir: IrTypeEnumRef,
    /// Allows for `#[repr]`'s other than [i32]
    pub(crate) repr: IrTypePrimitive,
}

#[derive(Copy, strum_macros::Display)]
pub enum IrTypeDelegateTime {
    Local,
    Utc,
    Naive,
    Duration,
}

pub struct IrTypeDelegateMap {
    pub key: Box<IrType>,
    pub value: Box<IrType>,
    pub element_delegate: IrTypeRecord,
}

pub struct IrTypeDelegateSet {
    pub inner: Box<IrType>,
}

pub struct IrTypeDelegateStreamSink {
    pub inner: Box<IrType>,
    pub codec: CodecMode,
}

#[derive(Copy, strum_macros::Display)]
pub enum IrTypeDelegateBigPrimitive {
    I128,
    U128,
}

pub struct IrTypeDelegateRustAutoOpaqueExplicit {
    pub inner: IrTypeRustOpaque,
    pub raw: IrRustAutoOpaqueRaw,
}
}

impl IrTypeTrait for IrTypeDelegate {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.get_delegate().visit_types(f, ir_context);

        #[allow(clippy::single_match)]
        match self {
            Self::StreamSink(ir) => ir.inner.visit_types(f, ir_context),
            // ... others
            _ => {}
        }
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => array.safe_ident(),
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::Char => "Char".to_owned(),
            // IrTypeDelegate::StringList => "StringList".to_owned(),
            // IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     "ZeroCopyBuffer_".to_owned() + &self.get_delegate().safe_ident()
            // }
            IrTypeDelegate::PrimitiveEnum(ir) => ir.ir.safe_ident(),
            IrTypeDelegate::Time(ir) => format!("Chrono_{}", ir),
            // IrTypeDelegate::TimeList(ir) => format!("Chrono_{}List", ir),
            IrTypeDelegate::Uuid => "Uuid".to_owned(),
            // IrTypeDelegate::Uuids => "Uuids".to_owned(),
            IrTypeDelegate::Backtrace => "Backtrace".to_owned(),
            IrTypeDelegate::AnyhowException => "AnyhowException".to_owned(),
            IrTypeDelegate::Map(ir) => {
                format!("Map_{}_{}", ir.key.safe_ident(), ir.value.safe_ident())
            }
            IrTypeDelegate::Set(ir) => format!("Set_{}", ir.inner.safe_ident()),
            IrTypeDelegate::StreamSink(ir) => {
                format!("StreamSink_{}_{}", ir.inner.safe_ident(), ir.codec)
            }
            IrTypeDelegate::BigPrimitive(ir) => ir.to_string(),
            IrTypeDelegate::RustAutoOpaqueExplicit(ir) => {
                format!("AutoExplicit_{}", ir.inner.safe_ident())
            }
        }
    }

    fn rust_api_type(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => {
                format!("[{}; {}]", array.inner().rust_api_type(), array.length)
            }
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::Char => "char".to_owned(),
            // IrTypeDelegate::StringList => "Vec<String>".to_owned(),
            // IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     format!(
            //         "flutter_rust_bridge::ZeroCopyBuffer<{}>",
            //         self.get_delegate().rust_api_type()
            //     )
            // }
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                ir.rust_api_type()
            }
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeDelegateTime::Naive => "chrono::NaiveDateTime",
                IrTypeDelegateTime::Local => "chrono::DateTime::<chrono::Local>",
                IrTypeDelegateTime::Utc => "chrono::DateTime::<chrono::Utc>",
                IrTypeDelegateTime::Duration => "chrono::Duration",
            }
            .to_owned(),
            // IrTypeDelegate::TimeList(ir) => match ir {
            //     IrTypeDelegateTime::Naive => "Vec<chrono::NaiveDateTime>",
            //     IrTypeDelegateTime::Local => "Vec<chrono::DateTime::<chrono::Local>>",
            //     IrTypeDelegateTime::Utc => "Vec<chrono::DateTime::<chrono::Utc>>",
            //     IrTypeDelegateTime::Duration => "Vec<chrono::Duration>",
            // }
            // .to_owned(),
            IrTypeDelegate::Uuid => "uuid::Uuid".to_owned(),
            // IrTypeDelegate::Uuids => "Vec<uuid::Uuid>".to_owned(),
            IrTypeDelegate::Backtrace => "backtrace::Backtrace".to_owned(),
            IrTypeDelegate::AnyhowException => {
                "flutter_rust_bridge::for_generated::anyhow::Error".to_owned()
            }
            IrTypeDelegate::Map(ir) => format!(
                "std::collections::HashMap<{}, {}>",
                ir.key.rust_api_type(),
                ir.value.rust_api_type()
            ),
            IrTypeDelegate::Set(ir) => {
                format!("std::collections::HashSet<{}>", ir.inner.rust_api_type())
            }
            IrTypeDelegate::StreamSink(ir) => {
                format!(
                    "StreamSink<{},flutter_rust_bridge::for_generated::{codec}Codec>",
                    ir.inner.rust_api_type(),
                    codec = ir.codec,
                )
            }
            IrTypeDelegate::BigPrimitive(ir) => match ir {
                IrTypeDelegateBigPrimitive::I128 => "i128".to_owned(),
                IrTypeDelegateBigPrimitive::U128 => "u128".to_owned(),
            },
            IrTypeDelegate::RustAutoOpaqueExplicit(ir) => {
                format!("RustAutoOpaque{}<{}>", ir.inner.codec, ir.raw.string)
            }
        }
    }

    fn as_primitive(&self) -> Option<&IrTypePrimitive> {
        match self {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { repr, .. }) => Some(repr),
            IrTypeDelegate::Time(_) => Some(&IrTypePrimitive::I64),
            _ => None,
        }
    }

    fn self_namespace(&self) -> Option<Namespace> {
        match self {
            IrTypeDelegate::PrimitiveEnum(inner) => inner.ir.self_namespace(),
            IrTypeDelegate::Array(inner) => Some(inner.namespace.clone()),
            _ => None,
        }
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        matches!(
            self,
            IrTypeDelegate::String
                | IrTypeDelegate::Char
                | IrTypeDelegate::PrimitiveEnum(_)
                | IrTypeDelegate::BigPrimitive(_)
                | IrTypeDelegate::RustAutoOpaqueExplicit(_)
        )
    }
}

impl IrTypeDelegate {
    pub fn get_delegate(&self) -> IrType {
        match self {
            IrTypeDelegate::Array(array) => array.get_delegate(),
            IrTypeDelegate::String => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
                strict_dart_type: true,
            }),
            IrTypeDelegate::Char => IrType::Delegate(IrTypeDelegate::String),
            // IrTypeDelegate::ZeroCopyBufferVecPrimitive(primitive) => {
            //     IrType::PrimitiveList(IrTypePrimitiveList {
            //         primitive: primitive.clone(),
            //     })
            // }
            // IrTypeDelegate::StringList => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::PrimitiveEnum(inner) => IrType::Primitive(inner.repr.clone()),
            IrTypeDelegate::Time(_) => IrType::Primitive(IrTypePrimitive::I64),
            // IrTypeDelegate::TimeList(_) => IrType::PrimitiveList(IrTypePrimitiveList {
            //     primitive: IrTypePrimitive::I64,
            // }),
            IrTypeDelegate::Uuid => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
                strict_dart_type: true,
            }),
            // IrTypeDelegate::Uuids => IrType::PrimitiveList(IrTypePrimitiveList {
            //     primitive: IrTypePrimitive::U8,
            // }),
            IrTypeDelegate::Backtrace => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::AnyhowException => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::Map(ir) => ir_list(IrType::Record(ir.element_delegate.clone()), true),
            IrTypeDelegate::Set(ir) => ir_list(*ir.inner.to_owned(), true),
            IrTypeDelegate::StreamSink(_) => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::BigPrimitive(_) => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::RustAutoOpaqueExplicit(ir) => IrType::RustOpaque(ir.inner.clone()),
        }
    }
}

impl IrTypeDelegateArray {
    pub fn get_delegate(&self) -> IrType {
        match &self.mode {
            IrTypeDelegateArrayMode::General(general) => IrType::GeneralList(IrTypeGeneralList {
                inner: general.clone(),
            }),
            IrTypeDelegateArrayMode::Primitive(primitive) => {
                IrType::PrimitiveList(IrTypePrimitiveList {
                    primitive: primitive.clone(),
                    strict_dart_type: true,
                })
            }
        }
    }

    pub fn inner(&self) -> IrType {
        match &self.mode {
            IrTypeDelegateArrayMode::General(general) => *general.clone(),
            IrTypeDelegateArrayMode::Primitive(primitive) => IrType::Primitive(primitive.clone()),
        }
    }

    pub fn safe_ident(&self) -> String {
        let length = &self.length;
        match &self.mode {
            IrTypeDelegateArrayMode::General(general) => {
                format!("{}_array_{length}", general.safe_ident())
            }
            IrTypeDelegateArrayMode::Primitive(primitive) => {
                format!("{}_array_{length}", primitive.safe_ident())
            }
        }
    }
}
