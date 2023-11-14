use clap::{Args, Parser, Subcommand};
use lib_flutter_rust_bridge_codegen::codegen::ConfigDump;

// The name `Cli`, `Commands` come from https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Show debug messages.
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
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
#[derive(Debug, Args, Default, Eq, PartialEq)]
pub(crate) struct GenerateCommandArgs {
    /// Path to a YAML config file.
    ///
    /// If present, other options and flags will be ignored.
    /// Accepts the same options as the CLI, but uses snake_case keys.
    #[arg(long)]
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
    pub c_output: Option<Vec<String>>,

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
    #[arg(long, value_enum, num_args = 0.., default_missing_values = ["config", "ir"])]
    pub dump: Option<Vec<ConfigDump>>,

    /// Disable language features introduced in Dart 3.
    #[arg(long)]
    pub no_dart3: bool,

    /// If set, the program will delay error reporting until all codegen operations have completed.
    #[arg(long)]
    pub keep_going: bool,
}

#[derive(Debug, Args)]
pub(crate) struct CreateCommandArgs {
    /// Name of the new project
    pub(crate) name: String,
}

#[derive(Debug, Args)]
pub(crate) struct IntegrateCommandArgs {
    // nothing yet
}

