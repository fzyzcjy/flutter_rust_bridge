// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `benchmark_api.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(unused_variables)]

use std::hint::black_box;

#[flutter_rust_bridge::frb(serialize)]
pub fn benchmark_void_twin_sse() {}

#[flutter_rust_bridge::frb(serialize)]
pub fn benchmark_input_bytes_twin_sse(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn benchmark_output_bytes_twin_sse(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}

pub struct BenchmarkBinaryTreeTwinSse {
    name: String,
    left: Option<BenchmarkBinaryTreeTwinSse>,
    right: Option<BenchmarkBinaryTreeTwinSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn benchmark_binary_tree_input_twin_sse(tree: BenchmarkBinaryTreeTwinSse) {
    black_box(tree);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn benchmark_binary_tree_output_twin_sse(
    depth: i32,
    name: String,
) -> BenchmarkBinaryTreeTwinSse {
    create_tree(depth, &name)
}

fn create_tree(depth: i32, name: &str) -> BenchmarkBinaryTreeTwinSse {
    if depth == 0 {
        BenchmarkBinaryTreeTwinSse {
            name: name.to_owned(),
            left: None,
            right: None,
        }
    } else {
        BenchmarkBinaryTreeTwinSse {
            name: name.to_owned(),
            left: Some(create_tree(depth - 1)),
            right: Some(create_tree(depth - 1)),
        }
    }
}

pub struct BenchmarkBlobTwinSse {
    first: Vec<u8>,
    second: Vec<u8>,
    third: Vec<u8>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn benchmark_blob_input_twin_sse(blob: BenchmarkBlobTwinSse) {
    black_box(blob);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn benchmark_blob_output_twin_sse(size: i32) -> BenchmarkBlobTwinSse {
    let data = vec![0; size as _];
    BenchmarkBlobTwinSse {
        first: data.clone(),
        second: data.clone(),
        third: data,
    }
}
