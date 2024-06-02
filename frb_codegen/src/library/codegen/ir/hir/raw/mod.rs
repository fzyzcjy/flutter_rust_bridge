use crate::utils::crate_name::CrateName;
use std::collections::HashMap;

pub(crate) struct HirRawPack {
    pub crates: HashMap<CrateName, syn::File>,
}
