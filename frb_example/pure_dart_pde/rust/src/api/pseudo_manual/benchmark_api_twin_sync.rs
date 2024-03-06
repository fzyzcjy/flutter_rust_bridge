// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `benchmark_api.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(unused_variables)]

use crate::auxiliary::protobuf_for_benchmark::{BinaryTreeProtobuf, BlobProtobuf};
use lazy_static::lazy_static;
use protobuf::Message;
use std::collections::HashMap;
use std::hint::black_box;

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_void_twin_sync() {}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_input_bytes_twin_sync(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_output_bytes_twin_sync(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}

// The `serde` is only used for comparison test
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkBinaryTreeTwinSync {
    pub name: String,
    pub left: Option<Box<BenchmarkBinaryTreeTwinSync>>,
    pub right: Option<Box<BenchmarkBinaryTreeTwinSync>>,
}

fn create_binary_tree(depth: i32, name: &str) -> BenchmarkBinaryTreeTwinSync {
    if depth == 0 {
        BenchmarkBinaryTreeTwinSync {
            name: name.to_owned(),
            left: None,
            right: None,
        }
    } else {
        BenchmarkBinaryTreeTwinSync {
            name: name.to_owned(),
            left: Some(Box::new(create_binary_tree(depth - 1, name))),
            right: Some(Box::new(create_binary_tree(depth - 1, name))),
        }
    }
}

fn create_binary_tree_protobuf(depth: i32, name: &str) -> BinaryTreeProtobuf {
    if depth == 0 {
        BinaryTreeProtobuf {
            name: name.to_owned(),
            left: protobuf::SingularPtrField::none(),
            right: protobuf::SingularPtrField::none(),
            ..Default::default()
        }
    } else {
        BinaryTreeProtobuf {
            name: name.to_owned(),
            left: protobuf::SingularPtrField::some(create_binary_tree_protobuf(depth - 1, name)),
            right: protobuf::SingularPtrField::some(create_binary_tree_protobuf(depth - 1, name)),
            ..Default::default()
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
    static ref BINARY_TREES: HashMap<i32, BenchmarkBinaryTreeTwinSync> =
        create_binary_tree_map(create_binary_tree);
    static ref BINARY_TREES_PROTOBUF: HashMap<i32, BinaryTreeProtobuf> =
        create_binary_tree_map(create_binary_tree_protobuf);
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_input_twin_sync(tree: BenchmarkBinaryTreeTwinSync) {
    black_box(tree);
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_output_twin_sync(depth: i32) -> BenchmarkBinaryTreeTwinSync {
    BINARY_TREES.get(&depth).unwrap().to_owned()
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_input_protobuf_twin_sync(raw: Vec<u8>) {
    let data = BinaryTreeProtobuf::parse_from_bytes(&raw).unwrap();
    black_box(data);
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_output_protobuf_twin_sync(depth: i32) -> Vec<u8> {
    BINARY_TREES_PROTOBUF
        .get(&depth)
        .unwrap()
        .write_to_bytes()
        .unwrap()
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_input_json_twin_sync(raw: String) {
    let obj: BenchmarkBinaryTreeTwinSync = serde_json::from_str(&raw).unwrap();
    black_box(obj);
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_binary_tree_output_json_twin_sync(depth: i32) -> String {
    serde_json::to_string(BINARY_TREES.get(&depth).unwrap()).unwrap()
}

// The `serde` is only used for comparison test
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkBlobTwinSync {
    pub first: Vec<u8>,
    pub second: Vec<u8>,
    pub third: Vec<u8>,
}

fn create_blob(size: i32) -> BenchmarkBlobTwinSync {
    let data = vec![0; size as _];
    BenchmarkBlobTwinSync {
        first: data.clone(),
        second: data.clone(),
        third: data,
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_input_twin_sync(blob: BenchmarkBlobTwinSync) {
    black_box(blob);
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_output_twin_sync(size: i32) -> BenchmarkBlobTwinSync {
    create_blob(size)
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_input_protobuf_twin_sync(raw: Vec<u8>) {
    let data = BlobProtobuf::parse_from_bytes(&raw).unwrap();
    black_box(data);
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_output_protobuf_twin_sync(size: i32) -> Vec<u8> {
    let data = vec![0; size as _];
    let output = BlobProtobuf {
        first: data.clone(),
        second: data.clone(),
        third: data,
        ..Default::default()
    };
    output.write_to_bytes().unwrap()
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_input_json_twin_sync(raw: String) {
    let obj: BenchmarkBlobTwinSync = serde_json::from_str(&raw).unwrap();
    black_box(obj);
}

#[flutter_rust_bridge::frb(sync)]
pub fn benchmark_blob_output_json_twin_sync(size: i32) -> String {
    serde_json::to_string(&create_blob(size)).unwrap()
}
