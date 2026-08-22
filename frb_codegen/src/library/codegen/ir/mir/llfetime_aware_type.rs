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

#[cfg(test)]
mod tests {
    use super::MirLifetimeAwareType;

    /// Preserves the original type and rewrites lifetimes to static.
    #[test]
    fn preserves_original_and_rewrites_lifetimes() {
        let ty = MirLifetimeAwareType::new("Foo<'a, 'static>".into());
        assert_eq!(ty.with_original_lifetime(), "Foo<'a, 'static>");
        assert_eq!(ty.with_static_lifetime(), "Foo<'static, 'static>");
    }
}
