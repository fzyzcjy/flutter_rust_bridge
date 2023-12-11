// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `benchmark_api.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(unused_variables)]

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hint::black_box;

pub async fn benchmark_void_twin_rust_async() {}

pub async fn benchmark_input_bytes_twin_rust_async(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

pub async fn benchmark_output_bytes_twin_rust_async(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}

#[derive(Clone)]
pub struct BenchmarkBinaryTreeTwinRustAsync {
    pub name: String,
    pub left: Option<Box<BenchmarkBinaryTreeTwinRustAsync>>,
    pub right: Option<Box<BenchmarkBinaryTreeTwinRustAsync>>,
}

pub async fn benchmark_binary_tree_input_twin_rust_async(tree: BenchmarkBinaryTreeTwinRustAsync) {
    black_box(tree);
}

lazy_static! {
    static ref BINARY_TREES: HashMap<i32, BenchmarkBinaryTreeTwinRustAsync> = {
        let mut m = HashMap::new();
        for depth in vec![0, 5, 10].into_iter() {
            m.insert(depth, create_tree(depth, "HelloWorld"));
        }
        m
    };
}

pub async fn benchmark_binary_tree_output_twin_rust_async(
    depth: i32,
) -> BenchmarkBinaryTreeTwinRustAsync {
    BINARY_TREES.get(&depth).unwrap().to_owned()
}

fn create_tree(depth: i32, name: &str) -> BenchmarkBinaryTreeTwinRustAsync {
    if depth == 0 {
        BenchmarkBinaryTreeTwinRustAsync {
            name: name.to_owned(),
            left: None,
            right: None,
        }
    } else {
        BenchmarkBinaryTreeTwinRustAsync {
            name: name.to_owned(),
            left: Some(Box::new(create_tree(depth - 1, name))),
            right: Some(Box::new(create_tree(depth - 1, name))),
        }
    }
}

pub struct BenchmarkBlobTwinRustAsync {
    first: Vec<u8>,
    second: Vec<u8>,
    third: Vec<u8>,
}

pub async fn benchmark_blob_input_twin_rust_async(blob: BenchmarkBlobTwinRustAsync) {
    black_box(blob);
}

pub async fn benchmark_blob_output_twin_rust_async(size: i32) -> BenchmarkBlobTwinRustAsync {
    let data = vec![0; size as _];
    BenchmarkBlobTwinRustAsync {
        first: data.clone(),
        second: data.clone(),
        third: data,
    }
}
