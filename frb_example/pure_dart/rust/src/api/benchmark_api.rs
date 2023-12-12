// Note: This file contains tons of duplicated code! Because it is only for benchmarking
// and many are ad-hoc code.

#![allow(unused_variables)]

use crate::auxiliary::protobuf_for_benchmark::{BinaryTreeProtobuf, BlobProtobuf};
use lazy_static::lazy_static;
use protobuf::Message;
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl BinaryTreeProtobuf {
    fn new(depth: i32, name: &str) -> Self {
        if depth == 0 {
            Self {
                name: name.to_owned(),
                left: protobuf::SingularPtrField::none(),
                right: protobuf::SingularPtrField::none(),
                ..Default::default()
            }
        } else {
            Self {
                name: name.to_owned(),
                left: protobuf::SingularPtrField::some(Self::new(depth - 1, name)),
                right: protobuf::SingularPtrField::some(Self::new(depth - 1, name)),
                ..Default::default()
            }
        }
    }
}

fn create_binary_tree_map<T>(creator: impl Fn(i32, &str) -> T) -> HashMap<i32, T> {
    let mut m = HashMap::new();
    for depth in vec![0, 5, 10].into_iter() {
        m.insert(depth, creator(depth, "HelloWorld"));
    }
    m
}

lazy_static! {
    static ref BINARY_TREES: HashMap<i32, BenchmarkBinaryTreeTwinNormal> =
        create_binary_tree_map(BenchmarkBinaryTreeTwinNormal::new);
    static ref BINARY_TREES_PROTOBUF: HashMap<i32, BinaryTreeProtobuf> =
        create_binary_tree_map(BinaryTreeProtobuf::new);
}

pub fn benchmark_binary_tree_input_twin_normal(tree: BenchmarkBinaryTreeTwinNormal) {
    black_box(tree);
}

pub fn benchmark_binary_tree_output_twin_normal(depth: i32) -> BenchmarkBinaryTreeTwinNormal {
    BINARY_TREES.get(&depth).unwrap().to_owned()
}

pub fn benchmark_binary_tree_input_protobuf_twin_normal(raw: Vec<u8>) {
    let data = BinaryTreeProtobuf::parse_from_bytes(&raw).unwrap();
    black_box(data);
}

pub fn benchmark_binary_tree_output_protobuf_twin_normal(depth: i32) -> Vec<u8> {
    BINARY_TREES_PROTOBUF
        .get(&depth)
        .unwrap()
        .write_to_bytes()
        .unwrap()
}

pub fn benchmark_binary_tree_input_json_twin_normal(raw: String) {
    let obj: BenchmarkBlobTwinNormal = serde_json::from_str(&raw).unwrap();
    black_box(obj);
}

pub fn benchmark_binary_tree_output_json_twin_normal(depth: i32) -> String {
    serde_json::to_string(BINARY_TREES.get(&depth).unwrap()).unwrap()
}

// The `serde` is only used for comparison test
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    let data = BlobProtobuf::parse_from_bytes(&raw).unwrap();
    black_box(data);
}

pub fn benchmark_blob_output_protobuf_twin_normal(size: i32) -> Vec<u8> {
    let data = vec![0; size as _];
    let output = BlobProtobuf {
        first: data.clone(),
        second: data.clone(),
        third: data,
        ..Default::default()
    };
    output.write_to_bytes().unwrap()
}

pub fn benchmark_blob_input_json_twin_normal(raw: String) {
    let obj: BenchmarkBlobTwinNormal = serde_json::from_str(&raw).unwrap();
    black_box(obj);
}

pub fn benchmark_blob_output_json_twin_normal(size: i32) -> String {
    serde_json::to_string(&BenchmarkBlobTwinNormal::new(size)).unwrap()
}
