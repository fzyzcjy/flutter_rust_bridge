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
    use clap::Parser;
    use lib_flutter_rust_bridge_codegen::codegen;
    use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_test_logging;
    use crate::binary::commands::{Cli, Commands};
    use crate::binary::commands_parser::compute_codegen_config;

    #[test]
    fn test_compute_codegen_config_e2e() {
        configure_opinionated_test_logging();

        fn body(args: Vec<&'static str>) -> codegen::Config {
            let cli = Cli::parse_from(args);
            let args = match cli.command {
                Commands::Generate(args) => args,
                _ => panic!()
            };
            compute_codegen_config(args).unwrap()
        }

        assert_eq!(body(vec!["", "generate"]).dart3, Some(true));
        assert_eq!(body(vec!["", "generate", "--no-dart3"]).dart3, Some(false));
    }
}
