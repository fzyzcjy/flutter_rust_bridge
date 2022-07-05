use env_logger::Env;
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};
use log::{debug, info};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    //  get valiable options from user input command
    let raw_opts = RawOpts::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or(if raw_opts.verbose {
        "debug"
    } else {
        "info"
    }))
    .init();

    let configs = config_parse(raw_opts);
    debug!("configs={:?}", configs);

    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&configs)?;
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }

    info!("Now go and use it :)");
    Ok(())
}
