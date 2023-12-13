use crate::codegen::config::internal_config::ControllerInternalConfig;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

pub(super) fn run(
    config: &ControllerInternalConfig,
    run_inner: &impl Fn() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    if config.watch {
        run_watch(run_inner, &config.watching_paths)
    } else {
        run_inner()
    }
}

fn run_watch(
    run_inner: &impl Fn() -> anyhow::Result<()>,
    watching_paths: &[PathBuf],
) -> anyhow::Result<()> {
    // ref: https://github.com/notify-rs/notify/blob/main/examples/monitor_raw.rs
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    for path in watching_paths {
        watcher.watch(path, RecursiveMode::Recursive)?;
    }

    for res in rx {
        match res {
            Ok(event) => log::info!("Change: {event:?}"),
            Err(error) => log::error!("Error: {error:?}"),
        }
    }
}
