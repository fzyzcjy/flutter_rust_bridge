use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::mir::namespace::Namespace;
use crate::codegen::mir::ty::enumeration::MirTypeEnumRef;
use crate::codegen::mir::ty::general_list::{ir_list, MirTypeGeneralList};
use crate::codegen::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::mir::ty::primitive_list::MirTypePrimitiveList;
use crate::codegen::mir::ty::record::MirTypeRecord;
use crate::codegen::mir::ty::rust_auto_opaque_implicit::MirRustAutoOpaqueRaw;
use crate::codegen::mir::ty::rust_opaque::MirTypeRustOpaque;
use crate::codegen::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
/// types that delegate to another type
pub enum MirTypeDelegate {
    Array(MirTypeDelegateArray),
    String,
    Char,
    // StringList,// TODO avoid this special case?
    // ZeroCopyBufferVecPrimitive(MirTypePrimitive),
    PrimitiveEnum(MirTypeDelegatePrimitiveEnum),
    Time(MirTypeDelegateTime),
    // TimeList(MirTypeDelegateTime),// TODO avoid this special case?
    Uuid,
    // Uuids,// TODO avoid this special case?
    Backtrace,
    AnyhowException,
    Map(MirTypeDelegateMap),
    Set(MirTypeDelegateSet),
    StreamSink(MirTypeDelegateStreamSink),
    BigPrimitive(MirTypeDelegateBigPrimitive),
    RustAutoOpaqueExplicit(MirTypeDelegateRustAutoOpaqueExplicit),
}

pub struct MirTypeDelegateArray {
    pub namespace: Namespace,
    pub length: usize,
    pub mode: MirTypeDelegateArrayMode,
}

pub enum MirTypeDelegateArrayMode {
    General(Box<MirType>),
    Primitive(MirTypePrimitive),
}

pub struct MirTypeDelegatePrimitiveEnum {
    pub(crate) ir: MirTypeEnumRef,
    /// Allows for `#[repr]`'s other than [i32]
    pub(crate) repr: MirTypePrimitive,
}

#[derive(Copy, strum_macros::Display)]
pub enum MirTypeDelegateTime {
    Local,
    Utc,
    Naive,
    Duration,
}

pub struct MirTypeDelegateMap {
    pub key: Box<MirType>,
    pub value: Box<MirType>,
    pub element_delegate: MirTypeRecord,
}

pub struct MirTypeDelegateSet {
    pub inner: Box<MirType>,
}

pub struct MirTypeDelegateStreamSink {
    pub inner: Box<MirType>,
    pub codec: CodecMode,
}

#[derive(Copy, strum_macros::Display)]
pub enum MirTypeDelegateBigPrimitive {
    I128,
    U128,
}

pub struct MirTypeDelegateRustAutoOpaqueExplicit {
    pub inner: MirTypeRustOpaque,
    pub raw: MirRustAutoOpaqueRaw,
}
}

impl MirTypeTrait for MirTypeDelegate {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.get_delegate().visit_types(f, mir_context);

        #[allow(clippy::single_match)]
        match self {
            Self::StreamSink(ir) => ir.inner.visit_types(f, mir_context),
            // ... others
            _ => {}
        }
    }

    fn safe_ident(&self) -> String {
        match self {
            MirTypeDelegate::Array(array) => array.safe_ident(),
            MirTypeDelegate::String => "String".to_owned(),
            MirTypeDelegate::Char => "Char".to_owned(),
            // MirTypeDelegate::StringList => "StringList".to_owned(),
            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     "ZeroCopyBuffer_".to_owned() + &self.get_delegate().safe_ident()
            // }
            MirTypeDelegate::PrimitiveEnum(ir) => ir.ir.safe_ident(),
            MirTypeDelegate::Time(ir) => format!("Chrono_{}", ir),
            // MirTypeDelegate::TimeList(ir) => format!("Chrono_{}List", ir),
            MirTypeDelegate::Uuid => "Uuid".to_owned(),
            // MirTypeDelegate::Uuids => "Uuids".to_owned(),
            MirTypeDelegate::Backtrace => "Backtrace".to_owned(),
            MirTypeDelegate::AnyhowException => "AnyhowException".to_owned(),
            MirTypeDelegate::Map(ir) => {
                format!("Map_{}_{}", ir.key.safe_ident(), ir.value.safe_ident())
            }
            MirTypeDelegate::Set(ir) => format!("Set_{}", ir.inner.safe_ident()),
            MirTypeDelegate::StreamSink(ir) => {
                format!("StreamSink_{}_{}", ir.inner.safe_ident(), ir.codec)
            }
            MirTypeDelegate::BigPrimitive(ir) => ir.to_string(),
            MirTypeDelegate::RustAutoOpaqueExplicit(ir) => {
                format!("AutoExplicit_{}", ir.inner.safe_ident())
            }
        }
    }

    fn rust_api_type(&self) -> String {
        match self {
            MirTypeDelegate::Array(array) => {
                format!("[{}; {}]", array.inner().rust_api_type(), array.length)
            }
            MirTypeDelegate::String => "String".to_owned(),
            MirTypeDelegate::Char => "char".to_owned(),
            // MirTypeDelegate::StringList => "Vec<String>".to_owned(),
            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     format!(
            //         "flutter_rust_bridge::ZeroCopyBuffer<{}>",
            //         self.get_delegate().rust_api_type()
            //     )
            // }
            MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { ir, .. }) => {
                ir.rust_api_type()
            }
            MirTypeDelegate::Time(ir) => match mir {
                MirTypeDelegateTime::Naive => "chrono::NaiveDateTime",
                MirTypeDelegateTime::Local => "chrono::DateTime::<chrono::Local>",
                MirTypeDelegateTime::Utc => "chrono::DateTime::<chrono::Utc>",
                MirTypeDelegateTime::Duration => "chrono::Duration",
            }
            .to_owned(),
            // MirTypeDelegate::TimeList(ir) => match mir {
            //     MirTypeDelegateTime::Naive => "Vec<chrono::NaiveDateTime>",
            //     MirTypeDelegateTime::Local => "Vec<chrono::DateTime::<chrono::Local>>",
            //     MirTypeDelegateTime::Utc => "Vec<chrono::DateTime::<chrono::Utc>>",
            //     MirTypeDelegateTime::Duration => "Vec<chrono::Duration>",
            // }
            // .to_owned(),
            MirTypeDelegate::Uuid => "uuid::Uuid".to_owned(),
            // MirTypeDelegate::Uuids => "Vec<uuid::Uuid>".to_owned(),
            MirTypeDelegate::Backtrace => "backtrace::Backtrace".to_owned(),
            MirTypeDelegate::AnyhowException => {
                "flutter_rust_bridge::for_generated::anyhow::Error".to_owned()
            }
            MirTypeDelegate::Map(ir) => format!(
                "std::collections::HashMap<{}, {}>",
                ir.key.rust_api_type(),
                ir.value.rust_api_type()
            ),
            MirTypeDelegate::Set(ir) => {
                format!("std::collections::HashSet<{}>", ir.inner.rust_api_type())
            }
            MirTypeDelegate::StreamSink(ir) => {
                format!(
                    "StreamSink<{},flutter_rust_bridge::for_generated::{codec}Codec>",
                    ir.inner.rust_api_type(),
                    codec = ir.codec,
                )
            }
            MirTypeDelegate::BigPrimitive(ir) => match mir {
                MirTypeDelegateBigPrimitive::I128 => "i128".to_owned(),
                MirTypeDelegateBigPrimitive::U128 => "u128".to_owned(),
            },
            MirTypeDelegate::RustAutoOpaqueExplicit(ir) => {
                format!("RustAutoOpaque{}<{}>", ir.inner.codec, ir.raw.string)
            }
        }
    }

    fn as_primitive(&self) -> Option<&MirTypePrimitive> {
        match self {
            MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { repr, .. }) => Some(repr),
            MirTypeDelegate::Time(_) => Some(&MirTypePrimitive::I64),
            _ => None,
        }
    }

    fn self_namespace(&self) -> Option<Namespace> {
        match self {
            MirTypeDelegate::PrimitiveEnum(inner) => inner.ir.self_namespace(),
            MirTypeDelegate::Array(inner) => Some(inner.namespace.clone()),
            _ => None,
        }
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        matches!(
            self,
            MirTypeDelegate::String
                | MirTypeDelegate::Char
                | MirTypeDelegate::PrimitiveEnum(_)
                | MirTypeDelegate::BigPrimitive(_)
                | MirTypeDelegate::RustAutoOpaqueExplicit(_)
        )
    }
}

impl MirTypeDelegate {
    pub fn get_delegate(&self) -> MirType {
        match self {
            MirTypeDelegate::Array(array) => array.get_delegate(),
            MirTypeDelegate::String => MirType::PrimitiveList(MirTypePrimitiveList {
                primitive: MirTypePrimitive::U8,
                strict_dart_type: true,
            }),
            MirTypeDelegate::Char => MirType::Delegate(MirTypeDelegate::String),
            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(primitive) => {
            //     MirType::PrimitiveList(MirTypePrimitiveList {
            //         primitive: primitive.clone(),
            //     })
            // }
            // MirTypeDelegate::StringList => MirType::Delegate(MirTypeDelegate::String),
            MirTypeDelegate::PrimitiveEnum(inner) => MirType::Primitive(inner.repr.clone()),
            MirTypeDelegate::Time(_) => MirType::Primitive(MirTypePrimitive::I64),
            // MirTypeDelegate::TimeList(_) => MirType::PrimitiveList(MirTypePrimitiveList {
            //     primitive: MirTypePrimitive::I64,
            // }),
            MirTypeDelegate::Uuid => MirType::PrimitiveList(MirTypePrimitiveList {
                primitive: MirTypePrimitive::U8,
                strict_dart_type: true,
            }),
            // MirTypeDelegate::Uuids => MirType::PrimitiveList(MirTypePrimitiveList {
            //     primitive: MirTypePrimitive::U8,
            // }),
            MirTypeDelegate::Backtrace => MirType::Delegate(MirTypeDelegate::String),
            MirTypeDelegate::AnyhowException => MirType::Delegate(MirTypeDelegate::String),
            MirTypeDelegate::Map(ir) => {
                mir_list(MirType::Record(ir.element_delegate.clone()), true)
            }
            MirTypeDelegate::Set(ir) => mir_list(*ir.inner.to_owned(), true),
            MirTypeDelegate::StreamSink(_) => MirType::Delegate(MirTypeDelegate::String),
            MirTypeDelegate::BigPrimitive(_) => MirType::Delegate(MirTypeDelegate::String),
            MirTypeDelegate::RustAutoOpaqueExplicit(ir) => MirType::RustOpaque(ir.inner.clone()),
        }
    }
}

impl MirTypeDelegateArray {
    pub fn get_delegate(&self) -> MirType {
        match &self.mode {
            MirTypeDelegateArrayMode::General(general) => {
                MirType::GeneralList(MirTypeGeneralList {
                    inner: general.clone(),
                })
            }
            MirTypeDelegateArrayMode::Primitive(primitive) => {
                MirType::PrimitiveList(MirTypePrimitiveList {
                    primitive: primitive.clone(),
                    strict_dart_type: true,
                })
            }
        }
    }

    pub fn inner(&self) -> MirType {
        match &self.mode {
            MirTypeDelegateArrayMode::General(general) => *general.clone(),
            MirTypeDelegateArrayMode::Primitive(primitive) => MirType::Primitive(primitive.clone()),
        }
    }

    pub fn safe_ident(&self) -> String {
        let length = &self.length;
        match &self.mode {
            MirTypeDelegateArrayMode::General(general) => {
                format!("{}_array_{length}", general.safe_ident())
            }
            MirTypeDelegateArrayMode::Primitive(primitive) => {
                format!("{}_array_{length}", primitive.safe_ident())
            }
        }
    }
}
