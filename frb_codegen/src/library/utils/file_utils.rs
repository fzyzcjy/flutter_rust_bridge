use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn temp_change_file(
    path: PathBuf,
    modifier: impl FnOnce(&str) -> String,
) -> anyhow::Result<TempChangeFile> {
    let content_original = fs::read_to_string(&path)?;
    let ans = TempChangeFile {
        path,
        content_original,
    };
    fs::write(&path, modifier(&content_original))?;
    Ok(ans)
}

pub(crate) struct TempChangeFile {
    path: PathBuf,
    content_original: String,
}

impl Drop for TempChangeFile {
    fn drop(&mut self) {
        fs::write(&self.path, &self.content_original).unwrap();
    }
}
