crate::mir! {
pub struct MirDartImport {
    pub uri: String,
    pub alias: Option<String>,
}
}

impl MirDartImport {
    pub fn to_code(&self) -> String {
        let as_part = if let Some(alias) = &self.alias {
            format!("as {alias}")
        } else {
            "".to_owned()
        };
        format!("import '{}' {as_part};", self.uri)
    }
}
