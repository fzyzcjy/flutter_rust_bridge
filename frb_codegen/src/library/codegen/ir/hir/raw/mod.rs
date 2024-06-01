use std::collections::HashMap;

pub(crate) struct HirRawPack<'a> {
    pub crates: HashMap<String, syn::File>,
}
