use crate::config::raw_opts::RawOpts;
use clap::Parser;

impl RawOpts {
    /// Parses options from arguments, or from a config file if no arguments are given.
    /// Terminates the program if argument validation fails.
    pub fn try_parse_args_or_yaml() -> anyhow::Result<Self> {
        // #[cfg(not(feature = "serde"))]
        // {
        //     Self::try_parse().map_err(|err| err.exit())
        // }

        // #[cfg(feature = "serde")]
        // {
        use anyhow::Context;
        use std::fs;

        let has_args = std::env::args_os().len() > 1;
        let err = match Self::try_parse() {
            Ok(opts) => return Ok(opts),
            Err(err) => err,
        };

        if has_args {
            err.exit();
        }

        from_configuration_files(...)

        _ = err.print();
        eprintln!("Hint: To call flutter_rust_bridge_codegen with no arguments, {hint}");
        std::process::exit(1)
        // }
    }
}
