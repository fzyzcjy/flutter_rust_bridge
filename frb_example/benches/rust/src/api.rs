#![allow(unused_variables)]

use crate::executor::{BenchExecutor, BenchHandler, Benchmark};
use flutter_rust_bridge::{frb, handler::ReportDartErrorHandler, SyncReturn};

lazy_static::lazy_static! {
  pub static ref FLUTTER_RUST_BRIDGE_ERROR_HANDLER: ReportDartErrorHandler = ReportDartErrorHandler {};
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
        .0
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
        panic!("strings must not be empty");
    }
    // do a simple op to avoid code being optimized away (you'd end up with a surprisingly fast `0ns`)
    strings.reverse();
    Ok(strings)
}

pub fn handle_bool(input: bool) -> bool {
    !input
}
pub fn handle_u32(input: u32) -> u32 {
    input + 58
}
pub fn handle_u64(input: u64) -> u64 {
    input + 58
}
pub fn handle_i8(input: i8) -> i8 {
    input + 58
}
pub fn handle_i16(input: i16) -> i16 {
    input + 58
}
pub fn handle_i32(input: i32) -> i32 {
    input + 58
}
pub fn handle_i64(input: i64) -> i64 {
    input + 58
}
pub fn handle_f32(input: f32) -> f32 {
    input + 58.
}
pub fn handle_f64(input: f64) -> f64 {
    input + 58.
}
pub fn handle_string(input: String) -> String {
    input.chars().rev().collect::<String>()
}

pub fn handle_sync_bool(input: bool) -> SyncReturn<bool> {
    SyncReturn(!input)
}
pub fn handle_sync_u32(input: u32) -> SyncReturn<u32> {
    SyncReturn(input + 58)
}
pub fn handle_sync_u64(input: u64) -> SyncReturn<u64> {
    SyncReturn(input + 58)
}
pub fn handle_sync_i8(input: i8) -> SyncReturn<i8> {
    SyncReturn(input + 58)
}
pub fn handle_sync_i16(input: i16) -> SyncReturn<i16> {
    SyncReturn(input + 58)
}
pub fn handle_sync_i32(input: i32) -> SyncReturn<i32> {
    SyncReturn(input + 58)
}
pub fn handle_sync_i64(input: i64) -> SyncReturn<i64> {
    SyncReturn(input + 58)
}
pub fn handle_sync_f32(input: f32) -> SyncReturn<f32> {
    SyncReturn(input + 58.)
}
pub fn handle_sync_f64(input: f64) -> SyncReturn<f64> {
    SyncReturn(input + 58.)
}
pub fn handle_sync_string(input: String) -> SyncReturn<String> {
    SyncReturn(input.chars().rev().collect::<String>())
}
