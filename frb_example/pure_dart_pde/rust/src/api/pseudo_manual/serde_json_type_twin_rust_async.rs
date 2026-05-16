// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `serde_json_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#[derive(Debug, Clone)]
pub struct FeatureSerdeJsonTwinRustAsync {
    pub data: serde_json::Value,
}

pub async fn handle_serde_json_value_twin_rust_async(
    val: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    Ok(val)
}

pub async fn handle_option_serde_json_value_twin_rust_async(
    val: Option<serde_json::Value>,
) -> anyhow::Result<Option<serde_json::Value>> {
    Ok(val)
}

pub async fn handle_vec_serde_json_value_twin_rust_async(
    val: Vec<serde_json::Value>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    Ok(val)
}

pub async fn handle_map_serde_json_value_twin_rust_async(
    val: std::collections::HashMap<String, serde_json::Value>,
) -> anyhow::Result<std::collections::HashMap<String, serde_json::Value>> {
    Ok(val)
}

pub async fn handle_nested_serde_json_value_twin_rust_async(
    wrapper: FeatureSerdeJsonTwinRustAsync,
) -> anyhow::Result<FeatureSerdeJsonTwinRustAsync> {
    Ok(wrapper)
}
