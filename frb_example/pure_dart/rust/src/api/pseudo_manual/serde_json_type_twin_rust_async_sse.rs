// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `serde_json_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[derive(Debug, Clone)]
pub struct FeatureSerdeJsonTwinRustAsyncSse {
    pub data: serde_json::Value,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_serde_json_value_twin_rust_async_sse(
    val: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_option_serde_json_value_twin_rust_async_sse(
    val: Option<serde_json::Value>,
) -> anyhow::Result<Option<serde_json::Value>> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_vec_serde_json_value_twin_rust_async_sse(
    val: Vec<serde_json::Value>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_map_serde_json_value_twin_rust_async_sse(
    val: std::collections::HashMap<String, serde_json::Value>,
) -> anyhow::Result<std::collections::HashMap<String, serde_json::Value>> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_nested_serde_json_value_twin_rust_async_sse(
    wrapper: FeatureSerdeJsonTwinRustAsyncSse,
) -> anyhow::Result<FeatureSerdeJsonTwinRustAsyncSse> {
    Ok(wrapper)
}
