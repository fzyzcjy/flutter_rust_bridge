//! Main documentation is in <https://github.com/fzyzcjy/flutter_rust_bridge>
//!
//! A thin command line interface. Please avoid putting logic here
//! (instead be in `lib.rs` and so on)

mod binary;

use crate::binary::commands::{Cli, Commands, CreateOrIntegrateCommandCommonArgs};
use crate::binary::commands_parser::{compute_codegen_config, compute_codegen_meta_config};
use clap::Parser;
use lib_flutter_rust_bridge_codegen::integration::{CreateConfig, IntegrateConfig};
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
        Commands::Generate(args) => {
            let meta_config = compute_codegen_meta_config(&args);
            let config = compute_codegen_config(args.primary)?;
            codegen::generate(config, meta_config)?
        }
        Commands::Create(args) => integration::create(CreateConfig {
            name: args.name,
            org: args.org,
            enable_local_dependency: args.common.local,
            rust_crate_name: args.common.rust_crate_name.clone(),
            rust_crate_dir: compute_rust_crate_dir(&args.common),
            template: args.template.into(),
        })?,
        Commands::Integrate(args) => integration::integrate(IntegrateConfig {
            enable_integration_test: !args.no_enable_integration_test,
            enable_local_dependency: args.common.local,
            rust_crate_name: args.common.rust_crate_name.clone(),
            rust_crate_dir: compute_rust_crate_dir(&args.common),
            template: args.template.into(),
        })?,
        Commands::BuildWeb(args) => {
            build_web::build(args.dart_root, args.dart_coverage, args.args)?
        }
        Commands::InternalGenerate(_args) => internal::generate()?,
    }
    Ok(())
}

fn compute_rust_crate_dir(config: &CreateOrIntegrateCommandCommonArgs) -> String {
    config.rust_crate_dir.clone().unwrap_or("rust".to_owned())
}

#[cfg(test)]
mod tests {
    use crate::binary::commands::Cli;
    use crate::binary::test_utils::set_cwd_test_fixture;
    use crate::main_given_cli;
    use clap::Parser;
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

    // we do not care about coverage of test themselves
    // frb-coverage:ignore-start
    fn body_execute_generate(name: &str) -> anyhow::Result<()> {
        // if want verbose log, enable it
        // configure_opinionated_test_logging();

        if env::var("FRB_SKIP_GENERATE_FRB_EXAMPLE_TEST").unwrap_or_default() == "1" {
            return Ok(());
        }

        set_cwd_test_fixture(&format!("../../frb_example/{name}"))?;
        main_given_cli(Cli::parse_from(vec!["", "generate"]))
    }
    // frb-coverage:ignore-end
}
