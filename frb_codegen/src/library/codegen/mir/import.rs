crate::ir! {
pub struct IrDartImport {
    pub uri: String,
    pub alias: Option<String>,
}
}

impl IrDartImport {
    pub fn to_code(&self) -> String {
        let as_part = if let Some(alias) = &self.alias {
            format!("as {alias}")
        } else {
            "".to_owned()
        };
        format!("import '{}' {as_part};", self.uri)
    }
}
