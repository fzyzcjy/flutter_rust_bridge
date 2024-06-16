use crate::codegen::parser::mir::parser::lifetime_replacer::replace_all_lifetimes_to_static;
crate::mir! {
pub struct MirLifetimeAwareType {
    raw: String,
}
}

impl MirLifetimeAwareType {
    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn lifetime_replaced_static(&self) -> String {
        replace_all_lifetimes_to_static(self.raw())
    }
}
