//! A thin command line interface. Please avoid putting logic here
//! (instead be in `lib.rs` and so on)

mod binary;

use clap::Parser;
use log::debug;
use lib_flutter_rust_bridge_codegen::*;
use crate::binary::commands::{Cli, Commands};
use crate::binary::commands_parser::compute_codegen_config;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    debug!("cli={cli:?}");

    match cli.command {
        Commands::Generate(args) => codegen::generate(&compute_codegen_config(args)?)?,
        Commands::Create(args) => integration::create(&args.name)?,
        Commands::Integrate(_) => integration::integrate()?,
    }
    Ok(())
}
