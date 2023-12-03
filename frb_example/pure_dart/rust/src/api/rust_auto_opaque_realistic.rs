use tempfile::NamedTempFile;

/// Wraps the famous temporary file crate
pub struct TempFileTwinNormal {
    // Use `NamedTempFile` instead of `tempfile::tempfile`, since the latter
    // rely on OS to cleanup file, but we want to demonstrate Rust's `drop`
    // logic in this demo
    file: NamedTempFile,
}

impl TempFileTwinNormal {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            file: NamedTempFile::new()?,
        })
    }

    pub fn path(&self) -> String {
        self.file.path().to_str().unwrap().into_string()
    }
}
