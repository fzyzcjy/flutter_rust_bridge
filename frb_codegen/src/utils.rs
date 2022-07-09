use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::path::Path;
use crate::ir::IrTypeStructRef;
use crate::ir::{IrFunc, IrFile, IrTypeBoxed};
use crate::ir::IrType::{Boxed, StructRef};

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

pub fn with_changed_file<F: FnOnce() -> anyhow::Result<()>>(
    path: &str,
    append_content: &str,
    f: F,
) -> anyhow::Result<()> {
    let content_original = fs::read_to_string(&path)?;
    fs::write(&path, content_original.clone() + append_content)?;

    f()?;

    Ok(fs::write(&path, content_original)?)
}

pub fn find_all_duplicates<T>(iter: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    iter.iter()
        .filter(|x| !uniq.insert(*x))
        .cloned()
        .collect::<Vec<_>>()
}

/// check api defined by users, if no duplicates, then generate all symbols (api function name),
/// including those generated implicitily by frb
pub fn get_symbols_if_no_duplicates(configs: &[crate::Opts]) -> Result<Vec<String>, anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for config in configs {
        let raw_ir_file = config.get_ir_file();

        // for checking explicit api duplication
        explicit_raw_symbols.extend(raw_ir_file.funcs.iter().map(|f| f.name.clone()));

        // for avoiding redundant generation in dart
        all_symbols.extend(raw_ir_file.get_all_symbols(config));
    }
    let duplicates = find_all_duplicates(&explicit_raw_symbols);
    if !duplicates.is_empty() {
        let duplicated_symbols = duplicates.join(",");

        let (symbol_str, verb_str) = if duplicates.len() == 1 {
            ("symbol", "has")
        } else {
            ("symbols", "have")
        };
        panic!(
            "{} [{}] {} already been defined",
            symbol_str, duplicated_symbols, verb_str
        );
    }

    Ok(all_symbols)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BlockIndex(pub usize);

impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(0);
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Methods and static methods helper functions

pub(crate) const STATIC_METHOD_MARKER: &str = "__static_method___";
pub(crate) const METHOD_MARKER: &str = "__method";

pub fn is_method(s: &String) -> bool {
    s.ends_with(METHOD_MARKER)
}

pub fn is_static_method(s: &String) -> bool {
    s.contains(STATIC_METHOD_MARKER)
}

// Tests if the function is `f` is a static method for struct with name `struct_name`
pub fn is_static_method_for_struct(f: &&IrFunc, struct_name: &String) -> bool {
    f.name.contains(STATIC_METHOD_MARKER) && f.name.split("___").last().unwrap() == struct_name
}

pub fn has_methods(struct_name: &String, ir_file: &IrFile) -> bool {
    ir_file
        .funcs
        .iter()
        .find(|f| {
            is_method_for_struct(f, &struct_name)
                || is_static_method_for_struct(f, &struct_name)
        })
        .is_some()
}

// Tests if the function in `f` is a method for struct with name `struct_name`
pub fn is_method_for_struct(f: &&IrFunc, struct_name: &String) -> bool {
    f.name.ends_with(METHOD_MARKER)
        && if let Boxed(IrTypeBoxed {
            exist_in_real_api: _,
            inner,
        }) = &f.inputs[0].ty
        {
            if let StructRef(IrTypeStructRef { name, freezed: _ }) = &**inner {
                *name == *struct_name
            } else {
                false
            }
        } else {
            false
        }
}

// Returns the name of the struct that this method is for
pub fn static_method_return_struct_name(s: &String) -> String {
    s.split(STATIC_METHOD_MARKER).last().unwrap().to_string()
}

// Returns the name of method itself
pub fn static_method_return_method_name(s: &String) -> String {
    s.split(STATIC_METHOD_MARKER).next().unwrap().to_string()
}

// Clears the method marker from this method name
pub fn clear_method_marker(s: &String) -> String {
    s.replace(METHOD_MARKER, "")
}