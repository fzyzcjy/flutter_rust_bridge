use clap::{Parser, Subcommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    todo!()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute main code generator
    Generate {
        /// hello world
        #[arg(short, long)]
        list: bool,
    },

    /// Create a new Flutter + Rust project
    Create {
        /// hello world
        #[arg(short, long)]
        list: bool,
    },

    /// Integrate Rust into existing Flutter project
    Integrate {
        /// hello world
        #[arg(short, long)]
        list: bool,
    },
}
