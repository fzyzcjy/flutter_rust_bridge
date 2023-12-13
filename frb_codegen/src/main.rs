//! Main documentation is in <https://github.com/fzyzcjy/flutter_rust_bridge>
//!
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
        Commands::Generate(args) => codegen::generate(compute_codegen_config(args.primary)?)?,
        Commands::Create(args) => integration::create(&args.name)?,
        Commands::BuildWeb(args) => build_web::build(args.dart_root, args.args)?,
        Commands::Integrate(args) => integration::integrate(!args.no_enable_integration_test)?,
        Commands::InternalGenerate(_args) => internal::generate()?,
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
    use std::env;

    #[test]
    #[serial]
    fn test_execute_generate_on_frb_example_dart_minimal() -> anyhow::Result<()> {
        body_execute_generate("dart_minimal")
    }

    #[test]
    #[serial]
    fn test_execute_generate_on_frb_example_pure_dart() -> anyhow::Result<()> {
        body_execute_generate("pure_dart")
    }

    fn body_execute_generate(name: &str) -> anyhow::Result<()> {
        configure_opinionated_test_logging();

        if env::var("FRB_SKIP_GENERATE_FRB_EXAMPLE_TEST").unwrap_or_default() == "1" {
            return Ok(());
        }

        set_cwd_test_fixture(&format!("../../frb_example/{name}"))?;
        main_given_cli(Cli::parse_from(vec!["", "generate"]))
    }
}
