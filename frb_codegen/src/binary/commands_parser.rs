use crate::binary::commands::{GenerateCommandArgs, GenerateCommandArgsPrimary};
use anyhow::{Context, Result};
use lib_flutter_rust_bridge_codegen::codegen::{Config, MetaConfig};

pub(crate) fn compute_codegen_config(args: GenerateCommandArgsPrimary) -> Result<Config> {
    let config_from_file = if let Some(config_file) = &args.config_file {
        Config::from_config_file(config_file)?.context("Cannot find config_file")?
    } else {
        Config::from_files_auto_option()?.unwrap_or_default()
    };

    let config_from_args = compute_codegen_config_from_naive_command_args(args);

    Ok(Config::merge(config_from_args, config_from_file))
}

pub(crate) fn compute_codegen_meta_config(args: &GenerateCommandArgs) -> MetaConfig {
    MetaConfig { watch: args.watch }
}

fn compute_codegen_config_from_naive_command_args(args: GenerateCommandArgsPrimary) -> Config {
    fn positive_bool_arg(x: bool) -> Option<bool> {
        x.then_some(true)
    }

    // arg like "no_something"
    fn negative_bool_arg(x: bool) -> Option<bool> {
        x.then_some(false)
    }

    Config {
        base_dir: None,
        rust_input: args.rust_input,
        dart_output: args.dart_output,
        c_output: args.c_output,
        duplicated_c_output: args.duplicated_c_output,
        rust_root: args.rust_root,
        rust_output: args.rust_output,
        dart_entrypoint_class_name: args.dart_entrypoint_class_name,
        dart_format_line_length: args.dart_format_line_length,
        dart_preamble: args.dart_preamble,
        rust_preamble: args.rust_preamble,
        dart_enums_style: negative_bool_arg(args.no_dart_enums_style),
        add_mod_to_lib: negative_bool_arg(args.no_add_mod_to_lib),
        llvm_path: args.llvm_path,
        llvm_compiler_opts: args.llvm_compiler_opts,
        dart_root: args.dart_root,
        build_runner: negative_bool_arg(args.no_build_runner),
        extra_headers: args.extra_headers,
        web: negative_bool_arg(args.no_web),
        deps_check: negative_bool_arg(args.no_deps_check),
        dart3: negative_bool_arg(args.no_dart3),
        full_dep: positive_bool_arg(args.full_dep),
        local: positive_bool_arg(args.local),
        default_external_library_loader_web_prefix: args.default_external_library_loader_web_prefix,
        dart_type_rename: None, // complex type, not supported on command line yet
        enable_lifetime: positive_bool_arg(args.enable_lifetime),
        type_64bit_int: positive_bool_arg(args.type_64bit_int),
        default_dart_async: negative_bool_arg(args.no_default_dart_async),
        stop_on_error: positive_bool_arg(args.stop_on_error),
        dump: args.dump,
        dump_all: positive_bool_arg(args.dump_all),
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::commands::{Cli, Commands};
    use crate::binary::commands_parser::compute_codegen_config;
    use crate::binary::test_utils::set_cwd_test_fixture;
    use clap::Parser;
    use itertools::concat;
    use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_test_logging;
    use lib_flutter_rust_bridge_codegen::{codegen, if_then_some};
    use serial_test::serial;

    // need to run serially, otherwise working directory will override each other
    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_from_files_auto_flutter_rust_bridge_yaml(
    ) -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("binary/commands_parser/flutter_rust_bridge_yaml")?;

        let config = run_command_line(vec!["", "generate"])?;
        assert_eq!(config.rust_input.unwrap(), "crate::hello".to_string());
        assert!(!config.dart3.unwrap());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_from_files_auto_pubspec_yaml() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("binary/commands_parser/pubspec_yaml")?;

        let config = run_command_line(vec!["", "generate"])?;
        assert_eq!(config.rust_input.unwrap(), "crate::hello".to_string());
        assert!(!config.dart3.unwrap());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_from_files_auto_pubspec_yaml_faulty() -> anyhow::Result<()>
    {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("binary/commands_parser/faulty_pubspec_yaml")?;

        let result = run_command_line(vec!["", "generate"]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .source()
            .unwrap()
            .to_string()
            .contains("misspelled_dart3"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_config_file() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("binary/commands_parser/config_file")?;

        let config = run_command_line(vec!["", "generate", "--config-file", "hello.yaml"])?;
        assert_eq!(config.rust_input.unwrap(), "crate::hello".to_string());
        assert!(!config.dart3.unwrap());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_config_file_faulty_file() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("binary/commands_parser/flutter_rust_bridge_yaml")?;
        let result = run_command_line(vec![
            "",
            "generate",
            "--config-file",
            "faulty_flutter_rust_bridge.yaml",
        ]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .source()
            .unwrap()
            .to_string()
            .contains("misspelled_dart3"));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_mode_from_naive_generate_command_args() {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("binary/commands_parser").unwrap(); // use whatever folder without config file

        // bool flags
        let common_args = vec![
            "",
            "generate",
            "--rust-input",
            "crate::hello",
            "--dart-output",
            "hello.dart",
            "--c-output",
            "hello.h",
        ];
        let config = run_command_line(common_args.clone()).expect("failed to parse cli args");
        assert_eq!(config.dart3, None);
        assert_eq!(config.rust_input.unwrap(), "crate::hello".to_string());
        assert_eq!(
            run_command_line(concat([common_args.clone(), vec!["--no-dart3"]]))
                .expect("failed to parse cli args")
                .dart3,
            Some(false)
        );
    }

    #[test]
    #[serial]
    fn test_compute_codegen_config_from_both_file_and_command_line() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        set_cwd_test_fixture("binary/commands_parser/flutter_rust_bridge_yaml")?;

        let config = run_command_line(vec!["", "generate", "--llvm-path", "/my/path"])?;
        assert_eq!(config.rust_input.unwrap(), "crate::hello".to_string());
        assert!(!config.dart3.unwrap());
        assert_eq!(config.llvm_path, Some(vec!["/my/path".to_owned()]));

        Ok(())
    }

    fn run_command_line(args: Vec<&'static str>) -> anyhow::Result<codegen::Config> {
        let cli = Cli::parse_from(args);
        let args = if_then_some!(let Commands::Generate(args) = cli.command, args).unwrap();
        compute_codegen_config(args.primary)
    }
}
