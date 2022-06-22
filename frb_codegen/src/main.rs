use env_logger::Env;
use lib_flutter_rust_bridge_codegen::{config_parse, frb_codegen, RawOpts};
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

    // before generation, get all symbols to be generated
    let mut all_symbols = Vec::new();
    for config in &configs {
        let curr_symbols = config
            .get_ir_file()
            .funcs
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<_>>();
        all_symbols.extend(curr_symbols);
    }

    // generation of rust api for ffi
    let mut defined_symbols = vec![];
    for (i, config) in configs.iter().enumerate() {
        frb_codegen(config, &mut defined_symbols, &all_symbols, i + 1).unwrap();
    }

    info!("Now go and use it :)");
    Ok(())
}
