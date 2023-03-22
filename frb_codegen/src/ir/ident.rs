use convert_case::{Case, Casing};

crate::ir! {
#[no_serde]
pub struct IrIdent {
    pub raw: String,
}
}

#[cfg(feature = "serde")]
impl serde::Serialize for IrIdent {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_newtype_struct("IrIdent", &self.raw)
    }
}

impl std::fmt::Display for IrIdent {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.raw)
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
        self.raw.to_case(Case::Camel)
    }

    pub fn remove_raw_prefix(&mut self) {
        if self.raw.starts_with("r#") {
            self.raw = self.raw[2..].to_string();
        }
    }
}
