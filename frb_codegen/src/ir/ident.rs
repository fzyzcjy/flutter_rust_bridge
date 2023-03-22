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

    // remove_raw_prefix removes the "r#" prefix from the identifier.
    pub fn remove_raw_prefix(&mut self) {
        self.raw = self.raw.strip_prefix("r#");
    }
}
