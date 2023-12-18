use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn temp_change_file(
    path: PathBuf,
    modifier: impl FnOnce(Option<String>) -> String,
) -> anyhow::Result<TempChangeFile> {
    let content_original = if path.exists() {
        Some(fs::read_to_string(&path)?)
    } else {
        None
    };

    let ans = TempChangeFile {
        path: path.clone(),
        content_original: content_original.clone(),
    };
    fs::write(&path, modifier(content_original))?;
    Ok(ans)
}

pub(crate) struct TempChangeFile {
    path: PathBuf,
    content_original: Option<String>,
}

impl Drop for TempChangeFile {
    fn drop(&mut self) {
        if let Some(content_original) = &self.content_original {
            fs::write(&self.path, content_original)
        } else {
            fs::remove_file(&self.path)
        }
        .unwrap()
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
        let path = dir.path().join("hello.txt");

        fs::write(&path, "a")?;

        assert_eq!(fs::read_to_string(&path)?, "a");

        let change = temp_change_file(path.to_owned(), |text| {
            assert_eq!(text, Some("a".to_owned()));
            text.unwrap_or_default() + "b"
        })?;

        assert_eq!(fs::read_to_string(&path)?, "ab");

        drop(change);

        assert_eq!(fs::read_to_string(&path)?, "a");

        drop(dir);
        Ok(())
    }

    #[test]
    fn test_temp_change_file_when_file_not_exist() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("hello.txt");

        assert!(!path.exists());

        let change = temp_change_file(path.to_owned(), |text| {
            assert_eq!(text, None);
            text.unwrap_or_default() + "b"
        })?;

        assert_eq!(fs::read_to_string(&path)?, "b");

        drop(change);

        assert!(!path.exists());

        drop(dir);
        Ok(())
    }
}
