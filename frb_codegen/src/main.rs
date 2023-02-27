use std::process::exit;

use clap::Parser;
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen_multi, get_symbols_if_no_duplicates, init_logger, RawOpts,
};
use log::{debug, error, info};

fn main() -> anyhow::Result<()> {
    //  get valiable options from user input command
    let raw_opts = RawOpts::parse();
    init_logger("./logs/", raw_opts.verbose).unwrap();

    let configs = config_parse(raw_opts);
    debug!("configs={:?}", configs);

    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&configs)?;
    for (config_index, _) in configs.iter().enumerate() {
        if let Err(err) = frb_codegen_multi(&configs, config_index, &all_symbols) {
            error!("fatal: {}", err);
            exit(1);
        }
    }

    info!("Now go and use it :)");
    Ok(())
}
