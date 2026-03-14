// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `array.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// [T; N] example
#[flutter_rust_bridge::frb(serialize)]
pub async fn get_array_twin_rust_async_sse() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct PointTwinRustAsyncSse {
    pub x: f32,
    pub y: f32,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn get_complex_array_twin_rust_async_sse() -> [PointTwinRustAsyncSse; 2] {
    [
        PointTwinRustAsyncSse { x: 1.0, y: 1.0 },
        PointTwinRustAsyncSse { x: 2.0, y: 2.0 },
    ]
}

pub struct MessageIdTwinRustAsyncSse(pub [u8; 32]);

#[flutter_rust_bridge::frb(serialize)]
pub async fn new_msgid_twin_rust_async_sse(id: [u8; 32]) -> MessageIdTwinRustAsyncSse {
    MessageIdTwinRustAsyncSse(id)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn use_msgid_twin_rust_async_sse(id: MessageIdTwinRustAsyncSse) -> [u8; 32] {
    id.0
}
pub struct BlobTwinRustAsyncSse(pub [u8; 1600]);
#[flutter_rust_bridge::frb(serialize)]
pub async fn boxed_blob_twin_rust_async_sse(blob: Box<[u8; 1600]>) -> BlobTwinRustAsyncSse {
    BlobTwinRustAsyncSse(*blob)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn use_boxed_blob_twin_rust_async_sse(blob: Box<BlobTwinRustAsyncSse>) -> [u8; 1600] {
    blob.0
}

pub struct FeedIdTwinRustAsyncSse(pub [u8; 8]);

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_boxed_feed_id_twin_rust_async_sse(id: [u8; 8]) -> Box<FeedIdTwinRustAsyncSse> {
    Box::new(FeedIdTwinRustAsyncSse(id))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_boxed_raw_feed_id_twin_rust_async_sse(
    id: FeedIdTwinRustAsyncSse,
) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestIdTwinRustAsyncSse(pub [i32; 2]);

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_test_id_twin_rust_async_sse(
    id: TestIdTwinRustAsyncSse,
) -> TestIdTwinRustAsyncSse {
    id
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn last_number_twin_rust_async_sse(array: [f64; 16]) -> f64 {
    array[15]
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn nested_id_twin_rust_async_sse(
    id: [TestIdTwinRustAsyncSse; 4],
) -> [TestIdTwinRustAsyncSse; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}
