use crate::codegen::generator::codec::sse::lang::Lang;
use crate::utils::cbindgen_keywords;
use convert_case::{Case, Casing};
crate::mir! {
#[serde(transparent)]
pub struct MirIdent {
    raw: String,
}
}

impl MirIdent {
    pub fn new(raw: String) -> MirIdent {
        MirIdent { raw }
    }

    pub fn rust_style(&self) -> &str {
        &self.raw
    }

    pub fn c_style(&self) -> String {
        convert_rust_to_c_style(&self.raw)
    }

    pub fn dart_style(&self) -> String {
        convert_rust_to_dart_style(&self.raw)
    }

    pub fn style(&self, lang: &Lang) -> String {
        match lang {
            Lang::DartLang(_) => self.dart_style(),
            Lang::RustLang(_) => self.rust_style().to_string(),
        }
    }
}

impl std::fmt::Display for MirIdent {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.raw)
    }
}

fn convert_rust_to_c_style(raw: &str) -> String {
    let mut ans = raw.to_owned();

    if let Some(stripped) = ans.strip_prefix("r#") {
        ans = stripped.to_owned();
    }

    // match behavior of ffigen
    if &ans == "async" {
        ans = "async1".to_owned();
    }

    // match behavior of cbindgen
    cbindgen_keywords::escape(&mut ans);

    ans
}

fn convert_rust_to_dart_style(raw: &str) -> String {
    (raw.strip_prefix("r#").unwrap_or(raw)).to_case(Case::Camel)
}
