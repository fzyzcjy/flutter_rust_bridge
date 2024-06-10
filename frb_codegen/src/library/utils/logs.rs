//! Utilities related to logging

use crate::utils::console::MULTI_PROGRESS;
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
    let level_filter = log_level_from_env_var().unwrap_or_else(|| verbose_to_level_filter(verbose));

    if level_filter == LevelFilter::Debug {
        std::fs::create_dir_all(path).unwrap();
    }

    let mut fern_logger = fern::Dispatch::new();
    fern_logger = log_format_simple(fern_logger);
    fern_logger = match level_filter {
        LevelFilter::Debug => fern_logger
            .level(LevelFilter::Debug)
            .chain(fern::DateBased::new(path, "%Y-%m-%d.log"))
            .chain(std::io::stdout()),
        LevelFilter::Info => fern_logger
            .level(LevelFilter::Info)
            .level_for("cbindgen", LevelFilter::Error)
            .chain(std::io::stdout()),
        // frb-coverage:ignore-start
        _ => panic!("only allow \"debug\" or \"info\""),
        // frb-coverage:ignore-end
    };

    let (max_level, fern_logger) = fern_logger.into_log();
    let log_wrapper = indicatif_log_bridge::LogWrapper::new(MULTI_PROGRESS.clone(), fern_logger);

    // ref: fern.apply / LogWrapper.try_init
    log::set_boxed_logger(Box::new(log_wrapper))?;
    log::set_max_level(max_level);

    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        log::error!("{}", info);
        prev(info);
    }));

    Ok(())
}

fn log_level_from_env_var() -> Option<LevelFilter> {
    (std::env::var("RUST_LOG").ok()).map(|value| log_level_from_str(&value))
}

fn log_level_from_str(value: &str) -> LevelFilter {
    match value {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        // frb-coverage:ignore-start
        _ => panic!("{}", "unknown RUST_LOG level: {value}"),
        // frb-coverage:ignore-end
    }
}

fn verbose_to_level_filter(verbose: bool) -> LevelFilter {
    if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    }
}

fn log_format_simple(d: fern::Dispatch) -> fern::Dispatch {
    let colored_output = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    d.format(move |out, message, record| {
        let time = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");

        let level = record.level();
        let level = if atty::is(atty::Stream::Stdout) {
            colored_output.color(level).to_string()
        } else {
            level.to_string()
        };

        out.finish(format_args!(
            "[{} {} {}:{}] {}",
            time,
            level,
            record.file().unwrap_or(""),
            record.line().unwrap_or(0),
            message
        ))
    })
}

/// Configure an opinionated way of logging, useful in tests.
pub fn configure_opinionated_test_logging() {
    // https://github.com/daboross/fern/issues/54
    // This will fail if called twice; don't worry.
    let _ = log_format_simple(fern::Dispatch::new())
        .level(log_level_from_env_var().unwrap_or(LevelFilter::Debug))
        .chain(fern::Output::call(|record| println!("{}", record.args())))
        .apply();
}

#[cfg(test)]
mod tests {
    use crate::utils::logs::log_level_from_str;
    use log::LevelFilter;

    #[test]
    pub fn test_log_level_from_str() {
        assert_eq!(log_level_from_str("trace"), LevelFilter::Trace);
        assert_eq!(log_level_from_str("debug"), LevelFilter::Debug);
        assert_eq!(log_level_from_str("info"), LevelFilter::Info);
        assert_eq!(log_level_from_str("warn"), LevelFilter::Warn);
        assert_eq!(log_level_from_str("error"), LevelFilter::Error);
        assert_eq!(log_level_from_str("off"), LevelFilter::Off);
    }
}
