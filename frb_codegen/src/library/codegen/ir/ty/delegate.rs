use crate::codegen::ir::pack::IrPack;
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
    TimeList(IrTypeDelegateTime),
    Uuid,
    Uuids,
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
    ir: IrTypeEnumRef,
    /// Allows for `#[repr]`'s other than [i32]
    repr: IrTypePrimitive,
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
                "ZeroCopyBuffer_".to_owned() + &self.get_delegate().dart_api_type()
            }
            IrTypeDelegate::PrimitiveEnum(ir) => ir.ir.safe_ident(),
            IrTypeDelegate::Time(ir) => format!("Chrono_{}", ir.safe_ident()),
            IrTypeDelegate::TimeList(ir) => format!("Chrono_{}List", ir.safe_ident()),
            IrTypeDelegate::Uuid => "Uuid".to_owned(),
            IrTypeDelegate::Uuids => "Uuids".to_owned(),
            IrTypeDelegate::Backtrace => "String".to_owned(),
            IrTypeDelegate::Anyhow => "FrbAnyhowException".to_owned(),
        }
    }
}
