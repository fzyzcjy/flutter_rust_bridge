use env_logger::Env;
use lib_flutter_rust_bridge_codegen::{config_parse, ensure_tools_available, frb_codegen, RawOpts};
use log::info;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let raw_opts = RawOpts::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or(if raw_opts.verbose {
        "debug"
    } else {
        "info"
    }))
    .init();

    // initial config(s) before generation
    ensure_tools_available()?;
    let configs = config_parse(raw_opts);

    // primary generation of rust api for ffi
    for each_config in &configs {
        frb_codegen(each_config).unwrap();
    }
    // TODO:primary check duplicated apis among all rust blocks

    info!("Now go and use it :)");
    Ok(())
}
