pub fn format_dart(path: &[PathBuf], line_length: u32) -> Result {
    debug!(
        "execute format_dart path={:?} line_length={}",
        path, line_length
    );
    let res = command_run!(
        call_shell[None],
        "dart",
        "format",
        "--line-length",
        line_length.to_string(),
        *path
    )?;
    if !res.status.success() {
        Err(Error::Dartfmt(
            String::from_utf8_lossy(&res.stderr).to_string(),
        ))?;
    }
    Ok(())
}
