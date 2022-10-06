#![allow(unused_variables)]

use crate::executor::{BenchErrorHandler, BenchExecutor, BenchHandler, Metrics};
use flutter_rust_bridge::frb;

lazy_static::lazy_static! {
  pub static ref FLUTTER_RUST_BRIDGE_ERROR_HANDLER: BenchErrorHandler = BenchErrorHandler::default();
  pub static ref FLUTTER_RUST_BRIDGE_HANDLER: BenchHandler = BenchHandler::new(BenchExecutor::new(*FLUTTER_RUST_BRIDGE_ERROR_HANDLER), *FLUTTER_RUST_BRIDGE_ERROR_HANDLER);
}

/// benchmark time unit
#[allow(dead_code)] // some variants are used on dart side!
#[derive(Debug, Clone)]
pub enum Unit {
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

/// metric used for [continuous-benchmark](https://github.com/marketplace/actions/continuous-benchmark) worflow
#[frb]
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: Option<u64>,
    /// benchmarking time unit is platform-dependent
    pub unit: Unit,
    /// allows to provide extra context for benchmarked wired function call
    /// e.g. `reverse 1,000 uuids`
    ///
    /// `non-final` to allow setting Rust metric `extra` from Dart
    ///
    /// TODO: once `hint` can be sent and consumed on Rust side, this field can be `final`
    #[frb(non_final)]
    pub extra: Option<String>,
}

/// exposes rust benchmark metrics for dart to collect
pub fn rust_metrics() -> anyhow::Result<Vec<Metric>> {
    Ok(FLUTTER_RUST_BRIDGE_HANDLER
        .metrics()
        .into_iter()
        .collect::<Vec<_>>())
}

///////////////// benchmark samples //////////////////

pub fn handle_uuids(mut ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    if ids.is_empty() {
        panic!("uuids must not be empty");
    }
    // do a simple op to avoid code being optimized away (you'd end up with a surprisingly fast `0ns`)
    ids.reverse();
    Ok(ids)
}

pub fn handle_uuids_convert_to_strings(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<String>> {
    Ok(ids
        .into_iter()
        .map(|x| x.hyphenated().to_string())
        .collect())
}

pub fn handle_strings(mut strings: Vec<String>) -> anyhow::Result<Vec<String>> {
    if strings.is_empty() {
        panic!("uuids must not be empty");
    }
    // do a simple op to avoid code being optimized away (you'd end up with a surprisingly fast `0ns`)
    strings.reverse();
    Ok(strings)
}
