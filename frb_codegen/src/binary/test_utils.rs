use std::path::PathBuf;

// duplicated from `test_utils.rs`, since we do not want to expose it from the lib
pub(crate) fn set_cwd_test_fixture(fixture_name: &str) -> anyhow::Result<()> {
    let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures")
        .join(fixture_name);
    Ok(std::env::set_current_dir(d)?)
}
