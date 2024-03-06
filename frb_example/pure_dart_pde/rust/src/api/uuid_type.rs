// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#[derive(Debug, Clone)]
pub struct FeatureUuidTwinNormal {
    pub one: uuid::Uuid,
    // pub many: Vec<uuid::Uuid>,
}

pub fn handle_uuid_twin_normal(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

pub fn handle_uuids_twin_normal(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

pub fn handle_nested_uuids_twin_normal(
    ids: FeatureUuidTwinNormal,
) -> anyhow::Result<FeatureUuidTwinNormal> {
    Ok(ids)
}
