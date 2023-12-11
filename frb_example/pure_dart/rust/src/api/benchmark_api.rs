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

#[derive(Clone)]
pub struct BenchmarkBinaryTreeTwinNormal {
    pub name: String,
    pub left: Option<Box<BenchmarkBinaryTreeTwinNormal>>,
    pub right: Option<Box<BenchmarkBinaryTreeTwinNormal>>,
}

pub fn benchmark_binary_tree_input_twin_normal(tree: BenchmarkBinaryTreeTwinNormal) {
    black_box(tree);
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

pub fn benchmark_binary_tree_output_twin_normal(depth: i32) -> BenchmarkBinaryTreeTwinNormal {
    BINARY_TREES.get(&depth).unwrap().to_owned()
}

fn create_tree(depth: i32, name: &str) -> BenchmarkBinaryTreeTwinNormal {
    if depth == 0 {
        BenchmarkBinaryTreeTwinNormal {
            name: name.to_owned(),
            left: None,
            right: None,
        }
    } else {
        BenchmarkBinaryTreeTwinNormal {
            name: name.to_owned(),
            left: Some(Box::new(create_tree(depth - 1, name))),
            right: Some(Box::new(create_tree(depth - 1, name))),
        }
    }
}

pub struct BenchmarkBlobTwinNormal {
    first: Vec<u8>,
    second: Vec<u8>,
    third: Vec<u8>,
}

pub fn benchmark_blob_input_twin_normal(blob: BenchmarkBlobTwinNormal) {
    black_box(blob);
}

pub fn benchmark_blob_output_twin_normal(size: i32) -> BenchmarkBlobTwinNormal {
    let data = vec![0; size as _];
    BenchmarkBlobTwinNormal {
        first: data.clone(),
        second: data.clone(),
        third: data,
    }
}
