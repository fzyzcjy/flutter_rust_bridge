use crate::codegen::generator::dart_api::base::*;
use enum_dispatch::enum_dispatch;

pub(crate) mod delegate;
pub(crate) mod enumeration;
pub(crate) mod rust_opaque;
pub(crate) mod structure;

#[enum_dispatch]
pub(crate) trait DartApiClassGeneratorTrait {
    fn generate_class(&self) -> Option<String> {
        None
    }
}

// the following have empty implementation
impl<'a> DartApiClassGeneratorTrait for BoxedDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for DartOpaqueDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for DynamicDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for GeneralListDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for OptionalDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for OptionalListDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for PrimitiveDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for PrimitiveListDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for RecordDartApiGenerator<'a> {}
impl<'a> DartApiClassGeneratorTrait for UnencodableDartApiGenerator<'a> {}
