use crate::codegen::parser::mir::parser::lifetime_extractor::{
    Lifetime, LifetimeExtractor, LIFETIME_STATIC,
};
use regex::Regex;
use syn::Type;

pub(crate) fn replace_all_lifetimes_to_static(ty_str: &str) -> String {
    let ty: Type = syn::parse_str(ty_str).unwrap();
    let lifetimes = LifetimeExtractor::extract_skipping_static(&ty);
    replace_lifetimes_to_static(ty_str, &lifetimes)
}

pub(crate) fn replace_lifetimes_to_static(ty: &str, lifetimes: &[Lifetime]) -> String {
    let mut ans = ty.to_owned();
    for lifetime in lifetimes.iter() {
        ans = replace_lifetime(&ans, &lifetime.0, LIFETIME_STATIC);
    }
    ans
}

fn replace_lifetime(ty: &str, lifetime_src: &str, lifetime_dst: &str) -> String {
    let regex = Regex::new(&format!("'{}([^a-zA-Z]|$)", regex::escape(lifetime_src))).unwrap();
    regex
        .replace_all(ty, &format!("'{lifetime_dst}${{1}}"))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Preserves generic, argument, and trait-object separators during replacement.
    #[test]
    fn preserves_separators_when_replacing_lifetimes() {
        let lifetimes = vec![Lifetime("a".to_owned())];

        assert_eq!(
            replace_lifetimes_to_static("Result<Foo<'a>, Bar<'a>>", &lifetimes),
            "Result<Foo<'static>, Bar<'static>>"
        );
        assert_eq!(
            replace_lifetimes_to_static("Box<dyn Trait<'a> + Send>", &lifetimes),
            "Box<dyn Trait<'static> + Send>"
        );
    }

    /// Replaces extracted nested and reference lifetimes through the public API.
    #[test]
    fn replaces_all_extracted_lifetimes_to_static() {
        assert_eq!(
            replace_all_lifetimes_to_static("Result<&'a Item, Cow<'static, Wrapper<'b>>>"),
            "Result<&'static Item, Cow<'static, Wrapper<'static>>>"
        );
    }
}
