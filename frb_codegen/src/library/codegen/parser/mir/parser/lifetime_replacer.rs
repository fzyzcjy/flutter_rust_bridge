use crate::codegen::parser::mir::parser::lifetime_extractor::{LIFETIME_STATIC, LifetimeExtractor};
use regex::Regex;
use syn::Type;

pub(crate) fn replace_all_lifetimes_to_static(ty_str: &str) -> String {
    let ty: Type = syn::parse_str(ty_str).unwrap();
    let lifetimes = LifetimeExtractor::extract_skipping_static(&ty);
    replace_lifetimes_to_static(ty_str, &lifetimes)
}

pub(crate) fn replace_lifetimes_to_static(ty: &str, lifetimes: &[String]) -> String {
    let mut ans = ty.to_owned();
    for lifetime in lifetimes.iter() {
        ans = replace_lifetime(&ans, lifetime, LIFETIME_STATIC);
    }
    ans
}

fn replace_lifetime(ty: &str, lifetime_src: &str, lifetime_dst: &str) -> String {
    let regex = Regex::new(&format!("'{lifetime_src}[^a-zA-Z]")).unwrap();
    regex.replace(ty, &format!("'{lifetime_dst}")).to_string()
}
