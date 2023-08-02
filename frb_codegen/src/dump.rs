#![cfg(feature = "serde")]

use std::collections::HashMap;

use crate::config::opts::Opts;
use crate::config::raw_opts::Dump;
use crate::parser::ParserResult;
use enum_iterator::all;

pub fn dump_multi(all_configs: &[Opts], dump: Vec<Dump>) -> anyhow::Result<()> {
    let dump = if dump.is_empty() {
        all().collect()
    } else {
        dump
    };
    let data = all_configs
        .iter()
        .map(|config| {
            let mut data = HashMap::new();
            for request in &dump {
                match request {
                    Dump::Config => data.insert("config", serde_yaml::to_value(config)?),
                    Dump::Ir => data.insert(
                        "ir",
                        config
                            .get_ir_file(all_configs)
                            .map(serde_yaml::to_value)??,
                    ),
                };
            }
            Ok(data)
        })
        .collect::<ParserResult<Vec<_>>>()?;
    let data = serde_yaml::to_string(&data)?;
    println!("{data}");
    Ok(())
}
