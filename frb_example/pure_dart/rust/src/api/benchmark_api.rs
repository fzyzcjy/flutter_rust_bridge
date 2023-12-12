#![allow(unused_variables)]

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hint::black_box;

pub fn benchmark_void_twin_normal() {}

pub fn benchmark_input_bytes_twin_normal(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

pub fn benchmark_output_bytes_twin_normal(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}

// The `serde` is only used for comparison test
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkBinaryTreeTwinNormal {
    pub name: String,
    pub left: Option<Box<BenchmarkBinaryTreeTwinNormal>>,
    pub right: Option<Box<BenchmarkBinaryTreeTwinNormal>>,
}

impl BenchmarkBinaryTreeTwinNormal {
    fn new(depth: i32, name: &str) -> BenchmarkBinaryTreeTwinNormal {
        if depth == 0 {
            BenchmarkBinaryTreeTwinNormal {
                name: name.to_owned(),
                left: None,
                right: None,
            }
        } else {
            BenchmarkBinaryTreeTwinNormal {
                name: name.to_owned(),
                left: Some(Box::new(Self::new(depth - 1, name))),
                right: Some(Box::new(Self::new(depth - 1, name))),
            }
        }
    }
}

lazy_static! {
    static ref BINARY_TREES: HashMap<i32, BenchmarkBinaryTreeTwinNormal> = {
        let mut m = HashMap::new();
        for depth in vec![0, 5, 10].into_iter() {
            m.insert(depth, create_tree(depth, "HelloWorld"));
        }
        m
    };
}

pub fn benchmark_binary_tree_input_twin_normal(tree: BenchmarkBinaryTreeTwinNormal) {
    black_box(tree);
}

pub fn benchmark_binary_tree_output_twin_normal(depth: i32) -> BenchmarkBinaryTreeTwinNormal {
    BINARY_TREES.get(&depth).unwrap().to_owned()
}

pub fn benchmark_binary_tree_input_protobuf_twin_normal(raw: Vec<u8>) {
    todo!()
}

pub fn benchmark_binary_tree_output_protobuf_twin_normal(depth: i32) -> Vec<u8> {
    todo!()
}

pub fn benchmark_binary_tree_input_json_twin_normal(raw: String) {
    let obj: BenchmarkBlobTwinNormal = serde_json::from_str(&raw).unwrap();
    black_box(obj);
}

pub fn benchmark_binary_tree_output_json_twin_normal(depth: i32) -> String {
    serde_json::to_string(BINARY_TREES.get(&depth).unwrap()).unwrap()
}

pub struct BenchmarkBlobTwinNormal {
    pub first: Vec<u8>,
    pub second: Vec<u8>,
    pub third: Vec<u8>,
}

impl BenchmarkBlobTwinNormal {
    pub fn new(size: i32) -> Self {
        let data = vec![0; size as _];
        Self {
            first: data.clone(),
            second: data.clone(),
            third: data,
        }
    }
}

pub fn benchmark_blob_input_twin_normal(blob: BenchmarkBlobTwinNormal) {
    black_box(blob);
}

pub fn benchmark_blob_output_twin_normal(size: i32) -> BenchmarkBlobTwinNormal {
    BenchmarkBlobTwinNormal::new(size)
}

pub fn benchmark_blob_input_protobuf_twin_normal(raw: Vec<u8>) {
    todo!()
}

pub fn benchmark_blob_output_protobuf_twin_normal(size: i32) -> Vec<u8> {
    todo!()
}

pub fn benchmark_blob_input_json_twin_normal(raw: String) {
    todo!()
}

pub fn benchmark_blob_output_json_twin_normal(size: i32) -> String {
    todo!()
}
