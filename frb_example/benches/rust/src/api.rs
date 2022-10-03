#![allow(unused_variables)]

use crate::executor::{BenchErrorHandler, BenchExecutor, BenchHandler, Metrics};

lazy_static::lazy_static! {
  pub static ref FLUTTER_RUST_BRIDGE_ERROR_HANDLER: BenchErrorHandler = BenchErrorHandler::default();
  pub static ref FLUTTER_RUST_BRIDGE_HANDLER: BenchHandler = BenchHandler::new(BenchExecutor::new(FLUTTER_RUST_BRIDGE_ERROR_HANDLER.clone()), FLUTTER_RUST_BRIDGE_ERROR_HANDLER.clone());
}

#[derive(Debug, Clone)]
pub struct FeatureUuid {
    pub one: uuid::Uuid,
    pub many: Vec<uuid::Uuid>,
}

pub fn rust_metrics() -> anyhow::Result<Vec<String>> {
    Ok(FLUTTER_RUST_BRIDGE_HANDLER
        .metrics()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>())
}

pub fn handle_uuid(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

pub fn handle_uuids(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

pub fn handle_uuids_convert_to_strings(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<String>> {
    Ok(ids
        .into_iter()
        .map(|x| x.hyphenated().to_string())
        .collect())
}

pub fn handle_nested_uuids(ids: FeatureUuid) -> anyhow::Result<FeatureUuid> {
    Ok(ids)
}

pub fn handle_strings(strings: Vec<String>) -> anyhow::Result<Vec<String>> {
    Ok(strings)
}

pub fn send_i64(value: i64) -> i64 {
    value
}
