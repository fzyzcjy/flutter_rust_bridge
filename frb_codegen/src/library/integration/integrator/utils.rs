use anyhow::Result;
use include_dir::{Dir, DirEntry};
use std::fs;
use std::path::Path;

// ref: `Dir::extract`
pub(super) fn extract_dir_and_modify(
    d: &Dir,
    base_path: &Path,
    modifier: &impl Fn(&[u8]) -> Vec<u8>,
) -> Result<()> {
    for entry in d.entries() {
        let path = base_path.join(entry.path());
        match entry {
            DirEntry::Dir(d) => {
                fs::create_dir_all(&path)?;
                extract_dir_and_modify(d, base_path, modifier)?;
            }
            DirEntry::File(f) => {
                fs::write(path, modifier(f.contents()))?;
            }
        }
    }
    Ok(())
}
