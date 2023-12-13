use crate::codegen::config::internal_config::ControllerInternalConfig;
use log::{info, warn};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

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
    let fs_change_rx = create_fs_watcher(watching_paths)?;

    loop {
        if let Err(e) = run_inner() {
            warn!("Error when running code generator: {e:?}");
        }

        info!("Watching file changes...");
        // If `recv` call ends, then we see at least one change
        fs_change_rx.recv()?;
        // Drain all other file changes
        while let Ok(_) = fs_change_rx.try_recv()? {}
    }
}

fn create_fs_watcher(watching_paths: &[PathBuf]) -> anyhow::Result<Receiver<()>> {
    // ref: https://github.com/notify-rs/notify/blob/main/examples/monitor_raw.rs
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(|_| tx.send(()).unwrap(), notify::Config::default())?;
    for path in watching_paths {
        watcher.watch(path, RecursiveMode::Recursive)?;
    }
    Ok(rx)
}
