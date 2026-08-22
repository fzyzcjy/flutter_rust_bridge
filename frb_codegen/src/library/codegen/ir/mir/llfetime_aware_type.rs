use crate::codegen::parser::mir::parser::lifetime_replacer::replace_all_lifetimes_to_static;
crate::mir! {
pub struct MirLifetimeAwareType {
    raw: String,
}
}

impl MirLifetimeAwareType {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn with_original_lifetime(&self) -> &str {
        &self.raw
    }

    pub fn with_static_lifetime(&self) -> String {
        replace_all_lifetimes_to_static(&self.raw)
    }
}
