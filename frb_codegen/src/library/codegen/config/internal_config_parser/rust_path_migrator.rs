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
    fn test_previous_config_unsupported() {
        assert!(migrate_rust_input_config(&None, "native/src/hello/**/*.rs").is_err());
    }

    #[test]
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
}
