/// initialize for catching log on disk afterwards;
/// after initialized, all info from unwrap, panic, info!(), debug!()...
/// woulde be recorded in file like：`./logs/2023-01-26.log`
///
/// # Examples
/// ```
/// init_logger(&"./logs/").expect("failed to initial log");
/// ```
pub fn init_logger(path: &str) -> Result<(), fern::InitError> {
    std::fs::create_dir_all(path).unwrap();
    let d = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}] {}",
            chrono::Local::now().format("%Y/%m/%d %H:%M:%S"),
            record.level(),
            message
        ))
    });

    #[cfg(debug_assertions)]
    d.level(log::LevelFilter::Debug)
        .chain(fern::DateBased::new(path, "%Y-%m-%d.log"))
        .chain(std::io::stdout())
        .apply()?;

    #[cfg(not(debug_assertions))]
    d.level(log::LevelFilter::Info)
        .chain(fern::DateBased::new(path, "%Y-%m-%d.log"))
        .apply()?;

    std::panic::set_hook(Box::new(|m| {
        log::error!("{}", m);
    }));

    Ok(())
}
