use crate::codegen::generator::misc::target::TargetOrCommonMap;
use anyhow::Context;
use std::path::{Path, PathBuf};

pub(super) struct DartOutputPathPack {
    pub(super) dart_decl_base_output_path: PathBuf,
    pub(super) dart_impl_output_path: TargetOrCommonMap<PathBuf>,
}

pub(super) fn compute_dart_output_path_pack(
    dart_output_dir: &Path,
) -> anyhow::Result<DartOutputPathPack> {
    Ok(DartOutputPathPack {
        dart_decl_base_output_path: dart_output_dir.to_owned(),
        dart_impl_output_path: compute_path_map(&dart_output_dir.join("frb_generated.dart"))
            .context("dart_output: is wrong: ")?,
    })
}

pub(super) fn compute_path_map(path_common: &Path) -> anyhow::Result<TargetOrCommonMap<PathBuf>> {
    let extension = path_common.extension()
        .context(format!(
            "Cannot use the path configuration\n {path_common:?}.\n\
            A path for input/output needs to include the file name (a glob, like *.rs, can be used)."
        ))?.to_str().context(format!("Cannot convert path to string for the path {path_common:?}"))?;

    Ok(TargetOrCommonMap {
        common: path_common.to_owned(),
        io: path_common.with_extension(format!("io.{extension}")),
        web: path_common.with_extension(format!("web.{extension}")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_compute_path_map() -> anyhow::Result<()> {
        let result = super::compute_path_map(&PathBuf::from("src/api/api.rs"))?;
        assert_eq!(result.common, PathBuf::from("src/api/api.rs"));
        assert_eq!(result.io, PathBuf::from("src/api/api.io.rs"));
        assert_eq!(result.web, PathBuf::from("src/api/api.web.rs"));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_path_map_with_glob() -> anyhow::Result<()> {
        let result = super::compute_path_map(&PathBuf::from("src/api/*.rs"))?;
        assert_eq!(result.common, PathBuf::from("src/api/*.rs"));
        assert_eq!(result.io, PathBuf::from("src/api/*.io.rs"));
        assert_eq!(result.web, PathBuf::from("src/api/*.web.rs"));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_path_map_faulty() -> anyhow::Result<()> {
        let result = super::compute_path_map(&PathBuf::from("src/api"));
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Cannot use the path configuration"));
        Ok(())
    }
}
