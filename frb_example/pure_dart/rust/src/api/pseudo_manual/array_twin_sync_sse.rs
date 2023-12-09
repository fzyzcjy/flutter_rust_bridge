// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `array.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// [T; N] example
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn get_array_twin_sync_sse() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct PointTwinSyncSse {
    pub x: f32,
    pub y: f32,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn get_complex_array_twin_sync_sse() -> [PointTwinSyncSse; 2] {
    [
        PointTwinSyncSse { x: 1.0, y: 1.0 },
        PointTwinSyncSse { x: 2.0, y: 2.0 },
    ]
}

pub struct MessageIdTwinSyncSse(pub [u8; 32]);

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn new_msgid_twin_sync_sse(id: [u8; 32]) -> MessageIdTwinSyncSse {
    MessageIdTwinSyncSse(id)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn use_msgid_twin_sync_sse(id: MessageIdTwinSyncSse) -> [u8; 32] {
    id.0
}
pub struct BlobTwinSyncSse(pub [u8; 1600]);
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn boxed_blob_twin_sync_sse(blob: Box<[u8; 1600]>) -> BlobTwinSyncSse {
    BlobTwinSyncSse(*blob)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn use_boxed_blob_twin_sync_sse(blob: Box<BlobTwinSyncSse>) -> [u8; 1600] {
    blob.0
}

pub struct FeedIdTwinSyncSse(pub [u8; 8]);

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_boxed_feed_id_twin_sync_sse(id: [u8; 8]) -> Box<FeedIdTwinSyncSse> {
    Box::new(FeedIdTwinSyncSse(id))
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_boxed_raw_feed_id_twin_sync_sse(id: FeedIdTwinSyncSse) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestIdTwinSyncSse(pub [i32; 2]);

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_test_id_twin_sync_sse(id: TestIdTwinSyncSse) -> TestIdTwinSyncSse {
    id
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn last_number_twin_sync_sse(array: [f64; 16]) -> f64 {
    array[15]
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn nested_id_twin_sync_sse(id: [TestIdTwinSyncSse; 4]) -> [TestIdTwinSyncSse; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}
