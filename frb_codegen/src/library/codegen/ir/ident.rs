use crate::codegen::generator::codec::sse::lang::Lang;
use convert_case::{Case, Casing};
crate::ir! {
#[serde(transparent)]
pub struct IrIdent {
    pub raw: String,
}
}

impl IrIdent {
    pub fn new(raw: String) -> IrIdent {
        IrIdent { raw }
    }

    pub fn rust_style(&self) -> &str {
        &self.raw
    }

    pub fn c_style(&self) -> String {
        convert_rust_to_c_style(self.raw)
    }

    pub fn dart_style(&self) -> String {
        (self.raw.strip_prefix("r#").unwrap_or(self.raw.as_str())).to_case(Case::Camel)
    }

    pub fn style(&self, lang: &Lang) -> String {
        match lang {
            Lang::DartLang(_) => self.dart_style(),
            Lang::RustLang(_) => self.rust_style().to_string(),
        }
    }
}

impl std::fmt::Display for IrIdent {
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
    if CBINDGEN_RESERVED_KEYWORDS.contains(&ans) {
        ans.push("_");
    }

    ans
}

// copied from https://github.com/mozilla/cbindgen/blob/ca78140c01518a655355f84da1f3872939123b66/src/bindgen/reserved.rs#L8
const CBINDGEN_RESERVED_KEYWORDS: &[&str] = &[
    "alignas",
    "alignof",
    "auto",
    "bool",
    "break",
    "case",
    "catch",
    "char",
    "char16_t",
    "char32_t",
    "char8_t",
    "class",
    "const",
    "const_cast",
    "consteval",
    "constexpr",
    "continue",
    "decltype",
    "default",
    "delete",
    "do",
    "double",
    "dynamic_cast",
    "else",
    "enum",
    "explicit",
    "export",
    "extern",
    "false",
    "float",
    "for",
    "friend",
    "goto",
    "if",
    "inline",
    "int",
    "long",
    "mutable",
    "namespace",
    "new",
    "noexcept",
    "nullptr",
    "operator",
    "private",
    "protected",
    "public",
    "register",
    "reinterpret_cast",
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "static_assert",
    "static_cast",
    "struct",
    "switch",
    "template",
    "this",
    "thread_local",
    "throw",
    "true",
    "try",
    "typedef",
    "typename",
    "union",
    "unsigned",
    "using",
    "virtual",
    "void",
    "volatile",
    "wchar_t",
    "while",
];
