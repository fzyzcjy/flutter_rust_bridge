use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;

use std::path::Path;

use crate::Opts;

use self::generate_template::*;

pub fn get_symbols_if_no_duplicates(configs: &[Opts]) -> Result<Vec<String>, anyhow::Error> {
    let mut opts = OptArray::new_without_resolve(configs);
    let irs = opts.collect_irs();
    opts.get_mut_irs().extend(irs);
    opts.get_symbols_if_no_duplicates()
}

pub mod generate_template;
mod parse_sig_from_lib;
pub fn mod_from_rust_path(code_path: &str, crate_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap()
        .with_extension("")
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('/', "::")
}

pub fn with_changed_file<F: FnOnce() -> anyhow::Result<()>>(
    path: &str,
    append_content: &str,
    f: F,
) -> anyhow::Result<()> {
    let content_original = fs::read_to_string(path)?;
    fs::write(path, content_original.clone() + append_content)?;

    f()?;

    Ok(fs::write(path, content_original)?)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BlockIndex(pub usize);

impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(0);
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[extend::ext]
impl std::path::Path {
    #[inline]
    fn file_name_str(&self) -> Option<&str> {
        self.file_name().and_then(OsStr::to_str)
    }
}
