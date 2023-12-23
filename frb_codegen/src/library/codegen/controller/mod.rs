use crate::codegen::config::internal_config::ControllerInternalConfig;
use crate::utils::path_utils::path_to_string;
use itertools::Itertools;
use log::{debug, warn};
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub(super) fn run(
    config: &ControllerInternalConfig,
    run_inner: &impl Fn() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    if config.watch {
        run_watch(
            run_inner,
            &config.watching_paths,
            &config.exclude_paths,
            config.max_count.unwrap_or(10000000),
        )
    } else {
        run_inner()
    }
}

fn run_watch(
    run_inner: &impl Fn() -> anyhow::Result<()>,
    watching_paths: &[PathBuf],
    exclude_paths: &[PathBuf],
    max_count: usize,
) -> anyhow::Result<()> {
    let (_watcher, fs_change_rx) = create_fs_watcher(watching_paths, exclude_paths.to_owned())?;

    for _i in 0..max_count {
        if let Err(e) = run_inner() {
            // We do not care about the warning message
            // frb-coverage:ignore-start
            warn!("Error when running code generator: {e:?}");
            // frb-coverage:ignore-end
        }

        println!(
            "Watching file changes on {}...",
            (watching_paths.iter())
                .map(|p| path_to_string(p).unwrap_or_default())
                .join(", ")
        );

        // If `recv` call ends, then we see at least one change
        fs_change_rx.recv()?;
        // Drain all other file changes
        while fs_change_rx.try_recv().is_ok() {}
    }

    Ok(())
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
                debug!("See interesting file change: {event:?}");
                tx.send(()).unwrap()
                // This bracket is weirdly not covered
                // frb-coverage:ignore-start
            }
            // frb-coverage:ignore-end
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use std::sync::Mutex;
    use tempfile::tempdir;

    #[serial]
    #[test]
    fn test_run_with_watch() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        fs::create_dir_all(temp_dir.path().join("my_folder"))?;

        let run_inner_count = Mutex::new(0);

        run(
            &ControllerInternalConfig {
                watch: true,
                watching_paths: vec![temp_dir.path().join("my_folder")],
                exclude_paths: vec![],
                max_count: Some(2),
            },
            &|| {
                let mut run_inner_count = run_inner_count.lock().unwrap();
                *run_inner_count += 1;
                fs::write(
                    (temp_dir.path().join("my_folder")).join(format!("{}.txt", run_inner_count)),
                    "content",
                )?;
                Ok(())
            },
        )?;

        assert_eq!(*run_inner_count.lock().unwrap(), 2);

        Ok(())
    }
}
