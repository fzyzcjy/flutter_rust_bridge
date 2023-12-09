#[cfg(feature = "chrono")]
#[inline]
pub fn decode_timestamp(ts: i64) -> Timestamp {
    #[cfg(wasm)]
    const PRECISION: i64 = 1_000;
    #[cfg(not(wasm))]
    const PRECISION: i64 = 1_000_000;

    let s = ts / PRECISION;
    let ns = (ts.rem_euclid(PRECISION) * (1_000_000_000 / PRECISION)) as u32;
    Timestamp { s, ns }
}

/// a timestamp with microseconds precision on native and milliseconds precision on web
#[cfg(feature = "chrono")]
pub struct Timestamp {
    /// seconds
    pub s: i64,
    /// nanoseconds
    pub ns: u32,
}

/// see [uuid::Bytes](https://docs.rs/uuid/1.1.2/uuid/type.Bytes.html)
#[cfg(feature = "uuid")]
pub(crate) const UUID_SIZE_IN_BYTES: usize = 16;

#[cfg(feature = "uuid")]
#[inline]
fn decode_uuid_ref(id: &[u8]) -> uuid::Uuid {
    uuid::Uuid::from_bytes(
        *<&[u8] as std::convert::TryInto<&[u8; UUID_SIZE_IN_BYTES]>>::try_into(id)
            .expect("invalid uuid slice"),
    )
}

#[cfg(feature = "uuid")]
#[inline]
pub fn decode_uuid(id: Vec<u8>) -> uuid::Uuid {
    decode_uuid_ref(id.as_slice())
}

// #[cfg(feature = "uuid")]
// #[inline]
// pub fn cst_decode_uuids(ids: Vec<u8>) -> Vec<uuid::Uuid> {
//     ids.as_slice()
//         .chunks(UUID_SIZE_IN_BYTES)
//         .map(cst_decode_uuid_ref)
//         .collect::<Vec<uuid::Uuid>>()
// }

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    #[test]
    fn test_decode_timestamp() {
        #[cfg(not(wasm))]
        {
            // input in microseconds
            let input: i64 = 3_496_567_123;
            let super::Timestamp { s, ns } = super::decode_timestamp(input);
            assert_eq!(s, 3_496);
            assert_eq!(ns, 567_123_000);
        }

        #[cfg(wasm)]
        {
            // input in milliseconds
            let input: i64 = 3_496_567;
            let super::Timestamp { s, ns } = super::decode_timestamp(input);
            assert_eq!(s, 3_496);
            assert_eq!(ns, 567_000_000);
        }
    }
}
