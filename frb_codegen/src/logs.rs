use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

/// Initializes logging to file and standard output.
/// All logs with level `debug`(`verbose`=true or `RUST_LOG`="debug") or above
/// will be recorded in `./logs/<date>.log`.
/// Logs with level `info` and above will be output to standard output, with colored tag.
///
/// # Example
///
/// ```
/// use lib_flutter_rust_bridge_codegen::init_logger;
/// init_logger("./logs/", false).expect("failed to initialize log");
/// ```
pub fn init_logger(path: &str, verbose: bool) -> Result<(), fern::InitError> {
    let colored_output = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    let mut d = fern::Dispatch::new();
    d = d.format(move |out, message, record| {
        let format = if atty::is(atty::Stream::Stdout) {
            format!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y/%m/%d %H:%M:%S"),
                colored_output.color(record.level()),
                message
            )
        } else {
            format!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y/%m/%d %H:%M:%S"),
                record.level(),
                message
            )
        };
        out.finish(format_args!("{}", format))
    });

    let log_level = match std::env::var("RUST_LOG") {
        Ok(val) => val,
        Err(_) => if verbose { "debug" } else { "info" }.to_owned(),
    };
    if log_level == "debug" {
        // #[cfg(debug_assertions)]
        std::fs::create_dir_all(path).unwrap();
        d.level(LevelFilter::Debug)
            .chain(fern::DateBased::new(path, "%Y-%m-%d.log"))
            .chain(std::io::stdout())
            .apply()?;
    } else {
        // #[cfg(not(debug_assertions))]
        d.level(LevelFilter::Info)
            .level_for("cbindgen", LevelFilter::Error)
            .chain(std::io::stdout())
            .apply()?;
    }

    Ok(())
}
