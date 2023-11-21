//! A thin command line interface. Please avoid putting logic here
//! (instead be in `lib.rs` and so on)

mod binary;

use crate::binary::commands::{Cli, Commands};
use crate::binary::commands_parser::compute_codegen_config;
use clap::Parser;
use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_logging;
use lib_flutter_rust_bridge_codegen::*;
use log::debug;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    configure_opinionated_logging("./logs/", cli.verbose)?;
    main_given_cli(cli)
}

// Only use as entrypoint of tests
fn main_given_cli(cli: Cli) -> anyhow::Result<()> {
    debug!("cli={cli:?}");
    match cli.command {
        Commands::Generate(args) => codegen::generate(compute_codegen_config(args)?)?,
        Commands::Create(args) => integration::create(&args.name)?,
        Commands::Integrate(_) => integration::integrate()?,
        Commands::InternalGenerate(args) => todo!(),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::binary::commands::Cli;
    use crate::binary::test_utils::set_cwd_test_fixture;
    use crate::main_given_cli;
    use clap::Parser;
    use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_test_logging;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_execute_generate_on_frb_example_pure_dart() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("../../frb_example/pure_dart")?;
        main_given_cli(Cli::parse_from(vec!["", "generate"]))
    }
}
