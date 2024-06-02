// This file is named `crates` not `crate`, because the latter is a Rust keyword

use crate::utils::crate_name::CrateName;

#[derive(Debug, Clone)]
pub struct HirRawCrate {
    pub(crate) name: CrateName,
    pub(crate) syn_file: syn::File,
}
