use std::fs;
use std::path::Path;

pub fn mod_from_rust_path(code_path: &str, crate_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap()
        .with_extension("")
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('/', "::")
}

pub fn with_changed_file(
    path: &str,
    append_content: &str,
    generate_c_from_rust_func: impl FnOnce() -> anyhow::Result<()>,
    generate_dart_from_c_func: impl FnOnce() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let content_original = fs::read_to_string(&path)?;
    fs::write(&path, content_original.clone() + append_content)?;

    generate_c_from_rust_func()?;
    generate_dart_from_c_func()?;

    Ok(fs::write(&path, content_original)?)
}

pub fn find_all_duplicates(mut vec: Vec<String>) -> anyhow::Result<()> {
    let mut conflicts = vec![];
    if vec.is_empty() {
        return Ok(());
    }
    vec.sort();

    for i in 0..(vec.len() - 1) {
        if vec[i] == vec[i + 1] && conflicts.binary_search(&vec[i]).is_err() {
            conflicts.push(vec[i].clone());
        }
    }

    if !conflicts.is_empty() {
        let mut conflict_symbols = conflicts
            .iter()
            .map(|s| s.clone() + ",")
            .collect::<String>();
        conflict_symbols.pop();

        let (symbol_str, verb_str) = if conflicts.len() == 1 {
            ("symbol", "has")
        } else {
            ("symbols", "have")
        };
        panic!(
            "{} [{}] {} already been defined",
            symbol_str, conflict_symbols, verb_str
        );
    }

    Ok(())
}

/// check api defined by users, if no duplicates, then generate all symbols (api function name)
/// ,including those generated implicitily by frb
pub fn get_symbols_if_no_duplicates(
    configs: &Vec<crate::Opts>,
) -> Result<Vec<String>, anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for (i, config) in configs.iter().enumerate() {
        let raw_ir_file = config.get_ir_file();

        // for checking explicit api duplication
        explicit_raw_symbols.extend(raw_ir_file.funcs.iter().map(|f| f.name.clone()));

        // for avoiding redundant generation in dart
        all_symbols.extend(raw_ir_file.get_all_symbols(config, i + 1));
    }
    find_all_duplicates(explicit_raw_symbols)?;
    Ok(all_symbols)
}
