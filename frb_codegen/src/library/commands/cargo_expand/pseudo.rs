use anyhow::ensure;
use log::warn;
use std::fs;
use std::path::Path;

pub(super) fn run(rust_crate_dir: &Path) -> anyhow::Result<syn::File> {
    warn!(
        "Skip cargo-expand on {rust_crate_dir:?}, \
         because cargo is already running and would block cargo-expand. \
         This might cause errors if your api contains macros or complex mods."
    );

    parse_file(&rust_crate_dir.join("src/lib.rs"))
}

fn parse_file(path: &Path) -> anyhow::Result<syn::File> {
    let code = fs::read_to_string(&path)?;
    let mut file = syn::parse_file(&code)?;
    modify_file(&mut file)?;
    Ok(file)
}

fn modify_file(file: &mut syn::File) -> anyhow::Result<()> {
    for item in file.items.iter_mut() {
        if let syn::Item::Mod(item_mod) = item {
            if item_mod.content.is_none() {
                modify_mod(item_mod)?;
            }
        }
    }
    Ok(())
}

fn modify_mod(item_mod: &mut syn::ItemMod) -> anyhow::Result<()> {
    ensure!(item_mod.content.is_none() && item_mod.semi.is_some());

    let mod_syn_file = parse_file(TODO)?;

    item_mod.semi = None;
    item_mod.content = Some(syn::token::Brace::default(), mod_syn_file.items);

    Ok(())
}
