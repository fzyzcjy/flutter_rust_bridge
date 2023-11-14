use clap::{Args, Parser, Subcommand, ArgAction, FromArgMatches, Command};
use itertools::Itertools;
use log::debug;
use lib_flutter_rust_bridge_codegen::*;
use lib_flutter_rust_bridge_codegen::codegen::ConfigDump;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    debug!("cli={cli:?}");

    match cli.command {
        Commands::Generate(args) => codegen::generate(&args.into_config())?,
        Commands::Create(args) => integration::create(&args.name)?,
        Commands::Integrate(_) => integration::integrate()?,
    }
    Ok(())
}

// The name `Cli`, `Commands` come from https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Execute the main code generator
    Generate(GenerateCommandArgs),

    /// Create a new Flutter + Rust project
    Create(CreateCommandArgs),

    /// Integrate Rust into existing Flutter project
    Integrate(IntegrateCommandArgs),
}

// Deliberately decoupled from `codegen::Config`,
// because the command line arguments contains extra things like `--config-file`,
// which is not a config to the real codegen.
#[derive(Debug, Args)]
struct GenerateCommandArgs {
    /// Path to a YAML config file.
    ///
    /// If present, other options and flags will be ignored.
    /// Accepts the same options as the CLI, but uses snake_case keys.
    pub config_file: Option<String>,

    /// Path of input Rust code
    #[arg(short, long)]
    pub rust_input: Option<String>,

    /// Path of output generated Dart code
    #[arg(short, long)]
    pub dart_output: Option<String>,

    /// If provided, generated Dart declaration code to this separate file
    #[arg(long)]
    pub dart_decl_output: Option<String>,

    /// Output path (including file name) of generated C header, each field corresponding to that of --rust-input.
    #[arg(short, long)]
    pub c_output: Vec<String>,

    /// Crate directory for your Rust project
    #[arg(long)]
    pub rust_crate_dir: Option<String>,

    /// Output path of generated Rust code
    #[arg(long)]
    pub rust_output: Option<String>,

    /// Generated class name
    #[arg(long)]
    pub class_name: Option<String>,

    /// Line length for Dart formatting
    #[arg(long)]
    pub dart_format_line_length: Option<u32>,

    /// The generated Dart enums will have their variant names camelCased.
    #[arg(long)]
    pub dart_enums_style: bool,

    /// Skip automatically adding `mod bridge_generated;` to `lib.rs`
    #[arg(long)]
    pub no_add_mod_to_lib: bool,

    /// Path to the installed LLVM
    #[arg(long, num_args = 1..)]
    pub llvm_path: Option<Vec<String>>,

    /// LLVM compiler opts
    #[arg(long)]
    pub llvm_compiler_opts: Option<String>,

    /// Path to root of Dart project, otherwise inferred from --dart-output
    #[arg(long, num_args = 1..)]
    pub dart_root: Option<String>,

    /// Skip running build_runner even when codegen-required code is detected
    #[arg(long)]
    pub no_build_runner: bool,

    /// No use bridge in Model
    #[arg(long)]
    pub no_use_bridge_in_method: bool,

    /// extra_headers is used to add dependencies header
    ///
    /// Note that when no_use_bridge_in_method=true and extra_headers is not set,
    /// the default is `import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart'`.
    #[arg(long)]
    pub extra_headers: Option<String>,

    /// Show debug messages.
    #[arg(short, long)]
    pub verbose: bool,

    /// Enable WASM module generation.
    /// Requires: --dart-decl-output
    #[arg(long)]
    pub wasm: bool,

    /// Inline declaration of Rust bridge modules
    #[arg(long)]
    pub inline_rust: bool,

    /// Skip dependencies check.
    #[arg(long)]
    pub skip_deps_check: bool,

    /// A list of data to be dumped. If specified without a value, defaults to all.
    #[cfg(feature = "serde")]
    #[arg(long, value_enum, num_args = 0.., default_missing_values = ["config", "ir"])]
    pub dump: Option<Vec<ConfigDump>>,

    /// Disable language features introduced in Dart 3.
    #[arg(long)]
    pub no_dart3: bool,

    /// If set, the program will delay error reporting until all codegen operations have completed.
    #[arg(long)]
    pub keep_going: bool,
}

impl GenerateCommandArgs {
    fn into_config(self) -> codegen::Config {
        codegen::Config {
            rust_input: self.rust_input,
            dart_output: self.dart_output,
            dart_decl_output: self.dart_decl_output,
            c_output: self.c_output,
            rust_crate_dir: self.rust_crate_dir,
            rust_output: self.rust_output,
            class_name: self.class_name,
            dart_format_line_length: self.dart_format_line_length,
            dart_enums_style: Some(self.dart_enums_style),
            add_mod_to_lib: Some(!self.no_add_mod_to_lib),
            llvm_path: self.llvm_path,
            llvm_compiler_opts: self.llvm_compiler_opts,
            dart_root: self.dart_root,
            build_runner: Some(!self.no_build_runner),
            use_bridge_in_method: Some(!self.no_use_bridge_in_method),
            extra_headers: self.extra_headers,
            verbose: Some(self.verbose),
            wasm: Some(self.wasm),
            inline_rust: Some(self.inline_rust),
            skip_deps_check: Some(self.skip_deps_check),
            dump: self.dump,
            dart3: Some(!self.no_dart3),
            keep_going: Some(self.keep_going),
        }
    }
}

#[derive(Debug, Args)]
struct CreateCommandArgs {
    /// Name of the new project
    name: String,
}

#[derive(Debug, Args)]
struct IntegrateCommandArgs {
    // nothing yet
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use lib_flutter_rust_bridge_codegen::codegen;
    use crate::{Cli, Commands};

    #[test]
    fn test_args_to_codegen_config() {
        assert_eq!(args_to_codegen_config(vec!["", "generate"]).dart3, Some(true));
        assert_eq!(args_to_codegen_config(vec!["", "generate", "--no-dart3"]).dart3, Some(false));
    }

    fn args_to_codegen_config(args: Vec<&'static str>) -> codegen::Config {
        let cli = Cli::parse_from(args);
        let args = match cli.command {
            Commands::Generate(args) => args,
            _ => panic!()
        };
        args.into_config()
    }
}
