// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `array.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// [T; N] example
#[flutter_rust_bridge::frb(sync)]
pub fn get_array_twin_sync() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct PointTwinSync {
    pub x: f32,
    pub y: f32,
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_complex_array_twin_sync() -> [PointTwinSync; 2] {
    [
        PointTwinSync { x: 1.0, y: 1.0 },
        PointTwinSync { x: 2.0, y: 2.0 },
    ]
}

pub struct MessageIdTwinSync(pub [u8; 32]);

#[flutter_rust_bridge::frb(sync)]
pub fn new_msgid_twin_sync(id: [u8; 32]) -> MessageIdTwinSync {
    MessageIdTwinSync(id)
}

#[flutter_rust_bridge::frb(sync)]
pub fn use_msgid_twin_sync(id: MessageIdTwinSync) -> [u8; 32] {
    id.0
}
pub struct BlobTwinSync(pub [u8; 1600]);
#[flutter_rust_bridge::frb(sync)]
pub fn boxed_blob_twin_sync(blob: Box<[u8; 1600]>) -> BlobTwinSync {
    BlobTwinSync(*blob)
}

#[flutter_rust_bridge::frb(sync)]
pub fn use_boxed_blob_twin_sync(blob: Box<BlobTwinSync>) -> [u8; 1600] {
    blob.0
}

pub struct FeedIdTwinSync(pub [u8; 8]);

#[flutter_rust_bridge::frb(sync)]
pub fn return_boxed_feed_id_twin_sync(id: [u8; 8]) -> Box<FeedIdTwinSync> {
    Box::new(FeedIdTwinSync(id))
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_boxed_raw_feed_id_twin_sync(id: FeedIdTwinSync) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestIdTwinSync(pub [i32; 2]);

#[flutter_rust_bridge::frb(sync)]
pub fn func_test_id_twin_sync(id: TestIdTwinSync) -> TestIdTwinSync {
    id
}

#[flutter_rust_bridge::frb(sync)]
pub fn last_number_twin_sync(array: [f64; 16]) -> f64 {
    array[15]
}

#[flutter_rust_bridge::frb(sync)]
pub fn nested_id_twin_sync(id: [TestIdTwinSync; 4]) -> [TestIdTwinSync; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}
