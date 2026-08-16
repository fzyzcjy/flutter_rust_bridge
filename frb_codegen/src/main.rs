//! Main documentation is in <https://github.com/fzyzcjy/flutter_rust_bridge>
//!
//! A thin command line interface. Please avoid putting logic here
//! (instead be in `lib.rs` and so on)

mod binary;

use crate::binary::commands::{
    Cli, Commands, CreateOrIntegrateCommandCommonArgs, GenerateCommandArgs,
};
use crate::binary::commands_parser::{compute_codegen_config, compute_codegen_meta_config};
use clap::Parser;
use lib_flutter_rust_bridge_codegen::integration::{CreateConfig, IntegrateConfig};
use lib_flutter_rust_bridge_codegen::misc::FvmInstallMode;
use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_logging;
use lib_flutter_rust_bridge_codegen::*;
use log::{debug, error, warn};
use std::env::set_current_dir;
use std::fs::canonicalize;
use std::io;
use std::path::Path;
use std::process::exit;

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
            if args.primary.switch_to_config_parent {
                switch_to_config_parent_directory(&args)?;
            }

            let meta_config = compute_codegen_meta_config(&args);
            let config = compute_codegen_config(args.primary)?;

            codegen::generate_with_fvm_install_mode(
                config,
                meta_config,
                FvmInstallMode::from_skip_fvm_install(args.skip_fvm_install),
            )?
        }
        Commands::Create(args) => integration::create(CreateConfig {
            name: args.name,
            org: args.org,
            enable_local_dependency: args.common.local,
            rust_crate_name: args.common.rust_crate_name.clone(),
            rust_crate_dir: compute_rust_crate_dir(&args.common),
            template: args.template.into(),
            integration_backend: args.integration_backend.into(),
            platforms: args.platforms,
            fvm_install_mode: FvmInstallMode::from_skip_fvm_install(args.skip_fvm_install),
        })?,
        Commands::Integrate(args) => integration::integrate(IntegrateConfig {
            enable_write_lib: !args.no_write_lib,
            enable_integration_test: !args.no_integration_test,
            enable_dart_fix: !args.no_dart_fix,
            enable_dart_format: !args.no_dart_format,
            enable_local_dependency: args.common.local,
            rust_crate_name: args.common.rust_crate_name.clone(),
            rust_crate_dir: compute_rust_crate_dir(&args.common),
            template: args.template.into(),
            integration_backend: args.integration_backend.into(),
            platforms: args.platforms,
            fvm_install_mode: FvmInstallMode::from_skip_fvm_install(args.skip_fvm_install),
        })?,
        Commands::BuildWeb(args) => build_web::build(
            args.dart_root,
            args.dart_coverage,
            args.args,
            FvmInstallMode::from_skip_fvm_install(args.skip_fvm_install),
        )?,
        Commands::InternalGenerate(_args) => internal::generate()?,
    }
    Ok(())
}

fn compute_rust_crate_dir(config: &CreateOrIntegrateCommandCommonArgs) -> String {
    let rust_crate_dir = config.rust_crate_dir.clone().unwrap_or("rust".to_owned());
    let path = Path::new(&rust_crate_dir);
    if path.is_absolute() {
        warn!("Argument given to --rust-crate-dir was an absolute Path. It will still be interpreted as relative to the new project root.")
    }
    rust_crate_dir
}

fn switch_to_config_parent_directory(args: &GenerateCommandArgs) -> Result<(), io::Error> {
    if let Some(config_path) = args.primary.config_file.as_deref() {
        if let Some(parent) = canonicalize(Path::new(config_path))?.parent() {
            set_current_dir(parent)?;
        } else {
            error!(
                "Couldn't switch current working directory to the specified config path's parent"
            );
            exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::binary::commands::{Cli, GenerateCommandArgs};
    use crate::binary::test_utils::set_cwd_test_fixture;
    use crate::{main_given_cli, switch_to_config_parent_directory};
    use clap::Parser;
    use serial_test::serial;
    use std::fs::{canonicalize, File};
    use std::{env, fs};

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

    #[test]
    #[serial]
    fn test_switch_to_config_parent_directory() -> anyhow::Result<()> {
        let temp_directory = std::env::temp_dir();

        let test_path = temp_directory.join("flutter_rust_bridge.yaml");

        let mut generate_command_args = GenerateCommandArgs::default();
        generate_command_args.primary.switch_to_config_parent = true;
        generate_command_args.primary.config_file = Some(test_path.to_str().unwrap().to_string());

        File::create(&test_path)?;

        switch_to_config_parent_directory(&generate_command_args)?;

        assert_eq!(
            canonicalize(temp_directory)?,
            canonicalize(std::env::current_dir()?)?
        );

        fs::remove_file(&test_path)?;

        Ok(())
    }
}
