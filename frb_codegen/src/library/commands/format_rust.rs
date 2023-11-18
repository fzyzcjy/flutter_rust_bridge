pub fn format_rust(path: &[PathBuf]) -> Result {
    debug!("execute format_rust path={:?}", path);
    let res = execute_command("rustfmt", path, None)?;
    if !res.status.success() {
        return Err(Error::Rustfmt(String::from_utf8_lossy(&res.stderr).to_string()).into());
    }
    Ok(())
}
