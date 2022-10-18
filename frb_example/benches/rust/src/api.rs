#![allow(unused_variables)]

use criterion::black_box;
use flutter_rust_bridge::SyncReturn;

/// benchmark time unit
#[allow(dead_code)] // some variants are used on dart side!
#[derive(Debug, Clone)]
pub enum Unit {
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

///////////////// benchmark samples //////////////////

pub fn handle_uuids(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(black_box(ids))
}

pub fn handle_uuids_convert_to_strings(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<String>> {
    Ok(black_box(
        ids.into_iter().map(|x| x.simple().to_string()).collect(),
    ))
}

pub fn handle_strings(strings: Vec<String>) -> anyhow::Result<Vec<String>> {
    Ok(black_box(strings))
}

pub fn handle_bool(input: bool) -> bool {
    black_box(input)
}
pub fn handle_u32(input: u32) -> u32 {
    black_box(input)
}
pub fn handle_u64(input: u64) -> u64 {
    black_box(input)
}
pub fn handle_i8(input: i8) -> i8 {
    black_box(input)
}
pub fn handle_i16(input: i16) -> i16 {
    black_box(input)
}
pub fn handle_i32(input: i32) -> i32 {
    black_box(input)
}
pub fn handle_i64(input: i64) -> i64 {
    black_box(input)
}
pub fn handle_f32(input: f32) -> f32 {
    black_box(input)
}
pub fn handle_f64(input: f64) -> f64 {
    black_box(input)
}
pub fn handle_string(input: String) -> String {
    black_box(input)
}

pub fn handle_sync_bool(input: bool) -> SyncReturn<bool> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_u32(input: u32) -> SyncReturn<u32> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_u64(input: u64) -> SyncReturn<u64> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_i8(input: i8) -> SyncReturn<i8> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_i16(input: i16) -> SyncReturn<i16> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_i32(input: i32) -> SyncReturn<i32> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_i64(input: i64) -> SyncReturn<i64> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_f32(input: f32) -> SyncReturn<f32> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_f64(input: f64) -> SyncReturn<f64> {
    SyncReturn(black_box(input))
}
pub fn handle_sync_string(input: String) -> SyncReturn<String> {
    SyncReturn(black_box(input))
}

pub fn dummy(unit: Unit) {}
