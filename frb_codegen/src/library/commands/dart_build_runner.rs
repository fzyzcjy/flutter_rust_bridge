pub fn dart_build_runner(dart_root: &str) -> Result {
    info!("Running build_runner at {}", dart_root);
    let repo = DartRepository::from_str(dart_root).unwrap();
    let out = command_run!(
        call_shell[Some(dart_root)],
        *repo.toolchain.as_run_command(),
        "run",
        "build_runner",
        "build",
        "--delete-conflicting-outputs",
        "--enable-experiment=class-modifiers",
    )?;
    if !out.status.success() {
        Err(anyhow::anyhow!(
            "Failed to run build_runner for {}: {}",
            dart_root,
            String::from_utf8_lossy(&out.stdout)
        ))?;
    }
    Ok(())
}
