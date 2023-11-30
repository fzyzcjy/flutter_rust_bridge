#![allow(unused_variables)]

pub fn benchmark_void_twin_normal() {}

pub fn benchmark_input_bytes_twin_normal(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

pub fn benchmark_output_bytes_twin_normal(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}
