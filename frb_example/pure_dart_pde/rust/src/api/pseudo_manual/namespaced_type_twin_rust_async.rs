// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `namespaced_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

pub mod first_store {
    pub struct ModelTwinRustAsync {
        pub value: i32,
    }
}

pub mod second_store {
    pub struct ModelTwinRustAsync {
        pub value: String,
    }
}

pub async fn duplicate_named_models_twin_rust_async() -> Vec<second_store::ModelTwinRustAsync> {
    vec![second_store::ModelTwinRustAsync {
        value: "second".to_owned(),
    }]
}
