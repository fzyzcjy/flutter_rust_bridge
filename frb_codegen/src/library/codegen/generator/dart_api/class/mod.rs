use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::*;
use delegate::DelegateDartApiClassGenerator;
use enum_dispatch::enum_dispatch;

pub(super) mod delegate;
pub(super) mod enumeration;
pub(super) mod rust_opaque;
pub(super) mod structure;

#[enum_dispatch(DartApiClassGeneratorTrait)]
#[derive(Debug, Clone)]
pub enum DartApiClassGenerator<'a> {
    Delegate(DelegateDartApiClassGenerator<'a>),
    EnumRef(EnumDartApiClassGenerator<'a>),
    RustOpaque(RustOpaqueDartApiClassGenerator<'a>),
    StructRef(StructDartApiClassGenerator<'a>),
}

impl<'a> GeneratorDartApiClass<'a> {
    pub fn new(ty: IrType, ir_pack: &'a IrPack) -> Self {
        Some(match ty {
            Delegate(ir) => DelegateDartApiClassGenerator::new(ir).into(),
            EnumRef(ir) => EnumRefDartApiClassGenerator::new(ir).into(),
            RustOpaque(ir) => RustOpaqueDartApiClassGenerator::new(ir).into(),
            StructRef(ir) => StructRefDartApiClassGenerator::new(ir).into(),
            _ => return None,
        })
    }
}

#[enum_dispatch]
pub trait DartApiClassGeneratorTrait {
    fn generate(&self) -> String;
}

#[doc(hidden)]
#[macro_export]
macro_rules! dart_api_class_generator_struct {
    ($cls:ident, $ir_cls:ty) => {
        #[derive(Debug, Clone)]
        pub(super) struct $cls<'a> {
            ir: $ir_cls,
        }

        impl<'a> for $cls<'a> {
            pub(super) fn new(ir: $ir_cls) -> Self {
                Self { ir }
            }
        }
    };
}
