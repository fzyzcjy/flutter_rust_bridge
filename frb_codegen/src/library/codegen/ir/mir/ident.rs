use crate::codegen::generator::codec::sse::lang::Lang;
use crate::utils::{cbindgen_keywords, dart_keywords};
use convert_case::{Case, Casing};

crate::mir! {
pub struct MirIdent {
    rust_style: String,
    dart_style: Option<String>,
}
}

impl MirIdent {
    pub fn new(rust_style: String, dart_style: Option<String>) -> MirIdent {
        MirIdent {
            rust_style,
            dart_style,
        }
    }

    pub fn rust_style(&self) -> String {
        self.rust_style.clone()
    }

    pub fn c_style(&self) -> String {
        convert_rust_to_c_style(&self.rust_style)
    }

    pub fn dart_style(&self) -> String {
        (self.dart_style.clone()).unwrap_or_else(|| convert_rust_to_dart_style(&self.rust_style))
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
        fmt.write_str(&self.rust_style)?;
        if let Some(dart_style) = &self.dart_style {
            write!(fmt, "(dart_style={})", dart_style)?;
        }
        Ok(())
    }
}

fn convert_rust_to_c_style(raw: &str) -> String {
    let mut ans = strip_prefix_rhash(raw).to_owned();

    // match behavior of ffigen
    if &ans == "async" {
        "async1".clone_into(&mut ans);
    }
    if &ans == "interface" {
        "interface1".clone_into(&mut ans);
    }

    // match behavior of cbindgen
    cbindgen_keywords::escape(&mut ans);

    ans
}

fn convert_rust_to_dart_style(raw: &str) -> String {
    let ans = strip_prefix_rhash(raw).to_case(Case::Camel);

    dart_keywords::escape(ans)
}

fn strip_prefix_rhash(raw: &str) -> &str {
    raw.strip_prefix("r#").unwrap_or(raw)
}
