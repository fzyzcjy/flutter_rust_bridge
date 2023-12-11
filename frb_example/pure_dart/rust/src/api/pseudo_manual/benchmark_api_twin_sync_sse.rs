// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `benchmark_api.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(unused_variables)]

use std::hint::black_box;

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_void_twin_sync_sse() {}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_input_bytes_twin_sync_sse(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_output_bytes_twin_sync_sse(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}

pub struct BenchmarkBinaryTreeTwinSyncSse {
    pub name: String,
    pub left: Option<Box<BenchmarkBinaryTreeTwinSyncSse>>,
    pub right: Option<Box<BenchmarkBinaryTreeTwinSyncSse>>,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_input_twin_sync_sse(tree: BenchmarkBinaryTreeTwinSyncSse) {
    black_box(tree);
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_output_twin_sync_sse(
    depth: i32,
    name: String,
) -> BenchmarkBinaryTreeTwinSyncSse {
    create_tree(depth, &name)
}

fn create_tree(depth: i32, name: &str) -> BenchmarkBinaryTreeTwinSyncSse {
    if depth == 0 {
        BenchmarkBinaryTreeTwinSyncSse {
            name: name.to_owned(),
            left: None,
            right: None,
        }
    } else {
        BenchmarkBinaryTreeTwinSyncSse {
            name: name.to_owned(),
            left: Some(Box::new(create_tree(depth - 1, name))),
            right: Some(Box::new(create_tree(depth - 1, name))),
        }
    }
}

pub struct BenchmarkBlobTwinSyncSse {
    first: Vec<u8>,
    second: Vec<u8>,
    third: Vec<u8>,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_input_twin_sync_sse(blob: BenchmarkBlobTwinSyncSse) {
    black_box(blob);
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_output_twin_sync_sse(size: i32) -> BenchmarkBlobTwinSyncSse {
    let data = vec![0; size as _];
    BenchmarkBlobTwinSyncSse {
        first: data.clone(),
        second: data.clone(),
        third: data,
    }
}
