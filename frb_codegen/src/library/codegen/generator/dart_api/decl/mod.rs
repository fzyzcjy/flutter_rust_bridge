use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait DartApiGeneratorDeclTrait {
    fn dart_api_type(&self) -> String;
}
