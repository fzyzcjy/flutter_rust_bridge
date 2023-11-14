use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

/// Configure an opinionated way of logging.
///
/// This is just one way of outputing logs, and users are free to use this function
/// or choose their own way of outputing logs. That's why this function is "opinionated".
///
/// It will log to file and standard output.
/// All logs with level `debug`(with parameter `verbose`=true or system variable `RUST_LOG`="debug") or above
/// will be recorded in `./logs/<date>.log`.
/// Logs with level `info` and above will be output to standard output, with colored tag.
///
/// # Example
///
/// ```
/// use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_logging;
/// configure_opinionated_logging("./logs/", false).expect("failed to initialize log");
/// ```
pub fn configure_opinionated_logging(path: &str, verbose: bool) -> Result<(), fern::InitError> {
    let colored_output = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    let mut d = fern::Dispatch::new();
    d = d.format(move |out, message, record| {
        let time = chrono::Local::now().format("%Y/%m/%d %H:%M:%S");
        let level = record.level();
        let format = if atty::is(atty::Stream::Stdout) {
            format!("{} [{}] {}", time, colored_output.color(level), message)
        } else {
            format!("{} [{}] {}", time, level, message)
        };
        out.finish(format_args!("{}", format))
    });

    std::fs::create_dir_all(path).unwrap();
    match std::env::var("RUST_LOG")
        .unwrap_or_else(|_| if verbose { "debug" } else { "info" }.to_owned())
        .as_str()
    {
        // #[cfg(debug_assertions)]
        "debug" => d
            .level(LevelFilter::Debug)
            .chain(fern::DateBased::new(path, "%Y-%m-%d.log"))
            .chain(std::io::stdout())
            .apply()?,
        // #[cfg(not(debug_assertions))]
        "info" => d
            .level(LevelFilter::Info)
            .level_for("cbindgen", LevelFilter::Error)
            .chain(std::io::stdout())
            .apply()?,
        _ => panic!("only allow \"debug\" and \"info\""),
    }

    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        log::error!("{}", info);
        prev(info);
    }));

    Ok(())
}

/// Configure an opinionated way of logging, useful in tests.
pub fn configure_opinionated_test_logging() {
    // https://github.com/daboross/fern/issues/54
    // This will fail if called twice; don't worry.
    let _ = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply();
}
