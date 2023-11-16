use crate::codegen::generator::dart_api::class::DartApiClassGenerator::*;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
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
            Delegate(ir) => TODO.into(),
            TODO => TODO,
            _ => return None,
        })
    }
}

#[enum_dispatch]
pub trait DartApiClassGeneratorTrait {
    fn generate(&self);
}
