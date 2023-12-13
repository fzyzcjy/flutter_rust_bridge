use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use lazy_static::lazy_static;
use std::fmt::Write;
use std::time::Duration;

lazy_static! {
    static ref MULTI_PROGRESS: MultiProgress = MultiProgress::new();
}

pub(crate) struct SimpleProgress {
    pb: ProgressBar,
}

impl Drop for SimpleProgress {
    fn drop(&mut self) {
        self.pb.finish();
        MULTI_PROGRESS.remove(&self.pb);
    }
}

pub(crate) fn simple_progress(message: String, level: usize) -> SimpleProgress {
    let style = ProgressStyle::with_template("{spinner} {level}{my_elapsed:.dim} {wide_msg}")
        .unwrap()
        .with_key("my_elapsed", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "[{:.1}s]", state.elapsed().as_secs_f64()).unwrap()
        })
        .with_key("level", move |state: &ProgressState, w: &mut dyn Write| {
            if level > 0 {
                write!(w, "└{} ", "──".repeat(level)).unwrap();
            }
        })
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let pb = MULTI_PROGRESS.add(ProgressBar::new_spinner());
    pb.set_style(style);
    pb.enable_steady_tick(Duration::from_millis(50));
    pb.set_message(message);
    SimpleProgress { pb }
}
