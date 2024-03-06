// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `uuid_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#[derive(Debug, Clone)]
pub struct FeatureUuidTwinSync {
    pub one: uuid::Uuid,
    // pub many: Vec<uuid::Uuid>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_uuid_twin_sync(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_uuids_twin_sync(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_nested_uuids_twin_sync(
    ids: FeatureUuidTwinSync,
) -> anyhow::Result<FeatureUuidTwinSync> {
    Ok(ids)
}
