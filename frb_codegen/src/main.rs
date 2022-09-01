use std::process::exit;

use clap::Parser;
use env_logger::Env;
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};
use log::{debug, error, info};

fn main() -> anyhow::Result<()> {
    //  get valiable options from user input command
    let raw_opts = RawOpts::parse();
    env_logger::Builder::from_env(Env::default().default_filter_or(if raw_opts.verbose {
        "debug"
    } else {
        "info,cbindgen=error"
    }))
    .init();

    let configs = config_parse(raw_opts);
    debug!("configs={:?}", configs);

    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&configs)?;
    for config in configs.iter() {
        if let Err(err) = frb_codegen(config, &all_symbols) {
            error!("fatal: {}", err);
            exit(1);
        }
    }

    info!("Now go and use it :)");
    Ok(())
}
