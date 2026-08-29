use cargo_metadata::{Metadata, MetadataCommand};
use std::path::Path;

pub(crate) fn execute_cargo_metadata(manifest_path: &Path) -> anyhow::Result<Metadata> {
    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(manifest_path);
    Ok(cmd.exec()?)
}

#[cfg(test)]
mod tests {
    use super::execute_cargo_metadata;
    use anyhow::Result;
    use std::fs;
    use tempfile::tempdir;

    /// Reads package identity and manifest path from an isolated minimal Cargo package.
    #[test]
    fn test_execute_cargo_metadata_for_minimal_package() -> Result<()> {
        let temp_dir = tempdir()?;
        let package_dir = temp_dir.path();
        fs::create_dir(package_dir.join("src"))?;
        let manifest_path = package_dir.join("Cargo.toml");
        fs::write(
            &manifest_path,
            "[package]\nname = \"metadata_test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )?;
        fs::write(
            package_dir.join("src/lib.rs"),
            "pub fn value() -> u8 { 1 }\n",
        )?;

        let metadata = execute_cargo_metadata(&manifest_path)?;
        let package = metadata.root_package().unwrap();
        assert_eq!(package.name, "metadata_test");
        assert_eq!(package.manifest_path.as_std_path(), manifest_path);
        Ok(())
    }
}
