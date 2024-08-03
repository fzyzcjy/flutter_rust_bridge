use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use enum_dispatch::enum_dispatch;

pub(crate) mod delegate;
pub(crate) mod enumeration;
pub(crate) mod enumeration_complex;
pub(crate) mod enumeration_simple;
pub(crate) mod rust_opaque;
pub(crate) mod structure;
mod structure_freezed;
mod structure_non_freezed;
mod trait_def;

#[enum_dispatch]
pub(crate) trait ApiDartGeneratorClassTrait {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        None
    }

    /// The code will not be put in dart api files, but instead be in `frb_generated.dart`
    fn generate_extra_impl_code(&self) -> Option<String> {
        None
    }
}

// the following have empty implementation
impl<'a> ApiDartGeneratorClassTrait for BoxedApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for DartFnApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for DartOpaqueApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for DynamicApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for GeneralListApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for OptionalApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for PrimitiveApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for PrimitiveListApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for RecordApiDartGenerator<'a> {}
impl<'a> ApiDartGeneratorClassTrait for RustAutoOpaqueImplicitApiDartGenerator<'a> {}
