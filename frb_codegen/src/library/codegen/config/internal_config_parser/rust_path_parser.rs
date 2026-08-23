use crate::codegen::config::internal_config_parser::rust_path_migrator::ConfigRustRootAndRustInput;
use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;
use crate::utils::path_utils::canonicalize_with_error_message;
use anyhow::ensure;
use itertools::Itertools;
use std::path::{Path, PathBuf};

pub(super) struct RustInputInfo {
    pub rust_crate_dir: PathBuf,
    pub third_party_crate_names: Vec<CrateName>,
    pub rust_input_namespace_pack: RustInputNamespacePack,
    pub rust_output_path: PathBuf,
}

pub(super) fn compute_rust_path_info(
    migrated_rust_input: &ConfigRustRootAndRustInput,
    base_dir: &Path,
    config_rust_output: &Option<String>,
) -> anyhow::Result<RustInputInfo> {
    let rust_input_namespace_prefixes_raw =
        compute_rust_input_namespace_prefixes_raw(&migrated_rust_input.rust_input);
    sanity_check_rust_input_namespace_prefixes(&rust_input_namespace_prefixes_raw);
    let rust_crate_dir = compute_rust_crate_dir(base_dir, &migrated_rust_input.rust_root)?;
    let rust_output_path = compute_rust_output_path(config_rust_output, base_dir, &rust_crate_dir)?;

    let rust_output_path_namespace =
        Namespace::new_from_rust_crate_path(&rust_output_path, &rust_crate_dir)?;

    Ok(RustInputInfo {
        rust_crate_dir,
        third_party_crate_names: compute_third_party_crate_names(
            &rust_input_namespace_prefixes_raw,
        ),
        rust_input_namespace_pack: RustInputNamespacePack {
            rust_input_namespace_prefixes: tidy_rust_input_namespace_prefixes(
                &rust_input_namespace_prefixes_raw,
            ),
            rust_output_path_namespace,
        },
        rust_output_path,
    })
}

fn sanity_check_rust_input_namespace_prefixes(rust_input_namespace_prefixes_raw: &[Namespace]) {
    if !(rust_input_namespace_prefixes_raw.iter()).any(|x| x.joined_path.contains("crate")) {
        // We do not care about codecov for this, since it is just a sanity check warning
        // frb-coverage:ignore-start
        log::warn!(
            "Reminder: `rust_input` field usually looks like `crate::api`, but no `crate` word is detected. \
            This is not a problem if the first-party crate is really not scanned.");
        // frb-coverage:ignore-end
    }
}

fn compute_rust_input_namespace_prefixes_raw(raw_rust_input: &str) -> Vec<Namespace> {
    raw_rust_input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| Namespace::new_raw(s.to_owned()))
        .collect()
}

fn tidy_rust_input_namespace_prefixes(raw: &[Namespace]) -> Vec<Namespace> {
    raw.iter()
        .map(|x| Namespace::new_raw(x.joined_path.replace('-', "_")))
        .collect_vec()
}

fn compute_rust_crate_dir(base_dir: &Path, rust_root: &str) -> anyhow::Result<PathBuf> {
    canonicalize_with_error_message(&base_dir.join(rust_root))
}

fn compute_rust_output_path(
    config_rust_output: &Option<String>,
    base_dir: &Path,
    rust_crate_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let ans = base_dir.join(
        (config_rust_output.clone().map(PathBuf::from))
            .unwrap_or_else(|| fallback_rust_output_path(rust_crate_dir)),
    );

    // We do not care about codecov for this, since it is just a sanity check warning
    // frb-coverage:ignore-start
    ensure!(
        ans.extension().is_some(),
        "Rust output path needs to include the file name."
    );
    // frb-coverage:ignore-end

    Ok(ans)
}

fn fallback_rust_output_path(rust_crate_dir: &Path) -> PathBuf {
    rust_crate_dir.join("src").join("frb_generated.rs")
}

fn compute_third_party_crate_names(
    rust_input_namespace_prefixes_raw: &[Namespace],
) -> Vec<CrateName> {
    rust_input_namespace_prefixes_raw
        .iter()
        .map(|x| x.path()[0])
        .filter(|x| *x != CrateName::SELF_CRATE)
        .sorted()
        .dedup()
        .map(|x| CrateName::new(x.to_owned()))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    /// Splits comma-delimited namespaces while trimming whitespace and blanks.
    fn test_compute_rust_input_namespace_prefixes_raw() {
        assert_eq!(
            compute_rust_input_namespace_prefixes_raw("module_b,module_a,"),
            vec![
                Namespace::new_raw("module_b".to_string()),
                Namespace::new_raw("module_a".to_string())
            ]
        );

        assert_eq!(
            compute_rust_input_namespace_prefixes_raw("module_a,"),
            vec![Namespace::new_raw("module_a".to_string())]
        );

        assert_eq!(
            compute_rust_input_namespace_prefixes_raw("module_a,module_b"),
            vec![
                Namespace::new_raw("module_a".to_string()),
                Namespace::new_raw("module_b".to_string())
            ]
        );

        assert_eq!(
            compute_rust_input_namespace_prefixes_raw("module_a, module_b"),
            vec![
                Namespace::new_raw("module_a".to_string()),
                Namespace::new_raw("module_b".to_string())
            ]
        );

        assert_eq!(
            compute_rust_input_namespace_prefixes_raw("module_a , module_b"),
            vec![
                Namespace::new_raw("module_a".to_string()),
                Namespace::new_raw("module_b".to_string())
            ]
        );

        assert_eq!(
            compute_rust_input_namespace_prefixes_raw("module_a ,module_b"),
            vec![
                Namespace::new_raw("module_a".to_string()),
                Namespace::new_raw("module_b".to_string())
            ]
        );

        assert_eq!(
            compute_rust_input_namespace_prefixes_raw("module_a , module_b, "),
            vec![
                Namespace::new_raw("module_a".to_string()),
                Namespace::new_raw("module_b".to_string())
            ]
        );
    }

    /// Drops entirely blank namespace entries.
    #[test]
    fn drops_blank_namespace_entries() {
        assert!(compute_rust_input_namespace_prefixes_raw(" , \t,\n ").is_empty());
    }

    /// Normalizes hyphens only in namespaces used by Rust parsing.
    #[test]
    fn normalizes_hyphens_in_tidy_namespaces() {
        let raw = compute_rust_input_namespace_prefixes_raw("crate::web-audio,third-party::api");

        assert_eq!(
            tidy_rust_input_namespace_prefixes(&raw),
            vec![
                Namespace::new_raw("crate::web_audio".to_owned()),
                Namespace::new_raw("third_party::api".to_owned()),
            ]
        );
    }

    /// Sorts and deduplicates third-party crates while excluding the current crate.
    #[test]
    fn extracts_sorted_unique_third_party_crates() {
        let raw = compute_rust_input_namespace_prefixes_raw(
            "zebra::api,crate::api,alpha::api,zebra::other,crate",
        );

        assert_eq!(
            compute_third_party_crate_names(&raw),
            vec![
                CrateName::new("alpha".to_owned()),
                CrateName::new("zebra".to_owned())
            ]
        );
    }

    /// Uses the canonical generated Rust output location by default.
    #[test]
    fn builds_fallback_rust_output_path() {
        assert_eq!(
            fallback_rust_output_path(Path::new("native")),
            PathBuf::from("native/src/frb_generated.rs")
        );
    }

    /// Resolves configured output paths relative to the base directory.
    #[test]
    fn resolves_configured_rust_output_path() -> anyhow::Result<()> {
        let result = compute_rust_output_path(
            &Some("generated/bridge.rs".to_owned()),
            Path::new("project"),
            Path::new("project/native"),
        )?;

        assert_eq!(result, PathBuf::from("project/generated/bridge.rs"));
        Ok(())
    }

    /// Rejects configured Rust output paths that omit a file name.
    #[test]
    fn rejects_rust_output_path_without_extension() {
        let error = compute_rust_output_path(
            &Some("generated/output".to_owned()),
            Path::new("project"),
            Path::new("project/native"),
        )
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("Rust output path needs to include the file name."));
    }

    /// Computes all Rust input details from a real temporary crate tree.
    #[test]
    fn computes_rust_path_info_from_temp_tree() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path().join("native");
        fs::create_dir_all(crate_dir.join("src"))?;
        let config = ConfigRustRootAndRustInput {
            rust_root: "native".to_owned(),
            rust_input: "crate::api,external-crate::model".to_owned(),
        };

        let result = compute_rust_path_info(&config, temp_dir.path(), &None)?;

        let canonical_crate_dir = crate_dir.canonicalize()?;
        assert_eq!(result.rust_crate_dir, canonical_crate_dir);
        assert_eq!(
            result.rust_output_path,
            result.rust_crate_dir.join("src/frb_generated.rs")
        );
        assert_eq!(
            result.third_party_crate_names,
            vec![CrateName::new("external-crate".to_owned())]
        );
        assert_eq!(
            result
                .rust_input_namespace_pack
                .rust_input_namespace_prefixes,
            vec![
                Namespace::new_raw("crate::api".to_owned()),
                Namespace::new_raw("external_crate::model".to_owned()),
            ]
        );
        assert_eq!(
            result.rust_input_namespace_pack.rust_output_path_namespace,
            Namespace::new_raw("crate::frb_generated".to_owned())
        );
        Ok(())
    }
}
