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

#[derive(Debug, Clone)]
pub struct FeatureBigNumberTwinSyncSse {
    pub signed: num_bigint::BigInt,
    pub unsigned: num_bigint::BigUint,
    pub reexported_signed: bigdecimal::num_bigint::BigInt,
    pub reexported_unsigned: bigdecimal::num_bigint::BigUint,
    pub decimal: rust_decimal::Decimal,
    pub big_decimal: bigdecimal::BigDecimal,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_big_number_types_twin_sync_sse(
    input: FeatureBigNumberTwinSyncSse,
) -> anyhow::Result<FeatureBigNumberTwinSyncSse> {
    assert_eq!(
        input.signed.to_string(),
        "-170141183460469231731687303715884105728"
    );
    assert_eq!(
        input.unsigned.to_string(),
        "340282366920938463463374607431768211455"
    );
    assert_eq!(
        input.reexported_signed.to_string(),
        "-123456789123456789123456789"
    );
    assert_eq!(
        input.reexported_unsigned.to_string(),
        "123456789123456789123456789"
    );
    assert_eq!(input.decimal.to_string(), "123456789.123456789");
    assert_eq!(input.big_decimal.to_string(), "-987654321.987654321");
    Ok(input)
}
