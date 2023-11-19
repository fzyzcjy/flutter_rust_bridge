use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn temp_change_file(
    path: PathBuf,
    modifier: impl FnOnce(String) -> String,
) -> anyhow::Result<TempChangeFile> {
    let content_original = fs::read_to_string(&path)?;
    let ans = TempChangeFile {
        path: path.clone(),
        content_original: content_original.clone(),
    };
    fs::write(&path, modifier(content_original))?;
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

pub fn create_dir_all_and_write<P: AsRef<Path>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> anyhow::Result<()> {
    fn inner(path: &Path, contents: &[u8]) -> anyhow::Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        Ok(fs::write(path, contents)?)
    }
    inner(path.as_ref(), contents.as_ref())
}
