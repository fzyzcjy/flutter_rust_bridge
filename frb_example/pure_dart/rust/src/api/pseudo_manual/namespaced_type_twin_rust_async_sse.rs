// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `namespaced_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub mod first_store {
    pub struct ModelTwinRustAsyncSse {
        pub value: i32,
    }
}

pub mod second_store {
    pub struct ModelTwinRustAsyncSse {
        pub value: String,
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn duplicate_named_models_twin_rust_async_sse() -> Vec<second_store::ModelTwinRustAsyncSse>
{
    vec![second_store::ModelTwinRustAsyncSse {
        value: "second".to_owned(),
    }]
}
