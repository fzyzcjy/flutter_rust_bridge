use crate::codegen::generator::dart_api::base::*;
use enum_dispatch::enum_dispatch;

pub(super) mod delegate;
pub(super) mod enumeration;
pub(super) mod rust_opaque;
pub(super) mod structure;

#[enum_dispatch]
pub(super) trait DartApiClassGeneratorTrait {
    fn generate_class(&self) -> Option<String> {
        None
    }
}

impl<'a> DartApiClassGeneratorTrait for BoxedDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for DartOpaqueDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for DelegateDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for DynamicDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for EnumRefDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for GeneralListDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for OptionalDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for OptionalListDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for PrimitiveDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for PrimitiveListDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for RecordDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for RustOpaqueDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for StructRefDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for UnencodableDartApiGenerator<'a> {}
