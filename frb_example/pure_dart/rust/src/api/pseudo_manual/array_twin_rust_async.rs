// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `array.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// [T; N] example
pub async fn get_array_twin_rust_async() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct PointTwinRustAsync {
    pub x: f32,
    pub y: f32,
}

pub async fn get_complex_array_twin_rust_async() -> [PointTwinRustAsync; 2] {
    [
        PointTwinRustAsync { x: 1.0, y: 1.0 },
        PointTwinRustAsync { x: 2.0, y: 2.0 },
    ]
}

pub struct MessageIdTwinRustAsync(pub [u8; 32]);

pub async fn new_msgid_twin_rust_async(id: [u8; 32]) -> MessageIdTwinRustAsync {
    MessageIdTwinRustAsync(id)
}

pub async fn use_msgid_twin_rust_async(id: MessageIdTwinRustAsync) -> [u8; 32] {
    id.0
}
pub struct BlobTwinRustAsync(pub [u8; 1600]);
pub async fn boxed_blob_twin_rust_async(blob: Box<[u8; 1600]>) -> BlobTwinRustAsync {
    BlobTwinRustAsync(*blob)
}

pub async fn use_boxed_blob_twin_rust_async(blob: Box<BlobTwinRustAsync>) -> [u8; 1600] {
    blob.0
}

pub struct FeedIdTwinRustAsync(pub [u8; 8]);

pub async fn return_boxed_feed_id_twin_rust_async(id: [u8; 8]) -> Box<FeedIdTwinRustAsync> {
    Box::new(FeedIdTwinRustAsync(id))
}

pub async fn return_boxed_raw_feed_id_twin_rust_async(id: FeedIdTwinRustAsync) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestIdTwinRustAsync(pub [i32; 2]);

pub async fn func_test_id_twin_rust_async(id: TestIdTwinRustAsync) -> TestIdTwinRustAsync {
    id
}

pub async fn last_number_twin_rust_async(array: [f64; 16]) -> f64 {
    array[15]
}

pub async fn nested_id_twin_rust_async(id: [TestIdTwinRustAsync; 4]) -> [TestIdTwinRustAsync; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}
