use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use lazy_static::lazy_static;
use std::fmt::Write;
use std::sync::Mutex;
use std::time::Duration;

lazy_static! {
    pub(super) static ref MULTI_PROGRESS: MultiProgress = MultiProgress::new();
}

pub(crate) struct SimpleProgressBar {
    message: String,
    level: usize,
    active_pb: Mutex<Option<ProgressBar>>,
}

impl SimpleProgressBar {
    pub fn new(message: &str, level: usize) -> Self {
        Self {
            message: message.to_owned(),
            level,
            active_pb: Mutex::new(None),
        }
    }

    pub(crate) fn start(&self) -> SimpleProgressBarHandle {
        let mut active_pb = self.active_pb.lock().unwrap();
        if active_pb.is_none() {
            *active_pb = Some(create_simple_progress_bar(self.message.clone(), self.level));
        }
        SimpleProgressBarHandle {
            pb: active_pb.as_ref().unwrap().to_owned(),
        }
    }
}

pub(crate) fn progress_bar_message(message: &str) -> String {
    progress_bar_message_with_level(message, log::max_level())
}

fn progress_bar_message_with_level(message: &str, max_level: log::LevelFilter) -> String {
    if max_level >= log::LevelFilter::Debug {
        message.to_owned()
    } else {
        format!("{message} (use --verbose for logs)")
    }
}

pub(crate) fn println_over_progress(line: impl AsRef<str>) {
    println_over_progress_inner(&MULTI_PROGRESS, line.as_ref(), |line| eprintln!("{line}"));
}

fn println_over_progress_inner(
    multi_progress: &MultiProgress,
    line: &str,
    fallback: impl FnOnce(&str),
) {
    if multi_progress.is_hidden() {
        fallback(line);
    } else {
        // CI draw targets are hidden; llvm-cov never observes a real TTY.
        // frb-coverage:ignore-start
        let _ = multi_progress.println(line);
        // frb-coverage:ignore-end
    }
}

pub(crate) struct SimpleProgressBarHandle {
    pb: ProgressBar,
}

impl Drop for SimpleProgressBarHandle {
    fn drop(&mut self) {
        self.pb.finish()
    }
}

fn create_simple_progress_bar(message: String, level: usize) -> ProgressBar {
    let style = ProgressStyle::with_template("{level:.dim}{my_elapsed:.dim} {msg} {spinner}")
        .unwrap()
        .with_key("my_elapsed", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "[{:.1}s]", state.elapsed().as_secs_f64()).unwrap()
        })
        .with_key("level", move |_state: &ProgressState, w: &mut dyn Write| {
            if level > 0 {
                write!(w, "  └{} ", "──".repeat(level)).unwrap();
            }
        })
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let pb = MULTI_PROGRESS.add(ProgressBar::new_spinner());
    pb.set_style(style);
    pb.enable_steady_tick(Duration::from_millis(50));
    pb.set_message(message);
    pb
}

#[cfg(test)]
mod tests {
    use super::*;
    use indicatif::ProgressDrawTarget;

    #[test]
    fn test_progress_bar_message_hints_verbose_unless_already_debug() {
        assert_eq!(
            progress_bar_message_with_level("Run Dart build_runner", log::LevelFilter::Info),
            "Run Dart build_runner (use --verbose for logs)"
        );
        assert_eq!(
            progress_bar_message_with_level("Run Dart build_runner", log::LevelFilter::Debug),
            "Run Dart build_runner"
        );
    }

    #[test]
    /// Hidden progress targets must still forward diagnostics to the fallback writer.
    fn test_println_over_progress_hidden_target_uses_fallback() {
        let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::hidden());
        let mut actual = None;

        println_over_progress_inner(&multi_progress, "diagnostic", |line| {
            actual = Some(line.to_owned());
        });

        assert_eq!(actual.as_deref(), Some("diagnostic"));
    }
}
