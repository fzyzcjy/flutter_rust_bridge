use clap::{Args, Parser, Subcommand};
use lib_flutter_rust_bridge_codegen::*;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate(args) => codegen::generate(&codegen::Config {})?,
        Commands::Create(args) => integration::create(&args.name)?,
        Commands::Integrate(_) => integration::integrate()?,
    }
    Ok(())
}

// The name `Cli`, `Commands` come from https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute the main code generator
    Generate(GenerateCommandArgs),

    /// Create a new Flutter + Rust project
    Create(CreateCommandArgs),

    /// Integrate Rust into existing Flutter project
    Integrate(IntegrateCommandArgs),
}

#[derive(Debug, Args)]
struct GenerateCommandArgs {
    /// hello world
    #[arg(short, long)]
    list: bool,
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
