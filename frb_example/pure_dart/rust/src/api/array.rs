// [T; N] example
pub fn get_array_twin_normal() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn get_complex_array_twin_normal() -> [Point; 2] {
    [Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }]
}

pub struct MessageId(pub [u8; 32]);

pub fn new_msgid_twin_normal(id: [u8; 32]) -> MessageId {
    MessageId(id)
}

pub fn use_msgid_twin_normal(id: MessageId) -> [u8; 32] {
    id.0
}
pub struct Blob(pub [u8; 1600]);
pub fn boxed_blob_twin_normal(blob: Box<[u8; 1600]>) -> Blob {
    Blob(*blob)
}

pub fn use_boxed_blob_twin_normal(blob: Box<Blob>) -> [u8; 1600] {
    blob.0
}

pub struct FeedId(pub [u8; 8]);

pub fn return_boxed_feed_id_twin_normal(id: [u8; 8]) -> Box<FeedId> {
    Box::new(FeedId(id))
}

pub fn return_boxed_raw_feed_id_twin_normal(id: FeedId) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestId(pub [i32; 2]);

pub fn func_test_id_twin_normal(id: TestId) -> TestId {
    id
}

pub fn last_number_twin_normal(array: [f64; 16]) -> f64 {
    array[15]
}

pub fn nested_id_twin_normal(id: [TestId; 4]) -> [TestId; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}
