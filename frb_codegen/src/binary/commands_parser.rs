use anyhow::{Context, Result};
use log::debug;
use lib_flutter_rust_bridge_codegen::codegen::Config;
use crate::binary::commands::GenerateCommandArgs;

pub(crate) fn compute_codegen_config(args: GenerateCommandArgs) -> Result<Config> {
    if args == Default::default() {
        debug!("compute_codegen_config: mode=from_files_auto");
        return Config::from_files_auto();
    }

    if let Some(config_file) = args.config_file {
        debug!("compute_codegen_config: mode=config_file");
        return Config::from_config_file(&config_file)?.context("Cannot find config_file");
    }

    debug!("compute_codegen_config: mode=from_naive_generate_command_args");
    Ok(compute_codegen_config_from_naive_command_args(args))
}

fn compute_codegen_config_from_naive_command_args(args: GenerateCommandArgs) -> Config {
    Config {
        rust_input: args.rust_input,
        dart_output: args.dart_output,
        dart_decl_output: args.dart_decl_output,
        c_output: args.c_output,
        rust_crate_dir: args.rust_crate_dir,
        rust_output: args.rust_output,
        class_name: args.class_name,
        dart_format_line_length: args.dart_format_line_length,
        dart_enums_style: Some(args.dart_enums_style),
        add_mod_to_lib: Some(!args.no_add_mod_to_lib),
        llvm_path: args.llvm_path,
        llvm_compiler_opts: args.llvm_compiler_opts,
        dart_root: args.dart_root,
        build_runner: Some(!args.no_build_runner),
        use_bridge_in_method: Some(!args.no_use_bridge_in_method),
        extra_headers: args.extra_headers,
        wasm: Some(args.wasm),
        inline_rust: Some(args.inline_rust),
        skip_deps_check: Some(args.skip_deps_check),
        dump: args.dump,
        dart3: Some(!args.no_dart3),
        keep_going: Some(args.keep_going),
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;
    use clap::Parser;
    use lib_flutter_rust_bridge_codegen::codegen;
    use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_test_logging;
    use crate::binary::commands::{Cli, Commands};
    use crate::binary::commands_parser::compute_codegen_config;
    use serial_test::serial;

    fn set_cwd_test_fixture(name: &str) -> anyhow::Result<()> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_fixtures");
        d.push(name);
        println!("set_cwd_test_fixture: {d:?}");
        Ok(std::env::set_current_dir(d)?)
    }

    // need to run serially, otherwise working directory will override each other
    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_from_files_auto_flutter_rust_bridge_yaml() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("commands_parser/flutter_rust_bridge_yaml")?;

        let config = run_command_line(vec!["", "generate"]);
        assert_eq!(config.rust_input.unwrap(), "hello.rs");
        assert_eq!(config.dart3.unwrap(), false);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_from_files_auto_pubspec_yaml() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("commands_parser/pubspec_yaml")?;

        let config = run_command_line(vec!["", "generate"]);
        assert_eq!(config.rust_input.unwrap(), "hello.rs");
        assert_eq!(config.dart3.unwrap(), false);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_config_file() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("commands_parser/config_file")?;

        println!("open");
        File::open("hello.yaml")?;

        let config = run_command_line(vec!["", "generate", "--config-file", "hello.yaml"]);
        assert_eq!(config.rust_input.unwrap(), "hello.rs");
        assert_eq!(config.dart3.unwrap(), false);

        Ok(())
    }

    #[test]
    fn test_compute_codegen_config_mode_from_naive_generate_command_args() {
        configure_opinionated_test_logging();

        // bool flags
        assert_eq!(run_command_line(vec!["", "generate", "--class-name", "hello"]).dart3, Some(true));
        assert_eq!(run_command_line(vec!["", "generate", "--class-name", "hello", "--no-dart3"]).dart3, Some(false));
    }

    fn run_command_line(args: Vec<&'static str>) -> codegen::Config {
        let cli = Cli::parse_from(args);
        let args = match cli.command {
            Commands::Generate(args) => args,
            _ => panic!()
        };
        compute_codegen_config(args).unwrap()
    }
}
