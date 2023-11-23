#[derive(Debug, Clone)]
pub struct FeatureUuid {
    pub one: uuid::Uuid,
    pub many: Vec<uuid::Uuid>,
}

pub fn handle_uuid(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

pub fn handle_uuids(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

pub fn handle_nested_uuids(ids: FeatureUuid) -> anyhow::Result<FeatureUuid> {
    Ok(ids)
}
