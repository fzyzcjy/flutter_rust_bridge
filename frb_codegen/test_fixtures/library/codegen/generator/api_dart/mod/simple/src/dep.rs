use std::collections::{HashMap, HashSet};

pub struct Simple {
    val: u32,
    values: Vec<String>,
}

#[frb(dart_collection_deep_equality)]
pub struct DeepCollectionStruct {
    values: Vec<String>,
    map: HashMap<String, String>,
    set: HashSet<String>,
    optional_values: Option<Vec<String>>,
}
