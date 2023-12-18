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
