use crate::codegen::config::internal_config::ControllerInternalConfig;
use crate::codegen::MetaConfig;
use std::path::Path;

pub(super) fn parse(
    meta_config: &MetaConfig,
    rust_crate_dir: &Path,
    rust_output_path: &Path,
) -> anyhow::Result<ControllerInternalConfig> {
    let watching_paths = vec![
        // The whole crate needs to be watched, because e.g. when a struct definition changes
        // in a non-input file, it may still cause the generated code to change.
        rust_crate_dir.join("src"),
    ];
    let exclude_paths = vec![rust_output_path.to_owned()];

    Ok(ControllerInternalConfig {
        watch: meta_config.watch,
        watching_paths,
        exclude_paths,
        max_count: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Builds exact watch and exclusion paths when watching is enabled.
    #[test]
    fn builds_paths_with_enabled_watch() -> anyhow::Result<()> {
        let config = MetaConfig { watch: true };
        let result = parse(
            &config,
            Path::new("native"),
            Path::new("native/src/frb_generated.rs"),
        )?;

        assert!(result.watch);
        assert_eq!(result.watching_paths, vec![PathBuf::from("native/src")]);
        assert_eq!(
            result.exclude_paths,
            vec![PathBuf::from("native/src/frb_generated.rs")]
        );
        assert_eq!(result.max_count, None);
        Ok(())
    }

    /// Preserves disabled watching without changing derived paths.
    #[test]
    fn preserves_disabled_watch() -> anyhow::Result<()> {
        let config = MetaConfig { watch: false };
        let result = parse(&config, Path::new("crate"), Path::new("crate/generated.rs"))?;

        assert!(!result.watch);
        assert_eq!(result.watching_paths, vec![PathBuf::from("crate/src")]);
        assert_eq!(
            result.exclude_paths,
            vec![PathBuf::from("crate/generated.rs")]
        );
        Ok(())
    }
}
