use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
pub struct ApiIdent {
    pub raw: String,
}

impl std::fmt::Display for ApiIdent {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.raw)
    }
}

impl ApiIdent {
    pub fn new(raw: String) -> ApiIdent {
        ApiIdent { raw }
    }

    pub fn rust_style(&self) -> &str {
        &self.raw
    }

    pub fn dart_style(&self) -> String {
        self.raw.to_case(Case::Camel)
    }
}
