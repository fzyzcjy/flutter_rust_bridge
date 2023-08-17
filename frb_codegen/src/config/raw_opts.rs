use clap::{ArgAction, Parser, ValueEnum};
use serde::Deserialize;

// Raw configs, which is mainly given by the user of flutter_rust_bridge
#[derive(Parser, Debug, PartialEq, Eq, Deserialize, Default)]
#[command(version)]
#[command(override_usage(
"flutter_rust_bridge_codegen [OPTIONS] --rust-input <RUST_INPUT>... --dart-output <DART_OUTPUT>...
       flutter_rust_bridge_codegen [CONFIG_FILE]"
))]
pub struct RawOpts {
    /// Path of input Rust code
    #[arg(short, long, required_unless_present = "config_file", num_args = 1..)]
    pub rust_input: Vec<String>,

    /// Path of output generated Dart code
    #[arg(short, long, required_unless_present = "config_file", num_args = 1..)]
    pub dart_output: Vec<String>,

    /// Path to a YAML config file.
    ///
    /// If present, other options and flags will be ignored.
    /// Accepts the same options as the CLI, but uses snake_case keys.
    #[serde(skip)]
    pub config_file: Option<String>,

    /// If provided, generated Dart declaration code to this separate file
    #[arg(long)]
    pub dart_decl_output: Option<String>,

    /// Output path (including file name) of generated C header, each field corresponding to that of --rust-input.
    #[arg(short, long)]
    pub c_output: Option<Vec<String>>,

    /// Extra output path (excluding file name) of generated C header
    #[arg(short, long)]
    pub extra_c_output_path: Option<Vec<String>>,

    /// Crate directory for your Rust project
    #[arg(long, num_args = 1..)]
    pub rust_crate_dir: Option<Vec<String>>,

    /// Output path of generated Rust code
    #[arg(long, num_args = 1..)]
    pub rust_output: Option<Vec<String>>,

    /// Generated class name
    #[arg(long, num_args = 1..)]
    pub class_name: Option<Vec<String>>,

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
    pub skip_add_mod_to_lib: bool,

    /// Path to the installed LLVM
    #[arg(long, num_args = 1..)]
    pub llvm_path: Option<Vec<String>>,

    /// LLVM compiler opts
    #[arg(long)]
    pub llvm_compiler_opts: Option<String>,

    /// Path to root of Dart project, otherwise inferred from --dart-output
    #[arg(long, num_args = 1..)]
    pub dart_root: Option<Vec<String>>,

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
    pub dump: Option<Vec<Dump>>,

    /// Disable language features introduced in Dart 3.
    #[arg(long = "no-dart3", action(ArgAction::SetFalse))]
    #[serde(default = "r#true")]
    pub dart3: bool,

    /// If set, the program will delay error reporting until all codegen operations have completed.
    #[arg(long)]
    #[serde(default)]
    pub keep_going: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, ValueEnum, enum_iterator::Sequence)]
#[serde(rename_all = "snake_case")]
pub enum Dump {
    Config,
    Ir,
}

#[inline(always)]
fn r#true() -> bool {
    true
}
