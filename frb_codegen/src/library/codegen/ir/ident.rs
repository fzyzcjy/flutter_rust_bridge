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

    pub fn c_style(&self) -> &str {
        // TODO correct?
        self.raw.strip_prefix("r#").unwrap_or(self.raw.as_str())
    }

    pub fn dart_style(&self) -> String {
        self.c_style().to_case(Case::Camel)
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
