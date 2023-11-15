#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub(crate) fn get_test_fixture_dir(fixture_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures")
        .join(fixture_name)
}

#[cfg(test)]
pub(crate) fn set_cwd_test_fixture(fixture_name: &str) -> anyhow::Result<()> {
    let d = get_test_fixture_dir(fixture_name);
    log::debug!("set_cwd_test_fixture: {d:?}");
    Ok(std::env::set_current_dir(d)?)
}
