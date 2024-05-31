use crate::codegen::Config;
use anyhow::ensure;
use std::path::PathBuf;

pub(super) struct ConfigRustRootAndRustInput {
    rust_root: PathBuf,
    rust_input: String,
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
        !(raw_rust_input.contains("*") || raw_rust_input.contains(".")),
        "Please migrate configuration `rust_input` to the new syntax.\
        For example, rust_input=`rust/src/api/**/*.rs` is now rust_input=`crate::api` and rust_root=`rust/`",
    );

    ConfigRustRootAndRustInput {
        rust_root: TODO,
        rust_input: TODO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_previous_config_auto_migrated() -> anyhow::Result<()> {
        let actual = migrate_rust_input_config(None, "rust/src/api/**/*.rs")?;
        assert_eq!(
            actual,
            ConfigRustRootAndRustInput {
                rust_root: "rust/".into(),
                rust_input: "crate::api".into()
            }
        );
        Ok(())
    }

    #[test]
    fn test_previous_config_unsupported() -> anyhow::Result<()> {
        assert!(migrate_rust_input_config(None, "native/src/hello/**/*.rs").is_err());
    }

    #[test]
    fn test_current_config() -> anyhow::Result<()> {
        assert_eq!(
            migrate_rust_input_config(None, "crate::apple"),
            ConfigRustRootAndRustInput {
                rust_root: "rust/".into(),
                rust_input: "crate::apple".into()
            }
        );

        assert_eq!(
            migrate_rust_input_config(Some("native/"), "crate::orange"),
            ConfigRustRootAndRustInput {
                rust_root: "native/".into(),
                rust_input: "crate::orange".into()
            }
        );
    }
}
