use anyhow::ensure;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(super) struct ConfigRustRootAndRustInput {
    pub rust_root: String,
    pub rust_input: String,
}

pub(super) fn migrate_rust_input_config(
    raw_rust_root: &Option<String>,
    raw_rust_input: &str,
) -> anyhow::Result<ConfigRustRootAndRustInput> {
    if raw_rust_input == "rust/src/api/**/*.rs" {
        return Ok(ConfigRustRootAndRustInput {
            rust_root: "rust/".into(),
            rust_input: "crate::api".into(),
        });
    }

    ensure!(
        !(raw_rust_input.contains('*') || raw_rust_input.contains('.')),
        "Please migrate configuration `rust_input` to the new syntax.\
        For example, rust_input=`rust/src/api/**/*.rs` is now rust_input=`crate::api` and rust_root=`rust/`",
    );

    Ok(ConfigRustRootAndRustInput {
        rust_root: raw_rust_root.clone().unwrap_or_else(|| "rust/".to_owned()),
        rust_input: raw_rust_input.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Migrates exactly the supported legacy glob configuration.
    fn test_previous_config_auto_migrated() {
        let actual = migrate_rust_input_config(&None, "rust/src/api/**/*.rs").unwrap();
        assert_eq!(
            actual,
            ConfigRustRootAndRustInput {
                rust_root: "rust/".into(),
                rust_input: "crate::api".into()
            }
        );
    }

    #[test]
    /// Rejects unsupported legacy glob configurations.
    fn test_previous_config_unsupported() {
        assert!(migrate_rust_input_config(&None, "native/src/hello/**/*.rs").is_err());
    }

    #[test]
    /// Keeps current syntax and an explicitly supplied root unchanged.
    fn test_current_config() {
        assert_eq!(
            migrate_rust_input_config(&None, "crate::apple").unwrap(),
            ConfigRustRootAndRustInput {
                rust_root: "rust/".into(),
                rust_input: "crate::apple".into()
            }
        );

        assert_eq!(
            migrate_rust_input_config(&Some("native/".to_owned()), "crate::orange").unwrap(),
            ConfigRustRootAndRustInput {
                rust_root: "native/".into(),
                rust_input: "crate::orange".into()
            }
        );
    }

    /// Rejects every unsupported wildcard and dot form in current syntax.
    #[test]
    fn rejects_wildcards_and_dots_except_the_exact_legacy_value() {
        for input in [
            "crate::*",
            "crate::api.*",
            "native/src/api/**/*.rs",
            "foo.bar",
        ] {
            let error = migrate_rust_input_config(&None, input).unwrap_err();
            assert!(error.to_string().contains("Please migrate configuration"));
        }
    }

    /// Uses the supplied root for modern input syntax.
    #[test]
    fn preserves_supplied_root_for_modern_input() -> anyhow::Result<()> {
        let result = migrate_rust_input_config(&Some("custom-root/".to_owned()), "crate::api")?;

        assert_eq!(result.rust_root, "custom-root/");
        assert_eq!(result.rust_input, "crate::api");
        Ok(())
    }
}
