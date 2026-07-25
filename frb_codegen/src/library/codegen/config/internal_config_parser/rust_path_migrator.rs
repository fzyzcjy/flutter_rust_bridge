use anyhow::ensure;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(super) struct ConfigRustRootAndRustInput<'a> {
    pub rust_root: &'a str,
    pub rust_input: &'a str,
}

pub(super) fn migrate_rust_input_config<'a>(
    raw_rust_root: Option<&'a str>,
    raw_rust_input: &'a str,
) -> anyhow::Result<ConfigRustRootAndRustInput<'a>> {
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
        rust_root: raw_rust_root.unwrap_or("rust/"),
        rust_input: raw_rust_input,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_previous_config_auto_migrated() {
        let actual = migrate_rust_input_config(None, "rust/src/api/**/*.rs").unwrap();
        assert_eq!(
            actual,
            ConfigRustRootAndRustInput {
                rust_root: "rust/",
                rust_input: "crate::api"
            }
        );
    }

    #[test]
    fn test_previous_config_unsupported() {
        assert!(migrate_rust_input_config(None, "native/src/hello/**/*.rs").is_err());
    }

    #[test]
    fn test_current_config() {
        assert_eq!(
            migrate_rust_input_config(None, "crate::apple").unwrap(),
            ConfigRustRootAndRustInput {
                rust_root: "rust/".into(),
                rust_input: "crate::apple".into()
            }
        );

        assert_eq!(
            migrate_rust_input_config(Some("native/"), "crate::orange").unwrap(),
            ConfigRustRootAndRustInput {
                rust_root: "native/".into(),
                rust_input: "crate::orange".into()
            }
        );
    }
}
