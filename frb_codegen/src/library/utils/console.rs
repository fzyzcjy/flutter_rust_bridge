use indicatif::{ProgressBar, ProgressStyle};

pub(crate) fn simple_progress() {
    let style =
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_msg}").unwrap();
    let pb = ProgressBar::new_spinner();
    pb.set_style(style);
    pb.finish_with_message("downloaded");
}
