// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `serde_json_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[derive(Debug, Clone)]
pub struct FeatureSerdeJsonTwinSyncSse {
    pub data: serde_json::Value,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_serde_json_value_twin_sync_sse(
    val: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_option_serde_json_value_twin_sync_sse(
    val: Option<serde_json::Value>,
) -> anyhow::Result<Option<serde_json::Value>> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_vec_serde_json_value_twin_sync_sse(
    val: Vec<serde_json::Value>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_map_serde_json_value_twin_sync_sse(
    val: std::collections::HashMap<String, serde_json::Value>,
) -> anyhow::Result<std::collections::HashMap<String, serde_json::Value>> {
    Ok(val)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_nested_serde_json_value_twin_sync_sse(
    wrapper: FeatureSerdeJsonTwinSyncSse,
) -> anyhow::Result<FeatureSerdeJsonTwinSyncSse> {
    Ok(wrapper)
}
