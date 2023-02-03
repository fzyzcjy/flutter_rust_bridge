use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

/// Initializes logging to file and standard output.
/// All logs with level `debug`(with parameter `verbose`=true or system variable `RUST_LOG`="debug") or above
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

    match std::env::var("RUST_LOG")
        .unwrap_or_else(|_| if verbose { "debug" } else { "info" }.to_owned())
        .as_str()
    {
        // #[cfg(debug_assertions)]
        "debug" => {
            std::fs::create_dir_all(path).unwrap();
            d.level(LevelFilter::Debug)
                .chain(fern::DateBased::new(path, "%Y-%m-%d.log"))
                .chain(std::io::stdout())
                .apply()?
        }
        // #[cfg(not(debug_assertions))]
        "info" => d
            .level(LevelFilter::Info)
            .level_for("cbindgen", LevelFilter::Error)
            .chain(std::io::stdout())
            .apply()?,
        _ => panic!("only allow \"debug\" and \"info\""),
    }

    Ok(())
}
