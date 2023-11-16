use log::debug;
use serde_json::Value;
use std::fs;
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

#[cfg(test)]
pub(crate) fn json_comparison_test(actual: &Value, matcher_path: &str) -> anyhow::Result<()> {
    debug!(
        "json_comparison_test actual:\n{}",
        serde_json::to_string_pretty(actual)?
    );

    let expect: Value = serde_json::from_str(&fs::read_to_string(matcher_path)?)?;

    assert_eq!(actual, expect);

    Ok(())
}
