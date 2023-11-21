use cargo_metadata::{Metadata, MetadataCommand};
use std::path::Path;

pub(crate) fn execute_cargo_metadata(manifest_path: &Path) -> anyhow::Result<Metadata> {
    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(manifest_path);
    Ok(cmd.exec()?)
}
