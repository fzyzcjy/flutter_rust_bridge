use std::path::Path;

pub(crate) fn command_arg_maybe_fvm(pwd: &Path) -> Option<String> {
    if detect_fvm(pwd) {
        Some("fvm".to_owned())
    } else {
        None
    }
}

fn detect_fvm(pwd: &Path) -> bool {
    TODO
}
