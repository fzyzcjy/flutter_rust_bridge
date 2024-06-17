use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::mir::custom_ser_des::MirCustomSerDes;
use crate::codegen::ir::mir::ty::enumeration::{MirEnumIdent, MirTypeEnumRef};
use crate::codegen::ir::mir::ty::general_list::{mir_list, MirTypeGeneralList};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;
use crate::codegen::ir::mir::ty::record::MirTypeRecord;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
    MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
};
use crate::codegen::ir::mir::ty::rust_opaque::MirTypeRustOpaque;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::utils::namespace::{Namespace, NamespacedName};

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
    CastedPrimitive(MirTypeDelegateCastedPrimitive),
    RustAutoOpaqueExplicit(MirTypeDelegateRustAutoOpaqueExplicit),
    ProxyVariant(MirTypeDelegateProxyVariant),
    ProxyEnum(MirTypeDelegateProxyEnum),
    DynTrait(MirTypeDelegateDynTrait),
    Lifetimeable(MirTypeDelegateLifetimeable),
    CustomSerDes(MirTypeDelegateCustomSerDes),
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
    pub(crate) mir: MirTypeEnumRef,
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
    pub inner_ok: Box<MirType>,
    pub inner_err: Box<MirType>,
    pub codec: CodecMode,
}

#[derive(Copy, strum_macros::Display)]
pub enum MirTypeDelegateBigPrimitive {
    I128,
    U128,
}

pub struct MirTypeDelegateCastedPrimitive {
    pub inner: MirTypePrimitive,
}

pub struct MirTypeDelegateRustAutoOpaqueExplicit {
    pub inner: MirTypeRustOpaque,
    pub raw: MirRustAutoOpaqueRaw,
}

pub struct MirTypeDelegateProxyVariant {
    pub inner: Box<MirType>,
    pub upstream: Box<MirType>,
    pub upstream_method_name: String,
}

pub struct MirTypeDelegateProxyEnum {
    pub original: Box<MirType>,
    pub delegate_namespace: Namespace,
    pub variants: Vec<MirTypeDelegateProxyVariant>,
}

pub struct MirTypeDelegateDynTrait {
    pub trait_def_name: NamespacedName,
    // `None` if and only if dummy mode
    pub data: Option<MirTypeDelegateDynTraitData>,
}

pub struct MirTypeDelegateLifetimeable {
    pub api_type: MirTypeRustAutoOpaqueImplicit,
    pub delegate: MirTypeDelegateRustAutoOpaqueExplicit,
}

pub struct MirTypeDelegateDynTraitData {
    pub delegate_namespace: Namespace,
    pub variants: Vec<MirTypeDelegateDynTraitVariant>,
}

pub struct MirTypeDelegateDynTraitVariant {
    pub ty: MirType,
}

pub struct MirTypeDelegateCustomSerDes {
    pub info: MirCustomSerDes,
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
            Self::StreamSink(mir) => {
                mir.inner_ok.visit_types(f, mir_context);
                mir.inner_err.visit_types(f, mir_context);
            }
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
            MirTypeDelegate::PrimitiveEnum(mir) => mir.mir.safe_ident(),
            MirTypeDelegate::Time(mir) => format!("Chrono_{}", mir),
            // MirTypeDelegate::TimeList(mir) => format!("Chrono_{}List", mir),
            MirTypeDelegate::Uuid => "Uuid".to_owned(),
            // MirTypeDelegate::Uuids => "Uuids".to_owned(),
            MirTypeDelegate::Backtrace => "Backtrace".to_owned(),
            MirTypeDelegate::AnyhowException => "AnyhowException".to_owned(),
            MirTypeDelegate::Map(mir) => {
                format!("Map_{}_{}", mir.key.safe_ident(), mir.value.safe_ident())
            }
            MirTypeDelegate::Set(mir) => format!("Set_{}", mir.inner.safe_ident()),
            MirTypeDelegate::StreamSink(mir) => {
                format!("StreamSink_{}_{}", mir.inner_ok.safe_ident(), mir.codec)
            }
            MirTypeDelegate::BigPrimitive(mir) => mir.to_string(),
            MirTypeDelegate::CastedPrimitive(mir) => {
                format!("CastedPrimitive_{}", mir.inner.safe_ident())
            }
            MirTypeDelegate::RustAutoOpaqueExplicit(mir) => {
                format!("AutoExplicit_{}", mir.inner.safe_ident())
            }
            MirTypeDelegate::DynTrait(mir) => mir.safe_ident(),
            MirTypeDelegate::ProxyVariant(mir) => {
                format!(
                    "ProxyVariant_{}_{}",
                    mir.upstream.safe_ident(),
                    mir.upstream_method_name
                )
            }
            MirTypeDelegate::ProxyEnum(mir) => {
                format!("ProxyEnum_{}", mir.get_delegate().safe_ident())
            }
            MirTypeDelegate::Lifetimeable(mir) => {
                format!("Lifetimeable_{}", mir.api_type.safe_ident())
            }
            MirTypeDelegate::CustomSerDes(mir) => {
                format!("CustomSerializer_{}", mir.info.rust_api_type.safe_ident())
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
            MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { mir, .. }) => {
                mir.rust_api_type()
            }
            MirTypeDelegate::Time(mir) => match mir {
                MirTypeDelegateTime::Naive => "chrono::NaiveDateTime",
                MirTypeDelegateTime::Local => "chrono::DateTime::<chrono::Local>",
                MirTypeDelegateTime::Utc => "chrono::DateTime::<chrono::Utc>",
                MirTypeDelegateTime::Duration => "chrono::Duration",
            }
            .to_owned(),
            // MirTypeDelegate::TimeList(mir) => match mir {
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
            MirTypeDelegate::Map(mir) => format!(
                "std::collections::HashMap<{}, {}>",
                mir.key.rust_api_type(),
                mir.value.rust_api_type()
            ),
            MirTypeDelegate::Set(mir) => {
                format!("std::collections::HashSet<{}>", mir.inner.rust_api_type())
            }
            MirTypeDelegate::StreamSink(mir) => {
                format!(
                    "StreamSink<{},flutter_rust_bridge::for_generated::{codec}Codec>",
                    mir.inner_ok.rust_api_type(),
                    codec = mir.codec,
                )
            }
            MirTypeDelegate::BigPrimitive(mir) => match mir {
                MirTypeDelegateBigPrimitive::I128 => "i128".to_owned(),
                MirTypeDelegateBigPrimitive::U128 => "u128".to_owned(),
            },
            MirTypeDelegate::CastedPrimitive(mir) => mir.inner.rust_api_type(),
            MirTypeDelegate::RustAutoOpaqueExplicit(mir) => {
                format!(
                    "RustAutoOpaque{}<{}>",
                    mir.inner.codec,
                    mir.raw.string.with_static_lifetime()
                )
            }
            MirTypeDelegate::DynTrait(mir) => format!("dyn {}", mir.trait_def_name.name),
            MirTypeDelegate::ProxyVariant(mir) => mir.inner.rust_api_type(),
            MirTypeDelegate::ProxyEnum(mir) => mir.original.rust_api_type(),
            MirTypeDelegate::Lifetimeable(mir) => mir.api_type.rust_api_type(),
            MirTypeDelegate::CustomSerDes(mir) => mir.info.rust_api_type.rust_api_type(),
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
            MirTypeDelegate::PrimitiveEnum(inner) => inner.mir.self_namespace(),
            MirTypeDelegate::Array(inner) => Some(inner.namespace.clone()),
            MirTypeDelegate::ProxyEnum(inner) => inner.original.self_namespace(),
            MirTypeDelegate::ProxyVariant(inner) => inner.inner.self_namespace(),
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
                | MirTypeDelegate::CastedPrimitive(_)
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
            MirTypeDelegate::Map(mir) => {
                mir_list(MirType::Record(mir.element_delegate.clone()), true)
            }
            MirTypeDelegate::Set(mir) => mir_list(*mir.inner.to_owned(), true),
            MirTypeDelegate::StreamSink(_) => MirType::Delegate(MirTypeDelegate::String),
            MirTypeDelegate::BigPrimitive(_) => MirType::Delegate(MirTypeDelegate::String),
            MirTypeDelegate::CastedPrimitive(mir) => MirType::Primitive(mir.inner.clone()),
            MirTypeDelegate::RustAutoOpaqueExplicit(mir) => MirType::RustOpaque(mir.inner.clone()),
            MirTypeDelegate::DynTrait(mir) => mir.get_delegate(),
            MirTypeDelegate::ProxyVariant(mir) => *mir.inner.clone(),
            MirTypeDelegate::ProxyEnum(mir) => mir.get_delegate(),
            MirTypeDelegate::Lifetimeable(mir) => MirType::Delegate(
                MirTypeDelegate::RustAutoOpaqueExplicit(mir.delegate.clone()),
            ),
            MirTypeDelegate::CustomSerDes(mir) => *mir.info.inner_type.clone(),
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

impl MirTypeDelegateProxyEnum {
    pub(crate) fn get_delegate(&self) -> MirType {
        MirType::EnumRef(MirTypeEnumRef {
            ident: MirEnumIdent(NamespacedName::new(
                self.delegate_namespace.clone(),
                self.delegate_enum_name(),
            )),
            is_exception: false,
        })
    }

    pub(crate) fn delegate_enum_name(&self) -> String {
        Self::delegate_enum_name_raw(&self.original)
    }

    pub(crate) fn delegate_enum_name_raw(original_ty: &MirType) -> String {
        format!("{}ProxyEnum", original_ty.safe_ident())
    }
}

impl MirTypeDelegateDynTrait {
    pub fn get_delegate(&self) -> MirType {
        if let Some(data) = &self.data {
            MirType::EnumRef(MirTypeEnumRef {
                ident: MirEnumIdent(NamespacedName::new(
                    data.delegate_namespace.clone(),
                    self.delegate_enum_name(),
                )),
                is_exception: false,
            })
        } else {
            MirType::Primitive(MirTypePrimitive::Unit)
        }
    }

    pub(crate) fn delegate_enum_name(&self) -> String {
        format!("{}Implementor", self.trait_def_name.name)
    }

    pub(crate) fn safe_ident(&self) -> String {
        format!("DynTrait_{}", self.trait_def_name.name)
    }

    pub(crate) fn data(&self) -> &MirTypeDelegateDynTraitData {
        self.data.as_ref().unwrap()
    }
}
