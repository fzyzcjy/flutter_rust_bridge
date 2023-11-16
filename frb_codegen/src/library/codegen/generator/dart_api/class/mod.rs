use crate::codegen::generator::dart_api::class::enumeration::EnumRefDartApiClassGenerator;
use crate::codegen::generator::dart_api::class::rust_opaque::RustOpaqueDartApiClassGenerator;
use crate::codegen::generator::dart_api::class::structure::StructRefDartApiClassGenerator;
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
    EnumRef(EnumRefDartApiClassGenerator<'a>),
    RustOpaque(RustOpaqueDartApiClassGenerator<'a>),
    StructRef(StructRefDartApiClassGenerator<'a>),
}

#[enum_dispatch]
pub trait DartApiClassGeneratorTrait {
    fn generate(&self) -> String;
}
