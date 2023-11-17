use crate::codegen::generator::dart_api::base::*;
use enum_dispatch::enum_dispatch;

pub(crate) mod delegate;
pub(crate) mod enumeration;
pub(crate) mod rust_opaque;
pub(crate) mod structure;

#[enum_dispatch]
pub(crate) trait DartApiGeneratorClassTrait {
    fn generate_class(&self) -> Option<String> {
        None
    }
}

// the following have empty implementation
impl<'a> DartApiGeneratorClassTrait for BoxedDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for DartOpaqueDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for DynamicDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for GeneralListDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for OptionalDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for OptionalListDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for PrimitiveDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for PrimitiveListDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for RecordDartApiGenerator<'a> {}
impl<'a> DartApiGeneratorClassTrait for UnencodableDartApiGenerator<'a> {}
