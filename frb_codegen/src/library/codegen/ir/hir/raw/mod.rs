use std::collections::HashMap;

pub(crate) struct HirRawPack {
    pub crates: HashMap<String, syn::File>,
}
