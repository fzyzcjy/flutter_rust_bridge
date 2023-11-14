use std::path::PathBuf;

pub(crate) fn set_cwd_test_fixture(fixture_name: &str) -> anyhow::Result<()> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_fixtures");
    d.push(fixture_name);
    log::debug!("set_cwd_test_fixture: {d:?}");
    Ok(std::env::set_current_dir(d)?)
}
