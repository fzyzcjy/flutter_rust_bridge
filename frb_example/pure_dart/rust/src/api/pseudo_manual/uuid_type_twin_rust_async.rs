// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `uuid_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[derive(Debug, Clone)]
pub struct FeatureUuidTwinRustAsync {
    pub one: uuid::Uuid,
    // pub many: Vec<uuid::Uuid>,
}

pub async fn handle_uuid_twin_rust_async(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

pub async fn handle_uuids_twin_rust_async(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

pub async fn handle_nested_uuids_twin_rust_async(
    ids: FeatureUuidTwinRustAsync,
) -> anyhow::Result<FeatureUuidTwinRustAsync> {
    Ok(ids)
}
