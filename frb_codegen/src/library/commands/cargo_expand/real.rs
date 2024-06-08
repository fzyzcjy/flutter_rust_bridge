use crate::codegen::dumper::Dumper;
use crate::codegen::ConfigDumpContent;
use crate::command_args;
use crate::library::commands::command_runner::execute_command;
use crate::utils::crate_name::CrateName;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{debug, info};
use regex::{Captures, Regex};
use std::borrow::Cow;
use std::env;
use std::path::Path;

pub(super) fn run(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    dumper: &Dumper,
) -> Result<syn::File> {
    let text = run_with_frb_aware(rust_crate_dir, interest_crate_name)?;
    (dumper.with_content(ConfigDumpContent::Source)).dump_str("cargo_expand.rs", &text)?;
    Ok(syn::parse_file(&text)?)
}

fn run_with_frb_aware(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
) -> Result<String> {
    Ok(decode_macro_frb_encoded_comments(&run_raw(
        rust_crate_dir,
        interest_crate_name,
        "--cfg frb_expand",
        true,
    )?)
    .into_owned())
}

/// Decode `#[doc = "frb_encoded(...)"]`, usually produced as a side-effect of cargo-expand.
fn decode_macro_frb_encoded_comments(code: &str) -> Cow<str> {
    lazy_static! {
        static ref PATTERN: Regex =
            Regex::new(r##"#\[doc =[\s\n]*"frb_encoded\(([\s\S]*?)\)"\]"##).unwrap();
    }

    PATTERN.replace_all(code, |captures: &Captures| {
        let hex_str = captures.get(1).unwrap().as_str();
        let decoded_str = String::from_utf8(hex::decode(hex_str).unwrap()).unwrap();
        decoded_str.to_string()
    })
}

#[allow(clippy::vec_init_then_push)]
fn run_raw(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    extra_rustflags: &str,
    allow_auto_install: bool,
) -> Result<String> {
    // let _pb = simple_progress("Run cargo-expand".to_owned(), 1);
    debug!("Running cargo expand in '{rust_crate_dir:?}'");

    let args_choosing_crate = if let Some(interest_crate_name) = interest_crate_name {
        vec!["-p", interest_crate_name.raw()]
    } else {
        vec![]
    };

    let args = command_args!(
        "expand",
        "--lib",
        "--theme=none",
        "--ugly",
        *args_choosing_crate,
    );
    let extra_env = [(
        "RUSTFLAGS".to_owned(),
        env::var("RUSTFLAGS").map(|x| x + " ").unwrap_or_default() + extra_rustflags,
    )]
    .into();

    let output = execute_command("cargo", &args, Some(rust_crate_dir), Some(extra_env))
        .with_context(|| format!("Could not expand rust code at path {rust_crate_dir:?}"))?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if stdout.is_empty() {
        if stderr.contains("no such command: `expand`") && allow_auto_install {
            info!("Cargo expand is not installed. Automatically install and re-run.");
            install_cargo_expand()?;
            return run_raw(rust_crate_dir, interest_crate_name, extra_rustflags, false);
        }
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("cargo expand returned empty output");
        // frb-coverage:ignore-end
    }

    Ok(stdout.lines().skip(1).join("\n"))
}

fn install_cargo_expand() -> Result<()> {
    execute_command(
        "cargo",
        &vec!["install".into(), "cargo-expand".into()],
        None,
        None,
    )?;
    Ok(())
}

// fn extract_module(raw_expanded: &str, module: Option<String>) -> Result<String> {
//     if let Some(module) = module {
//         let mut spaces = 0;
//         let mut expanded = raw_expanded;
//
//         for module in module.split("::") {
//             if expanded.contains(&format!("mod {module} {{}}")) {
//                 return Ok("".to_owned());
//             }
//
//             let indent = " ".repeat(spaces);
//             let searched = regex::Regex::new(&format!(
//                 "(?m)^{indent}(?:pub(?:\\([^\\)]+\\))?\\s+)?mod {module} \\{{$"
//             ))
//             .unwrap();
//             let start = match searched.find(expanded) {
//                 Some(m) => m.end() + 1,
//                 // #1830
//                 None => return Ok("".to_owned()),
//             };
//             let end = expanded[start..]
//                 .find(&format!("\n{}}}", indent))
//                 .map(|n| n + start)
//                 .unwrap_or(expanded.len());
//             spaces += 4;
//             expanded = &expanded[start..end];
//         }
//         return Ok(expanded.to_owned());
//     }
//     Ok(raw_expanded.to_owned())
// }
//
// #[cfg(test)]
// mod tests {
//     use super::extract_module;
//
//     #[test]
//     pub fn test_extract_module_simple() {
//         let src = "mod module_1 {
//     // code 1
// }
// mod module_2 {
//     // code 2
// }";
//         let extracted = extract_module(src, Some(String::from("module_1"))).unwrap();
//         assert_eq!(String::from("    // code 1"), extracted);
//         let extracted = extract_module(src, Some(String::from("module_2"))).unwrap();
//         assert_eq!(String::from("    // code 2"), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_submod() {
//         let src = "mod module {
//     mod submodule {
//         // sub code
//     }
// }";
//         let extracted = extract_module(src, Some(String::from("module::submodule"))).unwrap();
//         assert_eq!(String::from("        // sub code"), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_empty_submod() {
//         let src = "pub mod api {
//     // some code
// }
// mod another {}";
//         let extracted = extract_module(src, Some(String::from("another"))).unwrap();
//         assert_eq!(String::from(""), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_with_prefix() {
//         let src = "pub mod parent {
//     mod another {
//         // some code
//     }
// }";
//         let extracted = extract_module(src, Some(String::from("another"))).unwrap();
//         assert_eq!(String::from(""), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_with_same_name() {
//         let src = "pub mod parent {
//     mod another {
//         // some code
//     }
// }
// pub(self) mod another {
//     // 12345
// }                                                                                                                                      ";
//         let extracted = extract_module(src, Some(String::from("another"))).unwrap();
//         assert_eq!(String::from("    // 12345"), extracted);
//     }
// }
