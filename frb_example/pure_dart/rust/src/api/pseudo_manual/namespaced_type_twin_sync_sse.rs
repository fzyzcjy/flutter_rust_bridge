// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `namespaced_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub mod first_store {
    pub struct ModelTwinSyncSse {
        pub value: i32,
    }
}

pub mod second_store {
    pub struct ModelTwinSyncSse {
        pub value: String,
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn duplicate_named_models_twin_sync_sse() -> Vec<second_store::ModelTwinSyncSse> {
    vec![second_store::ModelTwinSyncSse {
        value: "second".to_owned(),
    }]
}
