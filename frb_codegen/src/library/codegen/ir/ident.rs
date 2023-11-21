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

    pub fn dart_style(&self) -> String {
        self.raw
            .strip_prefix("r#")
            .unwrap_or(self.raw.as_str())
            .to_string()
            .to_case(Case::Camel)
    }
}

impl std::fmt::Display for IrIdent {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.raw)
    }
}
