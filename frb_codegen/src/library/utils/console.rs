use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::time::Duration;

pub(crate) struct SimpleProgress {
    pb: ProgressBar,
}

impl Drop for SimpleProgress {
    fn drop(&mut self) {
        self.pb.finish();
    }
}

pub(crate) fn simple_progress(message: &str) -> SimpleProgress {
    let style = ProgressStyle::with_template("{spinner} {my_elapsed:.dim} {wide_msg}")
        .unwrap()
        .with_key("my_elapsed", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "[{:.1}s]", state.elapsed().as_secs_f64()).unwrap()
        })
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let pb = ProgressBar::new_spinner();
    pb.set_style(style);
    pb.enable_steady_tick(Duration::from_millis(50));
    pb.set_message(message);
    SimpleProgress { pb }
}
