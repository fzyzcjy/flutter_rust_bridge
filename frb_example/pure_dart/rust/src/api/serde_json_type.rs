#[derive(Debug, Clone)]
pub struct FeatureSerdeJsonTwinNormal {
    pub data: serde_json::Value,
}

pub fn handle_serde_json_value_twin_normal(
    val: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    Ok(val)
}

pub fn handle_option_serde_json_value_twin_normal(
    val: Option<serde_json::Value>,
) -> anyhow::Result<Option<serde_json::Value>> {
    Ok(val)
}

pub fn handle_vec_serde_json_value_twin_normal(
    val: Vec<serde_json::Value>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    Ok(val)
}

pub fn handle_map_serde_json_value_twin_normal(
    val: std::collections::HashMap<String, serde_json::Value>,
) -> anyhow::Result<std::collections::HashMap<String, serde_json::Value>> {
    Ok(val)
}

pub fn handle_nested_serde_json_value_twin_normal(
    wrapper: FeatureSerdeJsonTwinNormal,
) -> anyhow::Result<FeatureSerdeJsonTwinNormal> {
    Ok(wrapper)
}
