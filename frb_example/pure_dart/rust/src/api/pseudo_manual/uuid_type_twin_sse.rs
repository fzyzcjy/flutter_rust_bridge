// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `uuid_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[derive(Debug, Clone)]
pub struct FeatureUuidTwinSse {
    pub one: uuid::Uuid,
    // pub many: Vec<uuid::Uuid>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_uuid_twin_sse(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_uuids_twin_sse(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_nested_uuids_twin_sse(ids: FeatureUuidTwinSse) -> anyhow::Result<FeatureUuidTwinSse> {
    Ok(ids)
}
