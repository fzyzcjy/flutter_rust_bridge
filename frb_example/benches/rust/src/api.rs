#![allow(unused_variables)]

use crate::executor::{BenchErrorHandler, BenchExecutor, BenchHandler, Metrics};
use flutter_rust_bridge::frb;

lazy_static::lazy_static! {
  pub static ref FLUTTER_RUST_BRIDGE_ERROR_HANDLER: BenchErrorHandler = BenchErrorHandler::default();
  pub static ref FLUTTER_RUST_BRIDGE_HANDLER: BenchHandler = BenchHandler::new(BenchExecutor::new(*FLUTTER_RUST_BRIDGE_ERROR_HANDLER), *FLUTTER_RUST_BRIDGE_ERROR_HANDLER);
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Unit {
    Microseconds,
    Nanoseconds,
}

/// metric used for continuous-benchmark worflow
#[frb]
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: Option<u64>,
    pub unit: Unit,
    // non-final to allow setting Rust metric extra from Dart
    #[frb(non_final)]
    pub extra: Option<String>,
}

pub fn rust_metrics() -> anyhow::Result<Vec<Metric>> {
    Ok(FLUTTER_RUST_BRIDGE_HANDLER
        .metrics()
        .into_iter()
        .collect::<Vec<_>>())
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

pub fn handle_strings(strings: Vec<String>) -> anyhow::Result<Vec<String>> {
    Ok(strings)
}
