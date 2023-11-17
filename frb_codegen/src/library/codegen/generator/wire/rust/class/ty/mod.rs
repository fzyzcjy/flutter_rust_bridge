use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustClassGeneratorClassTrait {
    fn generate_class(&self) -> Option<String> {
        None
    }
}
