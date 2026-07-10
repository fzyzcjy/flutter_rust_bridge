// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `namespaced_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub mod first_store {
    pub struct ModelTwinSync {
        pub value: i32,
    }
}

pub mod second_store {
    pub struct ModelTwinSync {
        pub value: String,
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn duplicate_named_models_twin_sync() -> Vec<second_store::ModelTwinSync> {
    vec![second_store::ModelTwinSync {
        value: "second".to_owned(),
    }]
}
