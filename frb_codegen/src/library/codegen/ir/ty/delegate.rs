use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::enumeration::IrTypeEnumRef;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
/// types that delegate to another type
pub enum IrTypeDelegate {
    Array(IrTypeDelegateArray),
    String,
    StringList,
    ZeroCopyBufferVecPrimitive(IrTypePrimitive),
    PrimitiveEnum(IrTypeDelegatePrimitiveEnum),
    Time(IrTypeDelegateTime),
    TimeList(IrTypeDelegateTime),// TODO avoid this special case?
    Uuid,
    Uuids,// TODO avoid this special case?
    Backtrace,
    Anyhow,
}

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
}

impl IrTypeTrait for IrTypeDelegate {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        self.get_delegate().visit_types(f, ir_pack);

        // TODO avoid this hack
        // extras
        if let Self::TimeList(ir) = self {
            IrType::Delegate(IrTypeDelegate::Time(*ir)).visit_types(f, ir_pack);
        }
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => array.safe_ident(),
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::StringList => "StringList".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer_".to_owned() + &self.get_delegate().safe_ident()
            }
            IrTypeDelegate::PrimitiveEnum(ir) => ir.ir.safe_ident(),
            IrTypeDelegate::Time(ir) => format!("Chrono_{}", ir.to_string()),
            IrTypeDelegate::TimeList(ir) => format!("Chrono_{}List", ir.to_string()),
            IrTypeDelegate::Uuid => "Uuid".to_owned(),
            IrTypeDelegate::Uuids => "Uuids".to_owned(),
            IrTypeDelegate::Backtrace => "String".to_owned(),
            IrTypeDelegate::Anyhow => "FrbAnyhowException".to_owned(),
        }
    }

    fn rust_api_type(&self) -> String {
        match self {
            IrTypeDelegate::Array(array) => {
                format!("[{}; {}]", array.inner().rust_api_type(), array.length())
            }
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::StringList => "Vec<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                format!("ZeroCopyBuffer<{}>", self.get_delegate().rust_api_type())
            }
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

    fn as_primitive(&self) -> Option<&IrTypePrimitive> {
        match self {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { repr, .. }) => Some(repr),
            IrTypeDelegate::Time(_) => Some(&IrTypePrimitive::I64),
            _ => None,
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
            IrTypeDelegate::PrimitiveEnum(inner) => IrType::Primitive(inner.repr.clone()),
            IrTypeDelegate::Time(_) => IrType::Primitive(IrTypePrimitive::I64),
            IrTypeDelegate::TimeList(_) => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::I64,
            }),
            IrTypeDelegate::Uuid => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
            IrTypeDelegate::Uuids => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
            IrTypeDelegate::Backtrace => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::Anyhow => IrType::Delegate(IrTypeDelegate::String),
        }
    }
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

    pub fn inner(&self) -> IrType {
        match self {
            IrTypeDelegateArray::GeneralArray { general, .. } => *general.clone(),
            IrTypeDelegateArray::PrimitiveArray { primitive, .. } => {
                IrType::Primitive(primitive.clone())
            }
        }
    }

    pub fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, length } => {
                format!("{}_array_{length}", general.safe_ident())
            }
            IrTypeDelegateArray::PrimitiveArray { primitive, length } => {
                format!("{}_array_{length}", primitive.safe_ident())
            }
        }
    }

    pub fn length(&self) -> usize {
        *match self {
            IrTypeDelegateArray::GeneralArray { length, .. } => length,
            IrTypeDelegateArray::PrimitiveArray { length, .. } => length,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! forward_delegate_primitive_enum {
    ($self:ident, $func:ident($($tokens:tt)*), $ret:expr) => {
        if let IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum {ir, ..}) = &$self.ir {
            crate::library::codegen::generator::wire::rust::base::WireRustGenerator::new(
                crate::library::codegen::ir::ty::IrType::EnumRef(ir.clone()), $self.context.clone())
            .$func($($tokens)*)
        } else {
            $ret
        }
    };
}
