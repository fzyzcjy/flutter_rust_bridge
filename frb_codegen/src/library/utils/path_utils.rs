use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context};

pub fn glob_path(raw_path: &str, base_dir: &Path) -> Vec<PathBuf> {
    todo!()
}

pub fn path_to_string(path: &Path) -> anyhow::Result<String> {
    Ok(path.to_str().context("cannot convert path to str")?.to_owned())
}

pub fn find_parent_dir_with_file(path_start: &Path, probe_file_name: &str) -> anyhow::Result<PathBuf> {
    let mut path = path_start.to_owned();
    loop {
        if path.join(probe_file_name).is_file() { return Ok(path); }
        if !path.pop() { break; }
    }
    Err(anyhow!("Root of Dart library could not be inferred from Dart output"))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_glob_path_simple() {
        todo!()
    }
}
