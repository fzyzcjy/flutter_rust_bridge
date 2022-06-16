use env_logger::Env;
use lib_flutter_rust_bridge_codegen::{config_parse, ensure_tools_available, frb_codegen, RawOpts};
use log::{debug, info};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let raw_opts = RawOpts::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or(if raw_opts.verbose {
        "debug"
    } else {
        "info"
    }))
    .init();

    let configs = config_parse(raw_opts);
    debug!("configs={:?}", configs);

    // primary generation of rust api for ffi
    for config in &configs {
        frb_codegen(config).unwrap();
    }
    // TODO:primary check duplicated apis among all rust blocks

    info!("Now go and use it :)");
    Ok(())
}
