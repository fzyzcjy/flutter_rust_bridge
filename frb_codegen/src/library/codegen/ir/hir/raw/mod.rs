use std::collections::HashMap;
use crate::utils::crate_name::CrateName;

pub(crate) struct HirRawPack {
    pub crates: HashMap<CrateName, syn::File>,
}
