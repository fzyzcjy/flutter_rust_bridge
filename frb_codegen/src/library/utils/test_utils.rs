use log::debug;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub(crate) fn get_test_fixture_dir(fixture_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures")
        .join(fixture_name)
}

pub(crate) fn set_cwd_test_fixture(fixture_name: &str) -> anyhow::Result<()> {
    let d = get_test_fixture_dir(fixture_name);
    debug!("set_cwd_test_fixture: {d:?}");
    Ok(std::env::set_current_dir(d)?)
}

/// "golden" means comparison tests
/// see, for example, https://api.flutter.dev/flutter/flutter_test/matchesGoldenFile.html
/// for more information
pub(crate) fn json_golden_test(actual: &Value, matcher_path: &Path) -> anyhow::Result<()> {
    let actual_str = serde_json::to_string_pretty(actual)?;
    debug!("json_golden_test actual:\n{actual_str}");

    let expect: Value = if matcher_path.exists() {
        serde_json::from_str(&fs::read_to_string(matcher_path)?)?
    } else {
        ().into()
    };

    if enable_update_golden() {
        if actual != &expect {
            debug!("write golden data");
            fs::write(matcher_path, actual_str)?;
        }
    } else {
        assert_eq!(actual, &expect);
    }

    Ok(())
}

fn enable_update_golden() -> bool {
    let env_var = env::var("UPDATE_GOLDEN")
        .ok()
        .or_else(|| env::var("UPDATE_GOLDENS").ok())
        .unwrap_or_else(|| "".to_string())
        .to_lowercase();
    env_var == "true" || env_var == "1"
}
