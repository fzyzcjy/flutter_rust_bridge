use crate::codegen::generator::dart_api::base::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait DartApiGeneratorDeclTrait {
    fn dart_api_type(&self) -> String;
}
