// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `array.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// [T; N] example
#[flutter_rust_bridge::frb(serialize)]
pub fn get_array_twin_sse() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct PointTwinSse {
    pub x: f32,
    pub y: f32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn get_complex_array_twin_sse() -> [PointTwinSse; 2] {
    [
        PointTwinSse { x: 1.0, y: 1.0 },
        PointTwinSse { x: 2.0, y: 2.0 },
    ]
}

pub struct MessageIdTwinSse(pub [u8; 32]);

#[flutter_rust_bridge::frb(serialize)]
pub fn new_msgid_twin_sse(id: [u8; 32]) -> MessageIdTwinSse {
    MessageIdTwinSse(id)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn use_msgid_twin_sse(id: MessageIdTwinSse) -> [u8; 32] {
    id.0
}
pub struct BlobTwinSse(pub [u8; 1600]);
#[flutter_rust_bridge::frb(serialize)]
pub fn boxed_blob_twin_sse(blob: Box<[u8; 1600]>) -> BlobTwinSse {
    BlobTwinSse(*blob)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn use_boxed_blob_twin_sse(blob: Box<BlobTwinSse>) -> [u8; 1600] {
    blob.0
}

pub struct FeedIdTwinSse(pub [u8; 8]);

#[flutter_rust_bridge::frb(serialize)]
pub fn return_boxed_feed_id_twin_sse(id: [u8; 8]) -> Box<FeedIdTwinSse> {
    Box::new(FeedIdTwinSse(id))
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_boxed_raw_feed_id_twin_sse(id: FeedIdTwinSse) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestIdTwinSse(pub [i32; 2]);

#[flutter_rust_bridge::frb(serialize)]
pub fn func_test_id_twin_sse(id: TestIdTwinSse) -> TestIdTwinSse {
    id
}

#[flutter_rust_bridge::frb(serialize)]
pub fn last_number_twin_sse(array: [f64; 16]) -> f64 {
    array[15]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn nested_id_twin_sse(id: [TestIdTwinSse; 4]) -> [TestIdTwinSse; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}
