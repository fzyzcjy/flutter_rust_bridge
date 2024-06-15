use crate::codegen::parser::mir::parser::lifetime_extractor::LIFETIME_STATIC;

pub(crate) fn replace_lifetime_to_static(ty: &str, lifetimes: &[String]) -> String {
    let mut ans = ty.to_owned();
    for lifetime in &lifetimes {
        ans = replace_lifetime(&ans, lifetime, LIFETIME_STATIC);
    }
    ans
}

fn replace_lifetime(ty: &str, lifetime_src: &str, lifetime_dst: &str) -> String {

}
