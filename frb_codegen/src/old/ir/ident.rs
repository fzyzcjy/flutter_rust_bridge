use convert_case::{Case, Casing};

impl serde::Serialize for IrIdent {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_newtype_struct("IrIdent", &self.raw)
    }
}
