// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `benchmark_api.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(unused_variables)]

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hint::black_box;

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_void_twin_rust_async_sse() {}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_input_bytes_twin_rust_async_sse(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_output_bytes_twin_rust_async_sse(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}

#[derive(Clone)]
pub struct BenchmarkBinaryTreeTwinRustAsyncSse {
    pub name: String,
    pub left: Option<Box<BenchmarkBinaryTreeTwinRustAsyncSse>>,
    pub right: Option<Box<BenchmarkBinaryTreeTwinRustAsyncSse>>,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_binary_tree_input_twin_rust_async_sse(
    tree: BenchmarkBinaryTreeTwinRustAsyncSse,
) {
    black_box(tree);
}

lazy_static! {
    static ref BINARY_TREES: HashMap<i32, BenchmarkBinaryTreeTwinRustAsyncSse> = {
        let mut m = HashMap::new();
        for depth in vec![0, 5, 10].into_iter() {
            m.insert(depth, create_tree(depth, "HelloWorld"));
        }
        m
    };
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_binary_tree_output_twin_rust_async_sse(
    depth: i32,
) -> BenchmarkBinaryTreeTwinRustAsyncSse {
    BINARY_TREES.get(&depth).unwrap().to_owned()
}

fn create_tree(depth: i32, name: &str) -> BenchmarkBinaryTreeTwinRustAsyncSse {
    if depth == 0 {
        BenchmarkBinaryTreeTwinRustAsyncSse {
            name: name.to_owned(),
            left: None,
            right: None,
        }
    } else {
        BenchmarkBinaryTreeTwinRustAsyncSse {
            name: name.to_owned(),
            left: Some(Box::new(create_tree(depth - 1, name))),
            right: Some(Box::new(create_tree(depth - 1, name))),
        }
    }
}

pub struct BenchmarkBlobTwinRustAsyncSse {
    pub first: Vec<u8>,
    pub second: Vec<u8>,
    pub third: Vec<u8>,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_blob_input_twin_rust_async_sse(blob: BenchmarkBlobTwinRustAsyncSse) {
    black_box(blob);
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_blob_output_twin_rust_async_sse(size: i32) -> BenchmarkBlobTwinRustAsyncSse {
    let data = vec![0; size as _];
    BenchmarkBlobTwinRustAsyncSse {
        first: data.clone(),
        second: data.clone(),
        third: data,
    }
}
