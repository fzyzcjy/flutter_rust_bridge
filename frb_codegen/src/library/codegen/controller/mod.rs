use crate::codegen::config::internal_config::ControllerInternalConfig;
use crate::utils::path_utils::path_to_string;
use itertools::Itertools;
use log::{info, warn};
use notify::{Event, FsEventWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub(super) fn run(
    config: &ControllerInternalConfig,
    run_inner: &impl Fn() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    if config.watch {
        run_watch(run_inner, &config.watching_paths, &config.exclude_paths)
    } else {
        run_inner()
    }
}

fn run_watch(
    run_inner: &impl Fn() -> anyhow::Result<()>,
    watching_paths: &[PathBuf],
    exclude_paths: &[PathBuf],
) -> anyhow::Result<()> {
    let (_watcher, fs_change_rx) = create_fs_watcher(watching_paths, exclude_paths.to_owned())?;

    loop {
        if let Err(e) = run_inner() {
            warn!("Error when running code generator: {e:?}");
        }

        info!(
            "Watching file changes on {}...",
            (watching_paths.iter())
                .map(|p| path_to_string(p).unwrap_or_default())
                .join(", ")
        );

        // If `recv` call ends, then we see at least one change
        fs_change_rx.recv()?;
        // Drain all other file changes
        while let Ok(_) = fs_change_rx.try_recv() {}
    }
}

fn create_fs_watcher(
    watching_paths: &[PathBuf],
    exclude_paths: Vec<PathBuf>,
) -> anyhow::Result<(Debouncer<RecommendedWatcher>, Receiver<()>)> {
    // ref: https://github.com/notify-rs/notify/blob/main/examples/monitor_raw.rs

    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(
        // Should not be too large, otherwise an event is only sent at the end of the interval
        Duration::from_millis(300),
        move |event: DebounceEventResult| {
            if is_event_interesting(&event, &exclude_paths) {
                info!("See interesting file change: {event:?}"); // TODO change to `debug` level
                tx.send(()).unwrap()
            }
        },
    )
    .unwrap();

    for path in watching_paths {
        debouncer.watcher().watch(path, RecursiveMode::Recursive)?;
    }

    Ok((debouncer, rx))
}

fn is_event_interesting(event: &DebounceEventResult, exclude_paths: &[PathBuf]) -> bool {
    if let Ok(event) = event {
        (event.iter())
            .map(|e| e.path.clone())
            .all(|p| !exclude_paths.contains(&p.canonicalize().unwrap_or(p.clone())))
    } else {
        false
    }
}
