use flutter_rust_bridge::frb;
use log::{Level, Log, Metadata, Record, RecordBuilder};

// use log::{logger, LevelFilter, Record};

#[frb(init)]
pub fn init_app() {
    // flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    log::error!("/////// testing log levels ///////");
    log::error!("inital level : {}", log::max_level());
    log::trace!("-- trace test --");
    log::debug!("-- debug test --");
    log::info!("-- info test --");
    log::warn!("-- warn test --");
    log::error!("-- error test --");
    // log::set_max_level(log::LevelFilter::Trace);
    log::set_max_level(log::LevelFilter::Warn);
    log::error!(" ///// changed level to warn : {}", log::max_level());
    log::trace!("-- trace test --");
    log::debug!("-- debug test --");
    log::info!("-- info test --");
    log::warn!("-- warn test --");
    log::error!("-- error test --");

    log::info!(
        "From Rust: Minimal adder called with params {:?} and {:?}",
        a,
        b
    );
    a + b
}
