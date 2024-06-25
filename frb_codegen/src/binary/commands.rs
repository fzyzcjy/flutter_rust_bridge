use crate::codegen::ConfigDumpContent;
use clap::{Args, Parser, Subcommand, ValueEnum};
use lib_flutter_rust_bridge_codegen::misc::Template;
use std::path::PathBuf;

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

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Execute the main code generator
    Generate(GenerateCommandArgs),

    /// Create a new Flutter + Rust project
    Create(CreateCommandArgs),

    /// Integrate Rust into existing Flutter project
    Integrate(IntegrateCommandArgs),

    /// Compile for the Web (WASM)
    BuildWeb(BuildWebCommandArgs),

    /// Generate internally used code
    #[clap(hide = true)]
    InternalGenerate(InternalGenerateCommandArgs),
}

#[derive(Debug, Args, Default, Eq, PartialEq)]
pub(crate) struct GenerateCommandArgs {
    /// Automatically re-generate the output whenever the changes are detected on the input files
    #[arg(long)]
    pub watch: bool,

    #[clap(flatten)]
    pub primary: GenerateCommandArgsPrimary,
}

// Deliberately decoupled from `codegen::Config`,
// because the command line arguments contains extra things like `--config-file`,
// which is not a config to the real codegen.
#[derive(Debug, Args, Default, Eq, PartialEq, Clone)]
pub(crate) struct GenerateCommandArgsPrimary {
    /// Path to a YAML config file.
    ///
    /// If present, other options and flags will be ignored.
    /// Accepts the same options as the CLI, but uses snake_case keys.
    #[arg(long)]
    pub config_file: Option<String>,

    /// Input Rust files, such as `crate::api,crate::hello::world,another-third-party-crate`
    #[arg(short, long)]
    pub rust_input: Option<String>,

    /// Directory of output generated Dart code
    #[arg(short, long)]
    pub dart_output: Option<String>,

    /// Output path of generated C header
    #[arg(short, long)]
    pub c_output: Option<String>,

    /// Duplicate the files generated at the location `--c-output` specifies
    #[arg(long)]
    pub duplicated_c_output: Option<Vec<String>>,

    /// Crate directory for your Rust project
    #[arg(long)]
    pub rust_root: Option<String>,

    /// Output path of generated Rust code
    #[arg(long)]
    pub rust_output: Option<String>,

    /// Generated dart entrypoint class name
    #[arg(long)]
    pub dart_entrypoint_class_name: Option<String>,

    /// Line length for Dart formatting
    #[arg(long)]
    pub dart_format_line_length: Option<u32>,

    /// Raw header of output generated Dart code, pasted as-it-is.
    #[arg(long)]
    pub dart_preamble: Option<String>,

    /// Raw header of output generated Rust code, pasted as-it-is.
    #[arg(long)]
    pub rust_preamble: Option<String>,

    /// The generated Dart enums will not have their variant names camelCased.
    #[arg(long)]
    pub no_dart_enums_style: bool,

    /// Skip automatically adding `mod frb_generated;` to `lib.rs`
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

    /// extra_headers is used to add dependencies header
    #[arg(long)]
    pub extra_headers: Option<String>,

    /// Disable web module generation.
    #[arg(long)]
    pub no_web: bool,

    /// Skip dependencies check.
    #[arg(long)]
    pub no_deps_check: bool,

    /// The value for defaultExternalLibraryLoader.webPrefix
    #[arg(long)]
    pub default_external_library_loader_web_prefix: Option<String>,

    /// Disable language features introduced in Dart 3.
    #[arg(long)]
    pub no_dart3: bool,

    /// Enable full dependencies
    #[arg(long)]
    pub full_dep: bool,

    /// Use local version instead of the release version
    #[arg(long, hide = true)]
    pub local: bool,

    /// Enable parsing types with lifetimes (e.g. references and borrows)
    #[arg(long)]
    pub enable_lifetime: bool,

    /// Let 64 bit types be translated to `int`s instead of types like `BigInt`s
    #[arg(long)]
    pub type_64bit_int: bool,

    /// Whether default Dart code is asynchronous or synchronous
    #[arg(long)]
    pub no_default_dart_async: bool,

    /// If having error when, for example, parsing a function, directly stop instead of continue and skip it
    #[arg(long)]
    pub stop_on_error: bool,

    /// A list of data to be dumped. If specified without a value, defaults to all.
    #[arg(long, value_enum, num_args = 0.., default_missing_values = ["config", "ir"])]
    pub dump: Option<Vec<ConfigDumpContent>>,

    /// Dump all internal data. Same as `--dump` with all possible choices chosen.
    #[arg(long)]
    pub dump_all: bool,
}

#[derive(Debug, Args)]
pub(crate) struct CreateCommandArgs {
    /// Name of the new project
    pub(crate) name: String,

    /// The organization responsible for your new Flutter project, in reverse domain name notation.
    #[clap(long)]
    pub(crate) org: Option<String>,

    #[clap(flatten)]
    pub common: CreateOrIntegrateCommandCommonArgs,

    /// The template type to use to generate the flutter files.
    #[clap(short, long, value_enum, default_value = "app")]
    pub template: TemplateArg,
}

#[derive(Debug, Args)]
pub(crate) struct IntegrateCommandArgs {
    /// Generate code related to integration test
    #[arg(long)]
    pub(crate) no_enable_integration_test: bool,

    #[clap(flatten)]
    pub common: CreateOrIntegrateCommandCommonArgs,

    /// The template type to use for integration. This should usually match the type of flutter project
    /// being integrating with.
    #[clap(short, long, value_enum, default_value = "app")]
    pub template: TemplateArg,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub(crate) enum TemplateArg {
    /// (default) a Flutter application
    App,
    /// A shareable Flutter project that can be used across multiple Flutter applications.
    Plugin,
}

#[derive(Debug, Args)]
pub(crate) struct CreateOrIntegrateCommandCommonArgs {
    /// The name of the generated Rust crate
    #[arg(long)]
    pub rust_crate_name: Option<String>,

    /// The directory of the generated Rust crate
    #[arg(long)]
    pub rust_crate_dir: Option<String>,

    /// Use local version instead of the release version
    #[arg(long, hide = true)]
    pub local: bool,
}

#[derive(Debug, Args)]
#[command(disable_help_flag = true)]
pub(crate) struct BuildWebCommandArgs {
    /// Path to root of Dart project, otherwise inferred from current working directory
    #[arg(long)]
    pub dart_root: Option<PathBuf>,

    /// Run Dart code with coverage
    #[arg(long, hide = true)]
    pub dart_coverage: bool,

    // https://stackoverflow.com/questions/72399790/clap-capture-all-remaining-arguments-in-one-field-in-derive-api
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub(crate) args: Vec<String>,
}

#[derive(Debug, Args)]
pub(crate) struct InternalGenerateCommandArgs {}

impl From<TemplateArg> for Template {
    fn from(value: TemplateArg) -> Self {
        match value {
            TemplateArg::App => Template::App,
            TemplateArg::Plugin => Template::Plugin,
        }
    }
}
