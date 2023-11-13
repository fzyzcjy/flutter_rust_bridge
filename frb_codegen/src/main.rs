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
    /// hello world
    Generate {
        /// hello world
        #[arg(short, long)]
        list: bool,
    },
    /// hello world
    Create {
        /// hello world
        #[arg(short, long)]
        list: bool,
    },
    /// hello world
    Integrate {
        /// hello world
        #[arg(short, long)]
        list: bool,
    },
}
