use clap::{Args, Parser, Subcommand, ArgAction, FromArgMatches, Command};
use itertools::Itertools;
use serde::Deserialize;
use lib_flutter_rust_bridge_codegen::*;
use lib_flutter_rust_bridge_codegen::codegen::ConfigDump;

fn main() -> anyhow::Result<()> {
    let app = <Cli as clap::CommandFactory>::command();
    let app = add_negations(app);
    let matches = app.try_get_matches()?;
    let cli = Cli::from_arg_matches(&matches)?;

    println!("cli={cli:#?}");

    match cli.command {
        Commands::Generate(args) => codegen::generate(&codegen::Config {})?,
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

// Every option should have a --no- variant that makes it as if it was
// never passed.
// https://github.com/clap-rs/clap/issues/815
// https://github.com/ducaale/xh/blob/1a74a521e1f1def2f9463abcfe05b448f04c27be/src/cli.rs#L583
fn add_negations(mut command: Command) -> Command {
    let subcommand_names = command.get_subcommands().map(|c| c.get_name().to_string()).collect_vec();
    for subcommand_name in subcommand_names {
        command = command.mut_subcommand(subcommand_name, add_negations);
    }

    let negations: Vec<_> = command
        .get_arguments()
        .filter(|a| !a.is_positional())
        .map(|opt| {
            let long = opt.get_long().expect("long option");
            let negated_long: &'static str = Box::leak(format!("no-{}", long).into_boxed_str());

            clap::Arg::new(negated_long)
                .long(negated_long)
                .action(ArgAction::SetTrue)
                .help(format!("The opposite of --{long}"))
                // overrides_with is enough to make the flags take effect
                // We never have to check their values, they'll simply
                // unset previous occurrences of the original flag
                .overrides_with(opt.get_id())
        })
        .collect();
    command = command.args(negations);

    command
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

// TODO clean up the api and docs
#[derive(Debug, Args, Deserialize)]
struct GenerateCommandArgs {
    /// Path of input Rust code
    #[arg(short, long)]
    pub rust_input: Option<String>,

    /// Path of output generated Dart code
    #[arg(short, long)]
    pub dart_output: Option<String>,

    /// Path to a YAML config file.
    ///
    /// If present, other options and flags will be ignored.
    /// Accepts the same options as the CLI, but uses snake_case keys.
    #[serde(skip)]
    pub config_file: Option<String>,

    /// If provided, generated Dart declaration code to this separate file
    #[arg(long)]
    pub dart_decl_output: Option<String>,

    // TODO single c_output, then what about ios + macos?
    /// Output path (including file name) of generated C header, each field corresponding to that of --rust-input.
    #[arg(short, long)]
    pub c_output: Option<String>,

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
    #[arg(long, default_value = "80")]
    pub dart_format_line_length: u32,

    /// The generated Dart enums will have their variant names camelCased.
    #[arg(long)]
    #[serde(default)]
    pub dart_enums_style: bool,

    /// Skip automatically adding `mod bridge_generated;` to `lib.rs`
    #[arg(long)]
    #[serde(default)]
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

    // TODO about negation?
    /// Skip running build_runner even when codegen-required code is detected
    #[arg(long)]
    #[serde(default)]
    pub no_build_runner: bool,

    /// No use bridge in Model
    #[arg(long)]
    #[serde(default)]
    pub no_use_bridge_in_method: bool,

    /// extra_headers is used to add dependencies header
    ///
    /// Note that when no_use_bridge_in_method=true and extra_headers is not set,
    /// the default is `import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart'`.
    #[arg(long)]
    #[serde(default)]
    pub extra_headers: Option<String>,

    /// Show debug messages.
    #[arg(short, long)]
    #[serde(default)]
    pub verbose: bool,

    /// Enable WASM module generation.
    /// Requires: --dart-decl-output
    #[arg(long)]
    #[serde(default)]
    pub wasm: bool,

    /// Inline declaration of Rust bridge modules
    #[arg(long)]
    #[serde(default)]
    pub inline_rust: bool,

    /// Skip dependencies check.
    #[arg(long)]
    #[serde(default)]
    pub skip_deps_check: bool,

    /// A list of data to be dumped. If specified without a value, defaults to all.
    #[cfg(feature = "serde")]
    #[arg(long, value_enum, num_args = 0.., default_missing_values = ["config", "ir"])]
    pub dump: Option<Vec<ConfigDump>>,

    /// Disable language features introduced in Dart 3.
    #[arg(long = "no-dart3", action(ArgAction::SetFalse))]
    #[serde(default = "r#true")]
    pub dart3: bool,

    /// If set, the program will delay error reporting until all codegen operations have completed.
    #[arg(long)]
    #[serde(default)]
    pub keep_going: bool,
}

#[inline(always)]
fn r#true() -> bool {
    true
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
