//! A thin command line interface. Please avoid putting logic here
//! (instead be in `lib.rs` and so on)

mod cli;

use clap::{Args, Parser, Subcommand, FromArgMatches};
use itertools::Itertools;
use log::debug;
use lib_flutter_rust_bridge_codegen::*;
use crate::cli::commands::{Cli, Commands};

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

#[cfg(test)]
mod tests {
    use clap::Parser;
    use lib_flutter_rust_bridge_codegen::codegen;
    use crate::{Cli, Commands};

    #[test]
    fn test_args_to_codegen_config() {
        fn args_to_codegen_config(args: Vec<&'static str>) -> codegen::Config {
            let cli = Cli::parse_from(args);
            let args = match cli.command {
                Commands::Generate(args) => args,
                _ => panic!()
            };
            args.into_config()
        }

        assert_eq!(args_to_codegen_config(vec!["", "generate"]).dart3, Some(true));
        assert_eq!(args_to_codegen_config(vec!["", "generate", "--no-dart3"]).dart3, Some(false));
    }
}
