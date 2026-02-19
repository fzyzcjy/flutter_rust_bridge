use std::collections::HashMap;

// Generic enum marked with ignore - should not generate Rust code
#[flutter_rust_bridge::frb(ignore)]
pub enum Change<T> {
    Created { data: T },
    Updated { id: String, data: T },
    Deleted { id: String },
}

#[flutter_rust_bridge::frb(non_opaque)]
struct BatchTwinNormal<T> {
    items: Vec<T>,
}

// Type alias directly using the ignored generic enum - should generate Dart code
pub type BatchTwinNormalChangeMap = BatchTwinNormal<Change<HashMap<String, String>>>;

pub fn func_batch_change_map(arg: BatchTwinNormalChangeMap) -> BatchTwinNormalChangeMap {
    arg
}
