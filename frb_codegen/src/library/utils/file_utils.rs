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

#[cfg(test)]
mod tests {
    use crate::utils::file_utils::temp_change_file;
    use std::fs;

    #[test]
    fn test_temp_change_file_when_file_already_exists() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let file = dir.path().join("hello.txt");

        fs::write(&file.path(), "a")?;

        assert_eq!(fs::read_to_string(&file.path())?, "a");

        let change = temp_change_file(file.path().to_owned(), |text| text + "b")?;

        assert_eq!(fs::read_to_string(&file.path())?, "ab");

        drop(change);

        assert_eq!(fs::read_to_string(&file.path())?, "a");

        drop(dir);
        Ok(())
    }

    #[test]
    fn test_temp_change_file_when_file_not_exist() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let file = dir.path().join("hello.txt");

        assert_eq!(file.exists(), false);

        let change = temp_change_file(file.path().to_owned(), |text| text + "b")?;

        assert_eq!(fs::read_to_string(&file.path())?, "b");

        drop(change);

        assert_eq!(file.exists(), false);

        drop(dir);
        Ok(())
    }
}
