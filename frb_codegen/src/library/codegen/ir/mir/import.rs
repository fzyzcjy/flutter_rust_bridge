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

#[cfg(test)]
mod tests {
    use super::MirDartImport;

    /// Checks import code generation with and without an alias.
    #[test]
    fn renders_import_with_optional_alias() {
        assert_eq!(
            MirDartImport {
                uri: "a.dart".into(),
                alias: None
            }
            .to_code(),
            "import 'a.dart' ;"
        );
        assert_eq!(
            MirDartImport {
                uri: "a.dart".into(),
                alias: Some("a".into())
            }
            .to_code(),
            "import 'a.dart' as a;"
        );
    }
}
