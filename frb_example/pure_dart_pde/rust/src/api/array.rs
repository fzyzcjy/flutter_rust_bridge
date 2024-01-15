// [T; N] example
pub fn get_array_twin_normal() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct PointTwinNormal {
    pub x: f32,
    pub y: f32,
}

pub fn get_complex_array_twin_normal() -> [PointTwinNormal; 2] {
    [
        PointTwinNormal { x: 1.0, y: 1.0 },
        PointTwinNormal { x: 2.0, y: 2.0 },
    ]
}

pub struct MessageIdTwinNormal(pub [u8; 32]);

pub fn new_msgid_twin_normal(id: [u8; 32]) -> MessageIdTwinNormal {
    MessageIdTwinNormal(id)
}

pub fn use_msgid_twin_normal(id: MessageIdTwinNormal) -> [u8; 32] {
    id.0
}
pub struct BlobTwinNormal(pub [u8; 1600]);
pub fn boxed_blob_twin_normal(blob: Box<[u8; 1600]>) -> BlobTwinNormal {
    BlobTwinNormal(*blob)
}

pub fn use_boxed_blob_twin_normal(blob: Box<BlobTwinNormal>) -> [u8; 1600] {
    blob.0
}

pub struct FeedIdTwinNormal(pub [u8; 8]);

pub fn return_boxed_feed_id_twin_normal(id: [u8; 8]) -> Box<FeedIdTwinNormal> {
    Box::new(FeedIdTwinNormal(id))
}

pub fn return_boxed_raw_feed_id_twin_normal(id: FeedIdTwinNormal) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestIdTwinNormal(pub [i32; 2]);

pub fn func_test_id_twin_normal(id: TestIdTwinNormal) -> TestIdTwinNormal {
    id
}

pub fn last_number_twin_normal(array: [f64; 16]) -> f64 {
    array[15]
}

pub fn nested_id_twin_normal(id: [TestIdTwinNormal; 4]) -> [TestIdTwinNormal; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}
