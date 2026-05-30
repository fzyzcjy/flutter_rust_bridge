// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

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

#[derive(Debug, Clone)]
pub struct FeatureBigNumberTwinNormal {
    pub signed: num_bigint::BigInt,
    pub unsigned: num_bigint::BigUint,
    pub reexported_signed: bigdecimal::num_bigint::BigInt,
    pub reexported_unsigned: bigdecimal::num_bigint::BigUint,
    pub decimal: rust_decimal::Decimal,
    pub big_decimal: bigdecimal::BigDecimal,
}

pub fn handle_big_number_types_twin_normal(
    input: FeatureBigNumberTwinNormal,
) -> anyhow::Result<FeatureBigNumberTwinNormal> {
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
